use flexion::Path;

/// Words ending -re (3 syllables).
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['g', 'i', s @ ..] => {
					super::push(out, 2423);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2424);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3487);
							match s {
								[] => true,
								['-', 'u', 'a', 'e', 't', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4369);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3488);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2425);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2427);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1407);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2429);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2430);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3490);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4372);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['è', s @ ..] => {
					super::push(out, 2431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1408);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3492);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4373);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4931);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 4374);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3493);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3494);
							match s {
								[] => true,
								['l', 'y', 'c', '-', 'e', 'r', 't', 'î', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2434);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							super::push(out, 3495);
							match s {
								[] => true,
								['-', 'n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2435);
					s.is_empty()
				}
				['è', s @ ..] => {
					super::push(out, 2436);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2437);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2438);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3497);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4378);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4933);
											match s {
												[] => true,
												['v', '-', 'e', 's', 's', 'i', 'p', '\u{1}', s @ ..] => {
													super::push(out, 5279);
													s.is_empty()
												}
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
							super::push(out, 3498);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4379);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4934);
											s.is_empty()
										}
										['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4935);
											s.is_empty()
										}
										['t', 'a', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 4936);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1411);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 2439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n1444(s, out),
		['n', s @ ..] => {
			super::push(out, 1413);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2445);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3520);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4406);
									s.is_empty()
								}
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
			super::push(out, 1414);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2446);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 3521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2447);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							super::push(out, 3522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'p', s @ ..] => {
					super::push(out, 2448);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3523);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3524);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4407);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									super::push(out, 4408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2449);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2450);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2451);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3526);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1415);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2454);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3528);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4409);
									match s {
										[] => true,
										['-', 'r', 'u', 'o', 'm', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4979);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1416);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n2527(s, out),
				['e', s @ ..] => super::t1::n2528(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n1449(s, out),
		['u', s @ ..] => super::t1::n1450(s, out),
		['v', s @ ..] => super::t1::n1451(s, out),
		['y', s @ ..] => super::t1::n1452(s, out),
		['è', s @ ..] => super::t1::n1453(s, out),
		['ü', s @ ..] => super::t1::n1454(s, out),
		_ => false,
	}
}

/// Words ending -se (3 syllables).
pub(crate) fn n575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2501);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				['h', 'p', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				['n', 'i', 'm', s @ ..] => {
					super::push(out, 2506);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2508);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3614);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3615);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3616);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4467);
									match s {
										[] => true,
										['-', 'e', 's', 's', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 5018);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ç', s @ ..] => {
							super::push(out, 3617);
							match s {
								[] => true,
								['n', 'a', 'r', 'f', '-', 'e', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', '-', 'e', 's', 'i', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2510);
					match s {
						[] => true,
						['a', 'v', s @ ..] => {
							super::push(out, 3618);
							match s {
								[] => true,
								['-', 't', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2511);
					match s {
						[] => true,
						['n', 'u', s @ ..] => {
							super::push(out, 3619);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2512);
					match s {
						[] => true,
						['b', '-', 'e', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3621);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3622);
							match s {
								[] => true,
								['c', '-', 'r', 'e', 'i', 'r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', 'p', 's', s @ ..] => {
			super::push(out, 1425);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2514);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3623);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2515);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3624);
							s.is_empty()
						}
						['y', 'm', s @ ..] => {
							super::push(out, 3625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2516);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 2517);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', s @ ..] => {
					super::push(out, 2518);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2519);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3627);
							match s {
								[] => true,
								['r', 'e', 'i', 'r', 'u', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3628);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3629);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 4473);
									s.is_empty()
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
						['a', s @ ..] => {
							super::push(out, 3630);
							s.is_empty()
						}
						['i', 'm', s @ ..] => {
							super::push(out, 3631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 2520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1427);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1428);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2521);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 3632);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4474);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 5019);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1429);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2522);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 3633);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4475);
									match s {
										[] => true,
										['-', 'e', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 5020);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'c', 'r', 'e', 't', s @ ..] => {
							super::push(out, 3634);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 3635);
							match s {
								[] => true,
								['-', 'e', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2523);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3636);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4477);
									match s {
										[] => true,
										['o', 'c', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 5021);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3637);
							match s {
								[] => true,
								['c', 'u', 'd', '-', s @ ..] => {
									super::push(out, 4478);
									match s {
										[] => true,
										['e', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 5022);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3638);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 4479);
									s.is_empty()
								}
								['t', 'ê', 'r', 'p', '-', 'e', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3639);
							match s {
								[] => true,
								['i', 'v', '-', 'e', 's', 's', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n2600(s, out),
				['o', s @ ..] => super::t1::n2601(s, out),
				['u', s @ ..] => super::t1::n2602(s, out),
				['y', s @ ..] => {
					super::push(out, 2527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n1462(s, out),
		['y', s @ ..] => {
			super::push(out, 1431);
			s.is_empty()
		}
		['è', s @ ..] => super::t1::n1464(s, out),
		['ô', s @ ..] => super::t1::n1465(s, out),
		_ => false,
	}
}

/// Words ending -te (3 syllables).
pub(crate) fn n576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3660);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2534);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2535);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2536);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 3664);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2537);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2538);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2539);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2540);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2541);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3666);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 3667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2542);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2543);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3668);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3669);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4502);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 5034);
											match s {
												[] => true,
												['é', s @ ..] => {
													super::push(out, 5317);
													match s {
														[] => true,
														['d', '-', s @ ..] => super::t1::n5613(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5035);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 3670);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 4504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3671);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3672);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3673);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3674);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4506);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 5036);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 5318);
													match s {
														[] => true,
														['g', s @ ..] => super::t1::n5614(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2544);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3675);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2545);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3677);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4507);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5037);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5319);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 2546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2547);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3678);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4508);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5038);
											s.is_empty()
										}
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
					super::push(out, 2548);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 3679);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2549);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n1468(s, out),
		['l', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2561);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2562);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 3717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['a', s @ ..] => super::t1::n2645(s, out),
				['e', s @ ..] => {
					super::push(out, 2564);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3726);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'n', s @ ..] => {
							super::push(out, 3727);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 3728);
							match s {
								[] => true,
								['r', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 4537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n2647(s, out),
				['o', s @ ..] => super::t1::n2648(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n1471(s, out),
		['p', s @ ..] => super::t1::n1472(s, out),
		['r', s @ ..] => super::t1::n1473(s, out),
		['s', s @ ..] => super::t1::n1474(s, out),
		['t', s @ ..] => super::t1::n1475(s, out),
		['u', s @ ..] => super::t1::n1476(s, out),
		['x', s @ ..] => super::t1::n1477(s, out),
		['y', s @ ..] => super::t1::n1478(s, out),
		['è', s @ ..] => super::t1::n1479(s, out),
		['ê', s @ ..] => super::t1::n1480(s, out),
		['ï', s @ ..] => super::t1::n1481(s, out),
		['ô', s @ ..] => super::t1::n1482(s, out),
		_ => false,
	}
}

/// Words ending -ue (3 syllables).
pub(crate) fn n577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1450);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2623);
					match s {
						[] => true,
						['q', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4603);
									match s {
										[] => true,
										['e', 'g', 'u', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 5083);
											s.is_empty()
										}
										['à', '-', 'e', 't', 'ê', 't', '\u{1}', s @ ..] => {
											super::push(out, 5084);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 4604);
									s.is_empty()
								}
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
			super::push(out, 1452);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2624);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 3800);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4605);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 5085);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3801);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2625);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3802);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2626);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', 'o', s @ ..] => {
									super::push(out, 4607);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4608);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5086);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4609);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4610);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 4611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 3804);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4612);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 5087);
											s.is_empty()
										}
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
					super::push(out, 2627);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3805);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5088);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 5089);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 5090);
											match s {
												[] => true,
												['y', s @ ..] => {
													super::push(out, 5338);
													match s {
														[] => true,
														['s', s @ ..] => super::t1::n5621(s, out),
														_ => false,
													}
												}
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
							super::push(out, 3806);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4613);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4614);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['i', 't', s @ ..] => {
													super::push(out, 5339);
													s.is_empty()
												}
												['o', 'm', s @ ..] => {
													super::push(out, 5340);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'p', s @ ..] => {
											super::push(out, 5091);
											s.is_empty()
										}
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
					super::push(out, 2628);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1454);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1455);
			match s {
				[] => true,
				['b', '-', 'e', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => super::t1::n1489(s, out),
		['r', s @ ..] => {
			super::push(out, 1457);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['-', 'e', 'u', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2643);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ve (3 syllables).
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 565);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2644);
					match s {
						[] => true,
						['c', 'i', s @ ..] => {
							super::push(out, 3844);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3845);
							match s {
								[] => true,
								['u', 'o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4668);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3846);
							match s {
								[] => true,
								['r', 'u', s @ ..] => {
									super::push(out, 4669);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1460);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2645);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2646);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2647);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1461);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2649);
					match s {
						[] => true,
						['v', 'i', s @ ..] => {
							super::push(out, 3848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1462);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2651);
					match s {
						[] => true,
						['l', 'f', '-', 'n', 'a', 'm', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['è', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xe (3 syllables).
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 566);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1465);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2653);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2654);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1467);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ze (3 syllables).
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 567);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1469);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1470);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1471);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2658);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 3850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1472);
			s.is_empty()
		}
		['è', s @ ..] => {
			super::push(out, 1473);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ée (3 syllables).
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 568);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2660);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1476);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2661);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'r', 'r', 'o', 'g', s @ ..] => {
			super::push(out, 1477);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['o', 's', s @ ..] => {
					super::push(out, 2662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'n', s @ ..] => {
			super::push(out, 1479);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1480);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2663);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 2664);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3853);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4670);
									s.is_empty()
								}
								['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4671);
									s.is_empty()
								}
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
			super::push(out, 1481);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2665);
					match s {
						[] => true,
						['m', '-', 'e', 'd', '-', 'z', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2666);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2667);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3855);
							match s {
								[] => true,
								['i', 'h', 'c', '-', 'é', 'f', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4672);
									s.is_empty()
								}
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
			super::push(out, 1482);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 2668);
					match s {
						[] => true,
						['u', 'a', 'h', 'c', '-', 'e', 'd', '-', 'z', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üe (3 syllables).
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 569);
	s.is_empty()
}

/// The word œil-de-bœuf (3 syllables).
pub(crate) fn n5553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5366);
	s.is_empty()
}

/// The word masque-cheval (3 syllables).
pub(crate) fn n5554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5367);
	s.is_empty()
}

/// Words ending -eul (3 syllables).
pub(crate) fn n1563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1530);
	match s {
		[] => true,
		['ï', 'a', 's', 'i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 2708);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ussien (3 syllables).
pub(crate) fn n4860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4694);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5144);
			match s {
				[] => true,
				['-', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5442);
							s.is_empty()
						}
						['t', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -in (3 syllables).
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2730);
					match s {
						[] => true,
						['i', 'r', 'f', 'a', '-', s @ ..] => {
							super::push(out, 3904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2731);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3905);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4695);
									match s {
										[] => true,
										['n', 'n', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5145);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'u', 'p', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2732);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3906);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['l', 'l', 'i', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 5146);
											s.is_empty()
										}
										['n', 'g', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 5147);
											s.is_empty()
										}
										['r', 'u', 'e', 'l', 'p', '\u{1}', s @ ..] => {
											super::push(out, 5148);
											s.is_empty()
										}
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
					super::push(out, 2733);
					match s {
						[] => true,
						['r', 'e', 't', '-', 't', 'u', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 3907);
							s.is_empty()
						}
						['t', '-', 'n', 'e', '-', 'e', 't', 'u', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1550);
			match s {
				[] => true,
				['b', 'a', 'r', '-', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1551);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1552);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1553);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1554);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1555);
			match s {
				[] => true,
				['a', 'g', s @ ..] => {
					super::push(out, 2738);
					match s {
						[] => true,
						['a', 'm', '-', 'e', 'd', 'r', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1556);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2739);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3910);
							match s {
								[] => true,
								['-', s @ ..] => {
									match s {
										[] => true,
										['e', 'l', 'l', 'i', 'e', 'v', 'é', 'r', '\u{1}', s @ ..] => {
											super::push(out, 5149);
											s.is_empty()
										}
										['l', 'i', 'e', 'v', 'é', 'r', '\u{1}', s @ ..] => {
											super::push(out, 5150);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'q', 'e', s @ ..] => {
			super::push(out, 1557);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1558);
			match s {
				[] => true,
				['-', 'e', 'd', '-', 't', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2741);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nn (3 syllables).
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -on (3 syllables).
pub(crate) fn n623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1560);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1561);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2744);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1562);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1563);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2747);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1564);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1565);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2749);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1566);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2751);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2752);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2753);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 3914);
							match s {
								[] => true,
								['e', 'd', '-', 't', 'n', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4697);
									s.is_empty()
								}
								['i', 'm', 'r', 'u', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4698);
									s.is_empty()
								}
								['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3915);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2754);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3917);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3918);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4701);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2755);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 3920);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 3921);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2756);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3922);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3923);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 3924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2757);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 3925);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3926);
							match s {
								[] => true,
								['e', 'r', 'p', '-', 'n', 'o', 't', 'u', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 3927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2758);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3928);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4704);
									match s {
										[] => true,
										['i', 'f', 'i', s @ ..] => {
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 5368);
													s.is_empty()
												}
												['n', 'o', s @ ..] => {
													super::push(out, 5369);
													s.is_empty()
												}
												['r', 'o', s @ ..] => {
													super::push(out, 5370);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', 'n', s @ ..] => {
									super::push(out, 4705);
									match s {
										[] => true,
										['o', 'c', s @ ..] => {
											super::push(out, 5151);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'u', 'p', s @ ..] => {
									super::push(out, 4706);
									s.is_empty()
								}
								['m', 'i', s @ ..] => {
									super::push(out, 4707);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4708);
									match s {
										[] => true,
										['-', 't', 'a', 't', 'é', '\u{1}', s @ ..] => {
											super::push(out, 5152);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'u', s @ ..] => {
									super::push(out, 4709);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 5153);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 5154);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'i', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 5155);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 5156);
											s.is_empty()
										}
										['t', 'a', 'r', s @ ..] => {
											super::push(out, 5157);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', s @ ..] => {
									super::push(out, 4710);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 3929);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4711);
									match s {
										[] => true,
										['o', '-', 'e', 'm', 'ê', 'r', 't', 'x', 'e', '\u{1}', s @ ..] => {
											super::push(out, 5158);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'd', s @ ..] => {
									super::push(out, 4712);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 2759);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3931);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 1567);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1568);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2760);
					match s {
						[] => true,
						['s', '-', 'n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n2847(s, out),
				['l', s @ ..] => super::t1::n2848(s, out),
				['o', s @ ..] => super::t1::n2849(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t1::n1603(s, out),
		['n', s @ ..] => super::t1::n1604(s, out),
		['o', s @ ..] => {
			super::push(out, 1571);
			s.is_empty()
		}
		['p', 'u', 'j', '-', 'l', 'a', 'v', 'e', 'h', 'c', '\u{1}', s @ ..] => super::t1::n1606(s, out),
		['r', s @ ..] => super::t1::n1607(s, out),
		['s', s @ ..] => super::t1::n1608(s, out),
		['t', s @ ..] => super::t1::n1609(s, out),
		['v', s @ ..] => super::t1::n1610(s, out),
		['y', s @ ..] => super::t1::n1611(s, out),
		['ç', s @ ..] => super::t1::n1612(s, out),
		['é', s @ ..] => super::t1::n1613(s, out),
		_ => false,
	}
}

/// The word rouge-brun (3 syllables).
pub(crate) fn n1614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1580);
	s.is_empty()
}

/// Words ending -e-de-loup (3 syllables).
pub(crate) fn n5560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5372);
	s.is_empty()
}

/// The word porte-papier (3 syllables).
pub(crate) fn n5561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5373);
	s.is_empty()
}

/// The word gratte-papier (3 syllables).
pub(crate) fn n5562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5374);
	s.is_empty()
}

/// Words ending --pompier (3 syllables).
pub(crate) fn n5563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5375);
	s.is_empty()
}

/// The word bêche-de-mer (3 syllables).
pub(crate) fn n5564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5376);
	s.is_empty()
}

/// The word cardon-de-mer (3 syllables).
pub(crate) fn n5565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5377);
	s.is_empty()
}

/// Words ending -ter (3 syllables).
pub(crate) fn n1647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1613);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2810);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					match s {
						[] => true,
						['-', 'à', '-', 't', 'ê', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4732);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2811);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ir (3 syllables).
pub(crate) fn n643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 630);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['l', 'c', 'é', '-', 'e', 'r', 'r', 'e', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2815);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3969);
							match s {
								[] => true,
								['v', '-', 'e', 'r', 'i', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2816);
					match s {
						[] => true,
						['-', 't', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3970);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3971);
							match s {
								[] => true,
								['o', 't', 'n', 'e', '-', 'r', 'e', 'i', 'n', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2817);
					match s {
						[] => true,
						['s', 'u', s @ ..] => {
							super::push(out, 3972);
							match s {
								[] => true,
								['o', 'p', '-', 'n', 'o', 't', 'u', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4736);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2818);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3973);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4737);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5170);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5378);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2819);
					match s {
						[] => true,
						['a', 'l', '-', 'u', 'a', 'e', 't', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3974);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3975);
							match s {
								[] => true,
								['e', 's', 'é', 'r', '-', 'n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3976);
							match s {
								[] => true,
								['e', 'r', 'b', 'a', '-', 'e', 'n', 'r', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4739);
									s.is_empty()
								}
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
			super::push(out, 1617);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2820);
					match s {
						[] => true,
						['-', 'e', 'd', '-', 'd', 'n', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'j', '-', 'à', '-', 'e', 'n', 'i', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2821);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -or (3 syllables).
pub(crate) fn n644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 631);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2822);
					match s {
						[] => true,
						['m', '-', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'r', 'é', 'n', 'é', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4740);
									s.is_empty()
								}
								['t', 'n', 'e', 'g', 'r', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 4741);
									s.is_empty()
								}
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
			super::push(out, 1620);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3978);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ur (3 syllables).
pub(crate) fn n645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 632);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2823);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 3980);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3981);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4742);
									match s {
										[] => true,
										['-', 'a', 'l', '-', 'e', 'r', 'è', 'p', '\u{1}', s @ ..] => {
											super::push(out, 5171);
											s.is_empty()
										}
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
					super::push(out, 2824);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 3982);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4743);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5172);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5379);
													match s {
														[] => true,
														['t', s @ ..] => super::t1::n5632(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2825);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3983);
							match s {
								[] => true,
								['ê', s @ ..] => {
									super::push(out, 4744);
									match s {
										[] => true,
										['p', '-', 'n', 'i', 't', 'r', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5173);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'a', 'm', '-', 'e', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3984);
							s.is_empty()
						}
						['n', 'o', 'b', '-', 'e', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2826);
					match s {
						[] => true,
						['s', 'n', 'o', 'm', '-', 'e', 'u', 'q', 'o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2827);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3987);
							match s {
								[] => true,
								['h', 'c', '-', 'e', 't', 'l', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3988);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4746);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5174);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3989);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5175);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5380);
													match s {
														[] => true,
														['t', s @ ..] => super::t1::n5633(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 5176);
											match s {
												[] => true,
												['u', 'c', '-', 'r', 'u', 'e', 's', 's', 'a', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5381);
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', s @ ..] => {
											super::push(out, 5177);
											match s {
												[] => true,
												['r', 'o', 't', '-', s @ ..] => {
													super::push(out, 5382);
													match s {
														[] => true,
														['r', 'e', 'i', 'd', 'r', 'a', 'b', 'm', 'o', 'b', '\u{1}', s @ ..] => super::t1::n5634(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 5178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3990);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4747);
									match s {
										[] => true,
										['d', '-', 'e', 'r', 'f', 'f', 'u', 'o', 's', '\u{1}', s @ ..] => {
											super::push(out, 5179);
											s.is_empty()
										}
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
					super::push(out, 2828);
					match s {
						[] => true,
						['u', 'o', 'b', '-', 'é', 'b', 'é', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2829);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3992);
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									super::push(out, 4748);
									match s {
										[] => true,
										['s', s @ ..] => super::t1::n5366(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t1::n4132(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t1::n2918(s, out),
				['p', s @ ..] => super::t1::n2919(s, out),
				['r', s @ ..] => super::t1::n2920(s, out),
				['s', s @ ..] => super::t1::n2921(s, out),
				['t', s @ ..] => super::t1::n2922(s, out),
				['u', s @ ..] => super::t1::n2923(s, out),
				['v', s @ ..] => super::t1::n2924(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n1656(s, out),
		['œ', s @ ..] => super::t1::n1657(s, out),
		_ => false,
	}
}

/// Words ending -dant (3 syllables).
pub(crate) fn n2973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2885);
	s.is_empty()
}

/// Words ending -fant (3 syllables).
pub(crate) fn n2974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2886);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 4033);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4774);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 5199);
							match s {
								[] => true,
								['l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5389);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5390);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5449);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lant (3 syllables).
pub(crate) fn n2975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2887);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4034);
			match s {
				[] => true,
				['v', '-', 'f', 'r', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nant (3 syllables).
pub(crate) fn n2976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2888);
	s.is_empty()
}

/// Words ending -rant (3 syllables).
pub(crate) fn n2977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2889);
	match s {
		[] => true,
		['r', 'e', 'f', '-', 'l', 'a', 'h', 'c', 'é', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4035);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4036);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4776);
					match s {
						[] => true,
						['t', 's', 'e', 'r', '-', s @ ..] => {
							super::push(out, 5200);
							match s {
								[] => true,
								['e', 'r', 'u', 't', 'i', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 5391);
									s.is_empty()
								}
								['n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 5392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sant (3 syllables).
pub(crate) fn n2978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2890);
	s.is_empty()
}

/// Words ending -tant (3 syllables).
pub(crate) fn n2979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2891);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 4037);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4777);
					match s {
						[] => true,
						['b', '-', 'c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vant (3 syllables).
pub(crate) fn n2980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2892);
	match s {
		[] => true,
		['e', 'd', '-', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ent (3 syllables).
pub(crate) fn n1703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1669);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2893);
			match s {
				[] => true,
				['v', '-', 't', 'e', '-', 'a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 4039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2894);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4040);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4778);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5202);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5393);
											match s {
												[] => true,
												['v', s @ ..] => {
													super::push(out, 5450);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 5203);
									match s {
										[] => true,
										['i', 'l', s @ ..] => {
											super::push(out, 5394);
											match s {
												[] => true,
												['l', 'o', s @ ..] => {
													super::push(out, 5451);
													s.is_empty()
												}
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
							super::push(out, 4779);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5204);
									s.is_empty()
								}
								['r', 'a', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5205);
									s.is_empty()
								}
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
			super::push(out, 2895);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4041);
					match s {
						[] => true,
						['p', '-', s @ ..] => {
							super::push(out, 4780);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5206);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5395);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', 'a', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5207);
									s.is_empty()
								}
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
			super::push(out, 2896);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4042);
					match s {
						[] => true,
						['e', 'd', '-', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4781);
							s.is_empty()
						}
						['u', 'a', '-', 'l', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ot (3 syllables).
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1670);
			match s {
				[] => true,
				['a', 's', '-', 'n', 'e', '-', 'c', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2897);
					s.is_empty()
				}
				['o', 'r', '-', 't', 'i', 'a', 'r', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1671);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1672);
			match s {
				[] => true,
				['-', 'e', s @ ..] => {
					super::push(out, 2899);
					match s {
						[] => true,
						['r', 't', 'î', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rt (3 syllables).
pub(crate) fn n664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1673);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2900);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4044);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4783);
									match s {
										[] => true,
										['r', 'i', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 5208);
											s.is_empty()
										}
										['t', 'o', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 5209);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1674);
			match s {
				[] => true,
				['b', 'o', 'r', '-', 'à', '-', 'e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2901);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2902);
					match s {
						[] => true,
						['-', 'e', 'd', '-', 'é', 'r', 'b', 'é', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1675);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4046);
							match s {
								[] => true,
								['e', 'r', 'f', 'f', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4784);
									s.is_empty()
								}
								['u', 'a', 'e', 't', 'â', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2904);
					match s {
						[] => true,
						['-', 'a', 'l', '-', 'e', 'p', 'm', 'o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 4047);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1676);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2906);
					match s {
						[] => true,
						['c', '-', 't', 'ê', 'r', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -st (3 syllables).
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['a', 'l', 'l', 'a', 'b', '-', 'r', 'e', 't', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1677);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1678);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 2907);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2908);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4050);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2909);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1679);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1680);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ut (3 syllables).
pub(crate) fn n666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1681);
			match s {
				[] => true,
				['t', '-', s @ ..] => {
					super::push(out, 2910);
					match s {
						[] => true,
						['e', 'r', 'r', 'u', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4053);
							s.is_empty()
						}
						['s', 'i', 'a', 's', '-', 'e', 'j', '\u{1}', s @ ..] => {
							super::push(out, 4054);
							s.is_empty()
						}
						['à', '-', 'e', 'h', 'c', 'u', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 4055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ôt (3 syllables).
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 2911);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -it-neveu (3 syllables).
pub(crate) fn n5584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5396);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 5453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cou (3 syllables).
pub(crate) fn n1730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1696);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2929);
			match s {
				[] => true,
				['e', 'd', '-', 's', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4068);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dou (3 syllables).
pub(crate) fn n1731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1697);
	match s {
		[] => true,
		['a', 'm', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hou (3 syllables).
pub(crate) fn n1732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1698);
	match s {
		[] => true,
		['c', '-', 'e', 'p', 'u', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jou (3 syllables).
pub(crate) fn n1733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1699);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2932);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rou (3 syllables).
pub(crate) fn n1735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1701);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2933);
			match s {
				[] => true,
				['g', '-', s @ ..] => {
					super::push(out, 4069);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4796);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4797);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4798);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5215);
									match s {
										[] => true,
										['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5397);
											s.is_empty()
										}
										['v', 'e', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 5398);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4799);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4800);
							match s {
								[] => true,
								['i', 'r', 'u', 'o', 's', '-', 'e', 'v', 'u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5216);
									s.is_empty()
								}
								['r', 'u', 'o', '\u{1}', s @ ..] => {
									super::push(out, 5217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4801);
							s.is_empty()
						}
						['u', 'a', 'e', 't', 'e', 'v', 'u', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word grippe-sou (3 syllables).
pub(crate) fn n1736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1702);
	s.is_empty()
}

/// Words ending -ru (3 syllables).
pub(crate) fn n679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 666);
	s.is_empty()
}

/// The word tiramisu (3 syllables).
pub(crate) fn n680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 667);
	s.is_empty()
}

/// Words ending -tu (3 syllables).
pub(crate) fn n681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 668);
	match s {
		[] => true,
		['p', 'm', 'o', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1703);
			s.is_empty()
		}
		['u', 'z', 'n', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 1704);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vu (3 syllables).
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 669);
	match s {
		[] => true,
		['-', 'à', 'j', 'é', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		['é', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
			super::push(out, 1706);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word aperçu (3 syllables).
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 670);
	s.is_empty()
}

/// The word chasse-café (3 syllables).
pub(crate) fn n5587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5399);
	s.is_empty()
}

/// The word pousse-café (3 syllables).
pub(crate) fn n5588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5400);
	s.is_empty()
}

/// Words ending -ité (3 syllables).
pub(crate) fn n1786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1752);
	match s {
		[] => true,
		['c', 'i', 'd', 'é', s @ ..] => {
			super::push(out, 2953);
			s.is_empty()
		}
		['d', 'n', s @ ..] => {
			super::push(out, 2954);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2955);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4806);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 4807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2956);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4079);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4080);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2957);
			s.is_empty()
		}
		['r', 'a', s @ ..] => {
			super::push(out, 2958);
			s.is_empty()
		}
		['s', 'o', 'i', s @ ..] => {
			super::push(out, 2959);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['q', 'é', s @ ..] => {
					super::push(out, 4082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4084);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -été (3 syllables).
pub(crate) fn n1792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1758);
	match s {
		[] => true,
		['i', 'p', s @ ..] => {
			super::push(out, 2964);
			match s {
				[] => true,
				['-', 'e', 'd', '-', 't', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ôté (3 syllables).
pub(crate) fn n1793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1759);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2965);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ué (3 syllables).
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 702);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 1760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vé (3 syllables).
pub(crate) fn n716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 703);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ende (3 syllables).
pub(crate) fn n2198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2148);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3131);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3132);
			match s {
				[] => true,
				['a', '-', 'r', 'u', 'o', 'j', '\u{1}', s @ ..] => {
					super::push(out, 4150);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onde (3 syllables).
pub(crate) fn n2199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2149);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3133);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3134);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4152);
					match s {
						[] => true,
						['e', 'l', 'l', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', '-', 'n', 'o', 'l', 'l', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3135);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ode (3 syllables).
pub(crate) fn n1347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1319);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2150);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2151);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3136);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 4153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2152);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2153);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rde (3 syllables).
pub(crate) fn n1348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1320);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3139);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 4154);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 4827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3141);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4155);
							match s {
								[] => true,
								['r', '-', 'e', 'm', 'm', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4828);
									s.is_empty()
								}
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
			super::push(out, 2156);
			match s {
				[] => true,
				['m', '-', s @ ..] => {
					super::push(out, 3142);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4156);
							match s {
								[] => true,
								['g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4829);
									s.is_empty()
								}
								['l', 'l', 'i', 'u', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4830);
									s.is_empty()
								}
								['u', 'q', 'a', 'l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4831);
									s.is_empty()
								}
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
			super::push(out, 2157);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4157);
							s.is_empty()
						}
						['i', 'n', s @ ..] => {
							super::push(out, 4158);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4159);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4160);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4832);
									s.is_empty()
								}
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
			super::push(out, 2158);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ude (3 syllables).
pub(crate) fn n1349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1321);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2159);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3143);
					match s {
						[] => true,
						['c', '-', 'e', 'n', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2160);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2161);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yde (3 syllables).
pub(crate) fn n1350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1322);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2162);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2163);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 2164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ède (3 syllables).
pub(crate) fn n1351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1323);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 2165);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ïde (3 syllables).
pub(crate) fn n1352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1324);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2166);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3146);
					match s {
						[] => true,
						['p', 'y', s @ ..] => {
							super::push(out, 4162);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						['i', 't', s @ ..] => {
							super::push(out, 4167);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3148);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 4169);
							match s {
								[] => true,
								['h', 't', s @ ..] => {
									super::push(out, 4834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 4170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3150);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nge (3 syllables).
pub(crate) fn n1360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1331);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2187);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['c', 'é', s @ ..] => {
							super::push(out, 4184);
							match s {
								[] => true,
								['-', 'e', 'r', 'b', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3183);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 4185);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4843);
									s.is_empty()
								}
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
			super::push(out, 2188);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4186);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'è', 's', '\u{1}', s @ ..] => {
											super::push(out, 5235);
											s.is_empty()
										}
										['v', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 5236);
											s.is_empty()
										}
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
					super::push(out, 3185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2189);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rge (3 syllables).
pub(crate) fn n1361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1332);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2190);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3187);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2191);
			match s {
				[] => true,
				['g', '-', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'u', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4187);
							s.is_empty()
						}
						['n', 'e', 'i', 't', 'u', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 4188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2192);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3189);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ouge (3 syllables).
pub(crate) fn n2246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2194);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['-', 'u', 'a', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4190);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 4191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ège (3 syllables).
pub(crate) fn n1364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1335);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2195);
			match s {
				[] => true,
				['l', '-', 'e', 'n', 'ê', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3190);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word pied-de-mouche (3 syllables).
pub(crate) fn n5589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5401);
	s.is_empty()
}

/// Words ending -au-mouche (3 syllables).
pub(crate) fn n5590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5402);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5454);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sainte-nitouche (3 syllables).
pub(crate) fn n5591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5403);
	s.is_empty()
}

/// Words ending -phe (3 syllables).
pub(crate) fn n1367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1338);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2203);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3210);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4848);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 4849);
									match s {
										[] => true,
										['é', s @ ..] => {
											super::push(out, 5242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4850);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5243);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 4851);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 4852);
									s.is_empty()
								}
								['i', 'd', s @ ..] => {
									super::push(out, 4853);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5244);
											s.is_empty()
										}
										['y', 't', 'c', s @ ..] => {
											super::push(out, 5245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['o', 'r', s @ ..] => {
											super::push(out, 5246);
											s.is_empty()
										}
										['é', s @ ..] => {
											super::push(out, 5247);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 4854);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4855);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5248);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 5249);
											match s {
												[] => true,
												['h', s @ ..] => {
													super::push(out, 5404);
													match s {
														[] => true,
														['p', s @ ..] => super::t1::n5643(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 5250);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5405);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['é', s @ ..] => {
									super::push(out, 4856);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 5251);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2204);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2205);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 3213);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2206);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3214);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4210);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4857);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4858);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5252);
											s.is_empty()
										}
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
					super::push(out, 3215);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2207);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sie (3 syllables).
pub(crate) fn n1382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1351);
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 2239);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tie (3 syllables).
pub(crate) fn n1383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1352);
	s.is_empty()
}

/// Words ending -uie (3 syllables).
pub(crate) fn n1384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1353);
	s.is_empty()
}

/// Words ending -vie (3 syllables).
pub(crate) fn n1385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2240);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3248);
					match s {
						[] => true,
						['c', 'n', 'a', 'r', 'u', 's', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4229);
							s.is_empty()
						}
						['d', '-', 'u', 'a', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lexie (3 syllables).
pub(crate) fn n1386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	s.is_empty()
}

/// Words ending --comptable (3 syllables).
pub(crate) fn n5594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5406);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5456);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omobile (3 syllables).
pub(crate) fn n4390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4243);
	s.is_empty()
}

/// Words ending -acile (3 syllables).
pub(crate) fn n2320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2263);
	s.is_empty()
}

/// Words ending -dile (3 syllables).
pub(crate) fn n2321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2264);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3271);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4244);
					match s {
						[] => true,
						['o', 'r', 'c', '-', 'n', 'i', 'u', 'q', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gile (3 syllables).
pub(crate) fn n2322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2265);
	s.is_empty()
}

/// Words ending -hile (3 syllables).
pub(crate) fn n2323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2266);
	match s {
		[] => true,
		['p', 'o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4245);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4246);
					s.is_empty()
				}
				['l', 'a', s @ ..] => {
					super::push(out, 4247);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4248);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4249);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4250);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4251);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4252);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nile (3 syllables).
pub(crate) fn n2324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2267);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oile (3 syllables).
pub(crate) fn n2325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2268);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 3273);
			match s {
				[] => true,
				['é', s @ ..] => {
					super::push(out, 4253);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pile (3 syllables).
pub(crate) fn n2326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2269);
	match s {
		[] => true,
		['o', 'r', s @ ..] => {
			super::push(out, 3274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rile (3 syllables).
pub(crate) fn n2327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2270);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 3275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sile (3 syllables).
pub(crate) fn n2328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2271);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tile (3 syllables).
pub(crate) fn n2329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2272);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lle (3 syllables).
pub(crate) fn n1394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1363);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2273);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2274);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3278);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3279);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3280);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3281);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3282);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4254);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4872);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 5259);
											s.is_empty()
										}
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
					super::push(out, 3283);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3284);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4255);
							match s {
								[] => true,
								['o', 'm', 'e', 'd', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4873);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', s @ ..] => {
							super::push(out, 4256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3285);
					s.is_empty()
				}
				['v', 'a', 'r', s @ ..] => {
					super::push(out, 3286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2275);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3287);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4257);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4874);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5260);
											match s {
												[] => true,
												['s', 's', 'a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5407);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3288);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3289);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4258);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 4875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3290);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3291);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4260);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4261);
							match s {
								[] => true,
								['u', 'o', 'b', s @ ..] => {
									super::push(out, 4876);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 5261);
											s.is_empty()
										}
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
					super::push(out, 3292);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4262);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4877);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 5262);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'm', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 4878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3293);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4263);
							match s {
								[] => true,
								['f', '-', 'e', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3294);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3295);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3296);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4264);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4265);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3297);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3298);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4267);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4268);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4881);
									match s {
										[] => true,
										['e', 'r', s @ ..] => {
											super::push(out, 5263);
											match s {
												[] => true,
												['g', '-', 'e', 'm', 'm', 'o', 'h', '\u{1}', s @ ..] => {
													super::push(out, 5408);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 4882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['q', s @ ..] => {
							super::push(out, 4269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3299);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4270);
							match s {
								[] => true,
								['e', 'r', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4883);
									s.is_empty()
								}
								['n', 'e', '-', 'e', 's', 'i', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4271);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2276);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2277);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3300);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ole (3 syllables).
pub(crate) fn n1395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1364);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2278);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 3301);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 4273);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3302);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4274);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3303);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2279);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3305);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3306);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2280);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3308);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 3309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2281);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3310);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4277);
							match s {
								[] => true,
								['-', 'e', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2282);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2283);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', 'r', s @ ..] => {
			super::push(out, 2284);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ple (3 syllables).
pub(crate) fn n1396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1365);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2285);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rle (3 syllables).
pub(crate) fn n1397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1366);
	s.is_empty()
}

/// Words ending -ule (3 syllables).
pub(crate) fn n1398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1367);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3314);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3315);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3316);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4279);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4887);
									s.is_empty()
								}
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
			super::push(out, 2288);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4280);
							s.is_empty()
						}
						['d', 'é', s @ ..] => {
							super::push(out, 4281);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4888);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4889);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5264);
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 5409);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'u', 'p', s @ ..] => {
					super::push(out, 3317);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2289);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3319);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3320);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2290);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3322);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4283);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4890);
									match s {
										[] => true,
										['e', 'l', 'û', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ï', s @ ..] => {
					super::push(out, 3323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2291);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3324);
					match s {
						[] => true,
						['i', 'v', '-', 't', 'n', 'i', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2292);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2293);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2294);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2295);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3325);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 3326);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3327);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4285);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4891);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5266);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5410);
													match s {
														[] => true,
														['d', 'i', 'n', '\u{1}', s @ ..] => super::t1::n5645(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2296);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2297);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yle (3 syllables).
pub(crate) fn n1399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1368);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2298);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2299);
			s.is_empty()
		}
		['t', 's', s @ ..] => {
			super::push(out, 2300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -èle (3 syllables).
pub(crate) fn n1400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1369);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2301);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					super::push(out, 3329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'o', s @ ..] => {
			super::push(out, 2302);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2303);
			s.is_empty()
		}
		['t', 'n', s @ ..] => {
			super::push(out, 2304);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -êle (3 syllables).
pub(crate) fn n1401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1370);
	match s {
		[] => true,
		['m', '-', 'e', 'l', 'ê', 'p', '\u{1}', s @ ..] => {
			super::push(out, 2305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ôle (3 syllables).
pub(crate) fn n1402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1371);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 2306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -forme (3 syllables).
pub(crate) fn n3462(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4299);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4898);
					match s {
						[] => true,
						['d', '-', 't', 'u', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5270);
							s.is_empty()
						}
						['t', 'a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4300);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4301);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 4302);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sme (3 syllables).
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1380);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2333);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 3353);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 4303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3354);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 4304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 4305);
							match s {
								[] => true,
								['o', 'i', 't', 'a', 'n', s @ ..] => {
									super::push(out, 4901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 4306);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 4902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'é', s @ ..] => {
					super::push(out, 3355);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3356);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							super::push(out, 4308);
							match s {
								[] => true,
								['é', 'l', '-', 'e', 'm', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 4903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3357);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3358);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4309);
							s.is_empty()
						}
						['é', 'n', s @ ..] => {
							super::push(out, 4310);
							match s {
								[] => true,
								['g', 'a', 'm', 'o', 'r', s @ ..] => {
									super::push(out, 4904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 3359);
					match s {
						[] => true,
						['h', 't', 'n', s @ ..] => {
							super::push(out, 4311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ï', s @ ..] => {
			super::push(out, 2334);
			match s {
				[] => true,
				['o', 'a', 'm', '-', 'e', 'm', 's', 'i', 'n', 'i', 'n', 'é', 'l', '-', 'e', 'm', 's', 'i', 'x', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3360);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yme (3 syllables).
pub(crate) fn n1413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1382);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2336);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2337);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2338);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3362);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ème (3 syllables).
pub(crate) fn n1414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1383);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2339);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2340);
			s.is_empty()
		}
		['i', 't', 'n', s @ ..] => {
			super::push(out, 2341);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2342);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3363);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -maine (3 syllables).
pub(crate) fn n3501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3390);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 4325);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -taine (3 syllables).
pub(crate) fn n3502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3391);
	match s {
		[] => true,
		['i', 'm', s @ ..] => {
			super::push(out, 4326);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4327);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4909);
					match s {
						[] => true,
						['f', '-', 'e', 'n', 'r', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bine (3 syllables).
pub(crate) fn n2420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2359);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 3393);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cine (3 syllables).
pub(crate) fn n2421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2360);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3394);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4328);
					match s {
						[] => true,
						['-', 'e', 'm', 'u', 'g', 'é', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 4329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3395);
			match s {
				[] => true,
				['d', 'é', 'm', '-', 'e', 'm', 'm', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eine (3 syllables).
pub(crate) fn n2423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2362);
	match s {
		[] => true,
		['l', 'a', s @ ..] => {
			super::push(out, 3397);
			match s {
				[] => true,
				['b', '-', 'n', 'i', 'u', 'q', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fine (3 syllables).
pub(crate) fn n2424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2363);
	match s {
		[] => true,
		['é', s @ ..] => {
			super::push(out, 3398);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -agine (3 syllables).
pub(crate) fn n2425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2364);
	s.is_empty()
}

/// Words ending -hine (3 syllables).
pub(crate) fn n2426(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 3399);
			s.is_empty()
		}
		['t', 'u', s @ ..] => {
			super::push(out, 3400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kine (3 syllables).
pub(crate) fn n2427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2365);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3401);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -line (3 syllables).
pub(crate) fn n2428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2366);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 3402);
			s.is_empty()
		}
		['u', 'p', s @ ..] => {
			super::push(out, 3403);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3404);
			s.is_empty()
		}
		['é', 'y', s @ ..] => {
			super::push(out, 3405);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mine (3 syllables).
pub(crate) fn n2429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2367);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 3406);
			s.is_empty()
		}
		['a', 't', s @ ..] => {
			super::push(out, 3407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nnine (3 syllables).
pub(crate) fn n2430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2368);
	s.is_empty()
}

/// Words ending -oine (3 syllables).
pub(crate) fn n2431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2369);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3408);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pine (3 syllables).
pub(crate) fn n2432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2370);
	match s {
		[] => true,
		['é', s @ ..] => {
			super::push(out, 3410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rine (3 syllables).
pub(crate) fn n2433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2371);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4332);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3411);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3412);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					super::push(out, 4334);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sine (3 syllables).
pub(crate) fn n2434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2372);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3413);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 4335);
					match s {
						[] => true,
						['é', 'b', '-', 'e', 'l', 'b', 'u', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tine (3 syllables).
pub(crate) fn n2435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2373);
	match s {
		[] => true,
		['n', 'a', 's', s @ ..] => {
			super::push(out, 3414);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 3415);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uine (3 syllables).
pub(crate) fn n2436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2374);
	s.is_empty()
}

/// Words ending -lavine (3 syllables).
pub(crate) fn n2437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2375);
	s.is_empty()
}

/// Words ending -notoxine (3 syllables).
pub(crate) fn n2438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2376);
	s.is_empty()
}

/// Words ending -zine (3 syllables).
pub(crate) fn n2439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2377);
	match s {
		[] => true,
		['a', 'i', s @ ..] => {
			super::push(out, 3416);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4336);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oprotéine (3 syllables).
pub(crate) fn n2440(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'é', s @ ..] => {
			super::push(out, 3417);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 3418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mne (3 syllables).
pub(crate) fn n1419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2378);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3419);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nne (3 syllables).
pub(crate) fn n1420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1388);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2379);
			match s {
				[] => true,
				['e', 'j', '-', 'e', 'm', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2380);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 3421);
					match s {
						[] => true,
						['o', 'd', s @ ..] => {
							super::push(out, 4338);
							match s {
								[] => true,
								['a', 'r', 'b', 'a', 'l', '-', 't', 'e', '-', 'e', 'n', 'n', 'e', 'i', 'v', 'u', 'e', 'n', '-', 'e', 'r', 'r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 4913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2381);
			match s {
				[] => true,
				['l', 'l', s @ ..] => {
					super::push(out, 3423);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3424);
					match s {
						[] => true,
						['-', 'e', 'd', '-', 't', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'r', s @ ..] => {
					super::push(out, 3425);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -one (3 syllables).
pub(crate) fn n1421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1389);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2382);
			s.is_empty()
		}
		['c', 'i', s @ ..] => {
			super::push(out, 2383);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2384);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2385);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2386);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3428);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4340);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4341);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4914);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 4915);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4916);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4917);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4918);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5273);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4919);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 5274);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 4342);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4920);
									match s {
										[] => true,
										['é', s @ ..] => {
											super::push(out, 5275);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2387);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2388);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3429);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 4343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2389);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3430);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2390);
			match s {
				[] => true,
				['é', 'h', s @ ..] => {
					super::push(out, 3431);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4345);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4921);
									s.is_empty()
								}
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
			super::push(out, 2391);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2392);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3432);
					s.is_empty()
				}
				['é', 't', 's', 'o', 't', s @ ..] => {
					super::push(out, 3433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2393);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3434);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3435);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3436);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4347);
							s.is_empty()
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
				['c', s @ ..] => {
					super::push(out, 3437);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4348);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3438);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4349);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3439);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2394);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rne (3 syllables).
pub(crate) fn n1422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1390);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2395);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3442);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3443);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4350);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4924);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 5276);
											match s {
												[] => true,
												['e', 'r', 'i', 'v', 'a', 'n', '\u{1}', s @ ..] => {
													super::push(out, 5411);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 5412);
													match s {
														[] => true,
														['o', s @ ..] => super::t1::n5646(s, out),
														_ => false,
													}
												}
												['u', 'a', 'e', 't', 'a', 'b', '\u{1}', s @ ..] => {
													super::push(out, 5413);
													s.is_empty()
												}
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
							super::push(out, 4351);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4925);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 5277);
											match s {
												[] => true,
												['-', 'r', 'u', 'o', 't', '\u{1}', s @ ..] => {
													super::push(out, 5414);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3444);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2396);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 3445);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4353);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -une (3 syllables).
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	match s {
		[] => true,
		['a', 'f', s @ ..] => {
			super::push(out, 2398);
			s.is_empty()
		}
		['e', 'h', 'g', 'i', 'z', '\u{1}', s @ ..] => {
			super::push(out, 2399);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2400);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 3446);
					match s {
						[] => true,
						['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 't', s @ ..] => {
			super::push(out, 2401);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yne (3 syllables).
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['d', 'o', s @ ..] => {
			super::push(out, 2402);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2403);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -âne (3 syllables).
pub(crate) fn n1425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1393);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2404);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ène (3 syllables).
pub(crate) fn n1426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3450);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3451);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3452);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3453);
					match s {
						[] => true,
						['-', s @ ..] => {
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
			super::push(out, 2405);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3454);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3455);
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 4357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3457);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2406);
			match s {
				[] => true,
				['d', 'a', 't', s @ ..] => {
					super::push(out, 3459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3460);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3461);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4358);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3462);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3463);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4359);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 4927);
									match s {
										[] => true,
										['c', 'a', 'l', s @ ..] => {
											super::push(out, 5278);
											match s {
												[] => true,
												['y', s @ ..] => {
													super::push(out, 5415);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 3464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2407);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3465);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2408);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 3467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3468);
					s.is_empty()
				}
				['h', 'p', s @ ..] => {
					super::push(out, 3469);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3470);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2409);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3471);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2411);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 2412);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2413);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3473);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -êne (3 syllables).
pub(crate) fn n1427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	s.is_empty()
}

/// Words ending -ïne (3 syllables).
pub(crate) fn n1428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	s.is_empty()
}

/// Words ending -icône (3 syllables).
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	s.is_empty()
}

/// Words ending -ire (3 syllables).
pub(crate) fn n1444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1412);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2440);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3499);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3500);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4380);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 4381);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4937);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 5280);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4382);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4383);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4384);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4939);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5281);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4940);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4385);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3501);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4387);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4942);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4388);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4944);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 5282);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'x', s @ ..] => {
									super::push(out, 4945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4389);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4947);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 4948);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 4949);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5283);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5416);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 4950);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4951);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5284);
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 5417);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3502);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3503);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4390);
							match s {
								[] => true,
								['t', 'n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5285);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 5286);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4952);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4953);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4391);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4955);
									match s {
										[] => true,
										['i', 's', 's', s @ ..] => {
											super::push(out, 5287);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4392);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4956);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4957);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => super::t1::n4544(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3504);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 4393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3505);
					s.is_empty()
				}
				['s', s @ ..] => super::t1::n3621(s, out),
				['t', s @ ..] => super::t1::n3622(s, out),
				['u', 'q', 'i', 't', s @ ..] => {
					super::push(out, 3508);
					s.is_empty()
				}
				['v', s @ ..] => super::t1::n3624(s, out),
				['é', s @ ..] => super::t1::n3625(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t1::n2513(s, out),
		['o', s @ ..] => super::t1::n2514(s, out),
		['p', s @ ..] => super::t1::n2515(s, out),
		['r', s @ ..] => super::t1::n2516(s, out),
		_ => false,
	}
}

/// Words ending -arre (3 syllables).
pub(crate) fn n2527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2455);
	s.is_empty()
}

/// Words ending -erre (3 syllables).
pub(crate) fn n2528(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3529);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4410);
					match s {
						[] => true,
						['-', 't', 'n', 'i', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3530);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4411);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3531);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4412);
					match s {
						[] => true,
						['à', '-', 'd', 'e', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3532);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4413);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4983);
							match s {
								[] => true,
								['n', 'e', '-', 't', '-', 'a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 5294);
									s.is_empty()
								}
								['s', 'è', 'r', 'p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5295);
									s.is_empty()
								}
								['t', 'n', 'a', 'v', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tre (3 syllables).
pub(crate) fn n1449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1417);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2456);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3533);
					match s {
						[] => true,
						['h', 'c', 'y', s @ ..] => {
							super::push(out, 4414);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4984);
									s.is_empty()
								}
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
			super::push(out, 2457);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3534);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4415);
							match s {
								[] => true,
								['a', '-', 'e', 'r', 'b', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 4985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3535);
					match s {
						[] => true,
						['u', 'p', '-', 'o', 'l', 'é', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2458);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3536);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3537);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4417);
							match s {
								[] => true,
								['-', 'e', 't', 'u', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 4986);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4987);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5297);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4418);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 4988);
									match s {
										[] => true,
										['a', 'l', '-', 'e', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 5298);
											s.is_empty()
										}
										['t', 'e', 'l', 'e', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5299);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2459);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3538);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4419);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4989);
									match s {
										[] => true,
										['r', 'o', '-', 'e', 'm', 'm', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 5300);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4420);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4990);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5301);
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 5419);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'n', s @ ..] => {
					super::push(out, 3539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', 'l', '-', 'e', 't', 'r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2460);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2461);
			match s {
				[] => true,
				['o', 'f', '-', 'n', 'a', 'e', 'j', '\u{1}', s @ ..] => {
					super::push(out, 3540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['â', s @ ..] => {
			super::push(out, 2462);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3541);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4421);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3542);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3543);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4422);
							match s {
								[] => true,
								['t', '-', 'é', 'f', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['è', s @ ..] => {
			super::push(out, 2463);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', 'o', s @ ..] => {
							super::push(out, 4423);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									super::push(out, 4993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4994);
									s.is_empty()
								}
								['é', s @ ..] => {
									super::push(out, 4995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ê', s @ ..] => {
			super::push(out, 2464);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3544);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4424);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 4996);
									match s {
										[] => true,
										['-', 'e', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 5302);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['î', s @ ..] => {
			super::push(out, 2465);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3545);
					match s {
						[] => true,
						['m', '-', s @ ..] => {
							super::push(out, 4425);
							match s {
								[] => true,
								['r', 'e', 'i', 't', 'r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 4997);
									s.is_empty()
								}
								['t', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ure (3 syllables).
pub(crate) fn n1450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1418);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2466);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3546);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2467);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3548);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4427);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4999);
									s.is_empty()
								}
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
			super::push(out, 2468);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2469);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3550);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3551);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4429);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5000);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5303);
											s.is_empty()
										}
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
					super::push(out, 3552);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3554);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3555);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2471);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3557);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3558);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4432);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5001);
									s.is_empty()
								}
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
			super::push(out, 2472);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2473);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3560);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2474);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3562);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3563);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4433);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2475);
			match s {
				[] => true,
				['a', 'y', s @ ..] => {
					super::push(out, 3565);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3566);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3567);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3568);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3569);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3570);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2476);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2477);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2478);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3571);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3572);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 4438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3573);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3575);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4440);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2479);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 3576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2480);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				['c', 'e', 'f', s @ ..] => {
					super::push(out, 3578);
					s.is_empty()
				}
				['l', 'u', 'c', 'o', s @ ..] => {
					super::push(out, 3579);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3580);
					match s {
						[] => true,
						['i', 'e', 'c', s @ ..] => {
							super::push(out, 4442);
							match s {
								[] => true,
								['-', 'e', 'd', 'o', 'g', '\u{1}', s @ ..] => {
									super::push(out, 5003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3581);
					s.is_empty()
				}
				['p', 'a', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 2481);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['l', '-', 'n', 'i', 'a', 'v', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ç', s @ ..] => {
			super::push(out, 2482);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vre (3 syllables).
pub(crate) fn n1451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1419);
	match s {
		[] => true,
		['è', s @ ..] => {
			super::push(out, 2483);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3585);
					match s {
						[] => true,
						['l', '-', 'e', 'd', '-', 'c', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yre (3 syllables).
pub(crate) fn n1452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1420);
	match s {
		[] => true,
		['t', 'r', 'a', 'm', 'o', s @ ..] => {
			super::push(out, 2484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ère (3 syllables).
pub(crate) fn n1453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1421);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2485);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3586);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2486);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2487);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3587);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2488);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2489);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2490);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 3589);
					s.is_empty()
				}
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4447);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5004);
									match s {
										[] => true,
										['é', s @ ..] => {
											super::push(out, 5304);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4448);
							match s {
								[] => true,
								['h', 'c', s @ ..] => {
									super::push(out, 5005);
									s.is_empty()
								}
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
			super::push(out, 2491);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3590);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3591);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4449);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4450);
							s.is_empty()
						}
						['u', 'a', s @ ..] => {
							super::push(out, 4451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3592);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4452);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5006);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 5007);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 5305);
											match s {
												[] => true,
												['e', s @ ..] => {
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
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3593);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3594);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4453);
							match s {
								[] => true,
								['o', 'p', '-', 'e', 's', 'u', 'e', 'p', 'a', 's', '\u{1}', s @ ..] => {
									super::push(out, 5008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4454);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5009);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 5306);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 5421);
													match s {
														[] => true,
														['e', s @ ..] => super::t1::n5647(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['c', s @ ..] => {
											super::push(out, 5307);
											match s {
												[] => true,
												['-', 'e', 's', 'u', 'a', 'p', '\u{1}', s @ ..] => {
													super::push(out, 5422);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3596);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3597);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							super::push(out, 4455);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4456);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5010);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5308);
											match s {
												[] => true,
												['f', '-', s @ ..] => {
													super::push(out, 5423);
													match s {
														[] => true,
														['e', s @ ..] => super::t1::n5648(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2492);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 3599);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4457);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5011);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4458);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5012);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 5309);
											s.is_empty()
										}
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
					super::push(out, 3600);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 3601);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2494);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2495);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 3602);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4460);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5013);
									match s {
										[] => true,
										['a', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 5310);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'a', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3603);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 4462);
							match s {
								[] => true,
								['-', 'n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 5014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3604);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2496);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3605);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4464);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5015);
									s.is_empty()
								}
								['u', 'a', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n2572(s, out),
		['y', s @ ..] => {
			super::push(out, 2498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üre (3 syllables).
pub(crate) fn n1454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1422);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2499);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -isse (3 syllables).
pub(crate) fn n2600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2524);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3640);
			match s {
				[] => true,
				['c', '-', 'r', 'i', 'o', 'r', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 4482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3641);
			match s {
				[] => true,
				['s', '-', 't', 'i', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -osse (3 syllables).
pub(crate) fn n2601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2525);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3642);
			match s {
				[] => true,
				['-', 'e', 'd', 'n', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3643);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4485);
					match s {
						[] => true,
						['e', 's', 's', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3644);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3645);
			match s {
				[] => true,
				['b', '-', 'i', 'a', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usse (3 syllables).
pub(crate) fn n2602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2526);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4488);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4489);
					match s {
						[] => true,
						['-', 'e', 's', 's', 'u', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3646);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -use (3 syllables).
pub(crate) fn n1462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1430);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2528);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3647);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 4492);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5025);
									match s {
										[] => true,
										['-', 'r', 'u', 'e', 't', 'c', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 5311);
											s.is_empty()
										}
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
					super::push(out, 3648);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4493);
							match s {
								[] => true,
								['n', '-', s @ ..] => {
									super::push(out, 5026);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5312);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5424);
													match s {
														[] => true,
														['t', s @ ..] => super::t1::n5649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3649);
					match s {
						[] => true,
						['c', 'é', s @ ..] => {
							super::push(out, 4494);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5027);
									match s {
										[] => true,
										['-', 'e', 's', 'u', 'e', 'v', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 5313);
											s.is_empty()
										}
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
					super::push(out, 3650);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3651);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4495);
							match s {
								[] => true,
								['o', 'b', '-', 'é', 'b', 'é', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5028);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3653);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3654);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4496);
							match s {
								[] => true,
								['a', 'h', s @ ..] => {
									super::push(out, 5029);
									match s {
										[] => true,
										['c', '-', 'e', 'r', 't', 'î', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5314);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4497);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 5030);
									match s {
										[] => true,
										['-', 'e', 's', 'u', 'e', 'n', 'n', 'o', 's', 's', 'i', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5315);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3655);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 4498);
							match s {
								[] => true,
								['n', 'o', 'm', '-', 'e', 's', 's', 'u', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 5031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -èse (3 syllables).
pub(crate) fn n1464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1432);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2529);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3656);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4499);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5032);
									s.is_empty()
								}
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
			super::push(out, 2530);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2531);
			s.is_empty()
		}
		['i', 'd', '-', 't', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 2532);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2533);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3657);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4500);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5033);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5316);
											s.is_empty()
										}
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
					super::push(out, 3658);
					s.is_empty()
				}
				['i', 'k', 'o', 'h', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ôse (3 syllables).
pub(crate) fn n1465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1433);
	s.is_empty()
}

/// Words ending --démocrate (3 syllables).
pub(crate) fn n5613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5425);
	match s {
		[] => true,
		['e', 'l', 'a', 'r', 'é', 'b', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5462);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5463);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -glycérate (3 syllables).
pub(crate) fn n5614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5426);
	s.is_empty()
}

/// Words ending -ite (3 syllables).
pub(crate) fn n1468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1436);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2550);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3681);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3682);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3683);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3684);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2551);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3686);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'n', s @ ..] => {
			super::push(out, 2552);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3688);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4511);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2553);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3690);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3691);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4513);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4514);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4515);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4516);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4517);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3692);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3693);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3694);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4518);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4519);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5040);
									s.is_empty()
								}
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
			super::push(out, 2554);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3695);
					s.is_empty()
				}
				['r', 'e', 'd', s @ ..] => {
					super::push(out, 3696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2555);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3697);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4520);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3698);
					s.is_empty()
				}
				['é', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4522);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4523);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5041);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5320);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2556);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 3699);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4524);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3701);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3702);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4525);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3703);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3704);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3705);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4528);
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
				['a', s @ ..] => {
					super::push(out, 3706);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4529);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3707);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3708);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2557);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 3710);
					s.is_empty()
				}
				['c', 'e', s @ ..] => {
					super::push(out, 3711);
					s.is_empty()
				}
				['o', 'z', s @ ..] => {
					super::push(out, 3712);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4531);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 5045);
									s.is_empty()
								}
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
			super::push(out, 2558);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3714);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3715);
					match s {
						[] => true,
						['r', 'u', 'o', 'p', '-', 'e', 's', 'r', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', 'o', s @ ..] => {
			super::push(out, 2559);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 2560);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ante (3 syllables).
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2563);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3718);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3719);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 3720);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3721);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3722);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3723);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3724);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4534);
					match s {
						[] => true,
						['d', 'n', 'a', 'r', 'g', '-', 'e', 'r', 'è', 'i', 'r', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4535);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5047);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5321);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5427);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 5464);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'p', '-', 'e', 't', 'ê', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inte (3 syllables).
pub(crate) fn n2647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2565);
	s.is_empty()
}

/// Words ending -onte (3 syllables).
pub(crate) fn n2648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2566);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3729);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ote (3 syllables).
pub(crate) fn n1471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1439);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2567);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2568);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2569);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 3731);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 4538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2570);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3732);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4539);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 5048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3733);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3734);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3735);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2571);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3736);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3737);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4541);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 5049);
									match s {
										[] => true,
										['e', 'n', 'i', 'e', 'l', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5322);
											s.is_empty()
										}
										['n', 'o', 's', 's', 'i', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 5323);
											s.is_empty()
										}
										['u', 'a', 'e', 't', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 5324);
											s.is_empty()
										}
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
					super::push(out, 3738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2572);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3739);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3740);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2573);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2574);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3742);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2575);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3743);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pte (3 syllables).
pub(crate) fn n1472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1440);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2576);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3744);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rte (3 syllables).
pub(crate) fn n1473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1441);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2577);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2578);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2579);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3745);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4543);
							match s {
								[] => true,
								['n', 'i', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5050);
									s.is_empty()
								}
								['u', 'a', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3746);
					match s {
						[] => true,
						['-', 'e', 'd', '-', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ste (3 syllables).
pub(crate) fn n1474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1442);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2580);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3747);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'm', s @ ..] => {
					super::push(out, 3748);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3749);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3750);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2581);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3751);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2582);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3753);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4547);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'e', s @ ..] => {
							super::push(out, 4548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 3754);
					match s {
						[] => true,
						['a', 'g', s @ ..] => {
							super::push(out, 4549);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4550);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4551);
							match s {
								[] => true,
								['l', 'o', s @ ..] => {
									match s {
										[] => true,
										['i', 's', s @ ..] => {
											super::push(out, 5325);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 5326);
											s.is_empty()
										}
										['t', 's', s @ ..] => {
											super::push(out, 5327);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'p', 'o', s @ ..] => {
					super::push(out, 3755);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3756);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5053);
									s.is_empty()
								}
								['r', 'é', s @ ..] => {
									super::push(out, 5054);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 5055);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', s @ ..] => {
					super::push(out, 3757);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4555);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4556);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5056);
									s.is_empty()
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
						['a', s @ ..] => {
							super::push(out, 4557);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5058);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									super::push(out, 5059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 5060);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 5061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3758);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4558);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4559);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4561);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 5062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 4562);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 5063);
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
						['i', s @ ..] => {
							super::push(out, 4563);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4565);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							super::push(out, 4566);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', 'r', s @ ..] => {
							super::push(out, 4567);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5065);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5328);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 5428);
													s.is_empty()
												}
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
							super::push(out, 4568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3759);
					match s {
						[] => true,
						['g', 'o', s @ ..] => {
							super::push(out, 4569);
							s.is_empty()
						}
						['q', 'a', s @ ..] => {
							super::push(out, 4570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', s @ ..] => {
					super::push(out, 3760);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 3761);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2583);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3762);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 4572);
							match s {
								[] => true,
								['e', 'r', 'b', 'm', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 5066);
									s.is_empty()
								}
								['n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 5067);
									s.is_empty()
								}
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
			super::push(out, 2584);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3763);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4573);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3764);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4574);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 5069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => super::t1::n2667(s, out),
		['ï', s @ ..] => {
			super::push(out, 2586);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tte (3 syllables).
pub(crate) fn n1475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1443);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2587);
			match s {
				[] => true,
				['j', '-', 'e', 'd', '-', 'l', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2588);
			match s {
				[] => true,
				['b', 'm', s @ ..] => {
					super::push(out, 3770);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3771);
					match s {
						[] => true,
						['c', 'u', s @ ..] => {
							super::push(out, 4578);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									super::push(out, 5070);
									match s {
										[] => true,
										['-', 'n', 'o', 'g', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 5329);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'm', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3773);
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							super::push(out, 4579);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4580);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4581);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 5071);
									match s {
										[] => true,
										['q', s @ ..] => {
											super::push(out, 5330);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 5429);
													s.is_empty()
												}
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
							super::push(out, 4582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3774);
					match s {
						[] => true,
						['i', 'v', s @ ..] => {
							super::push(out, 4583);
							match s {
								[] => true,
								['-', 'e', 'n', 'i', 'p', 'é', '\u{1}', s @ ..] => {
									super::push(out, 5072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3775);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3776);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3777);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', s @ ..] => {
									super::push(out, 5073);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5331);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'a', s @ ..] => {
									super::push(out, 5074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3778);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3779);
					match s {
						[] => true,
						['u', 'o', s @ ..] => {
							super::push(out, 4585);
							match s {
								[] => true,
								['r', 'p', 'é', '-', 'é', 'b', 'é', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5075);
									s.is_empty()
								}
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
			super::push(out, 2589);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2590);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3780);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4586);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4587);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5076);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 5332);
											match s {
												[] => true,
												['e', 'h', 'c', 'u', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5430);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 5431);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2591);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3781);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ute (3 syllables).
pub(crate) fn n1476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1444);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2592);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2593);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2594);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3783);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3784);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2595);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2596);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2597);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3785);
					match s {
						[] => true,
						['m', '-', s @ ..] => {
							super::push(out, 4589);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5077);
									match s {
										[] => true,
										['s', 'o', 'p', 'é', 'd', '\u{1}', s @ ..] => {
											super::push(out, 5333);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 5334);
											match s {
												[] => true,
												['t', 'o', 'c', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 5432);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2598);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xte (3 syllables).
pub(crate) fn n1477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1445);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2600);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 3787);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yte (3 syllables).
pub(crate) fn n1478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1446);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2601);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2602);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'o', 'l', s @ ..] => {
			super::push(out, 2603);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2604);
			match s {
				[] => true,
				['p', 'o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4590);
							s.is_empty()
						}
						['r', 'é', 'n', s @ ..] => {
							super::push(out, 4591);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 4592);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 4593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'o', s @ ..] => {
			super::push(out, 2605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ète (3 syllables).
pub(crate) fn n1479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1447);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2606);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2607);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 3789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2608);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3790);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 4594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2609);
			s.is_empty()
		}
		['h', 'c', s @ ..] => {
			super::push(out, 2610);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3791);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4595);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5080);
									s.is_empty()
								}
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
			super::push(out, 2612);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3792);
					match s {
						[] => true,
						['g', 'o', s @ ..] => {
							super::push(out, 4596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3793);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2614);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2615);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3794);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4598);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5081);
									match s {
										[] => true,
										['t', 'n', 'i', '-', 'r', 'u', 'e', 't', 'i', 's', 'o', 'p', 'm', 'o', 'c', '-', 'r', 'u', 'e', 't', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 5335);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2616);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2617);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3795);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ête (3 syllables).
pub(crate) fn n1480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1448);
	match s {
		[] => true,
		['f', '-', 'e', 'l', 'b', 'u', 'o', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 2618);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 3796);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4599);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5082);
									match s {
										[] => true,
										['o', 'p', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 5336);
											s.is_empty()
										}
										['s', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 5337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'u', 'p', 'p', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4600);
							s.is_empty()
						}
						['à', '-', 'e', 't', 'ê', 't', '\u{1}', s @ ..] => {
							super::push(out, 4601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2620);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 3797);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ïte (3 syllables).
pub(crate) fn n1481(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2621);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2622);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3798);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ôte (3 syllables).
pub(crate) fn n1482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1449);
	s.is_empty()
}

/// Words ending -synagogue (3 syllables).
pub(crate) fn n5621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5433);
	s.is_empty()
}

/// Words ending -que (3 syllables).
pub(crate) fn n1489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1456);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3808);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3809);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3810);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4615);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5092);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 5341);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4616);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5093);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5342);
											s.is_empty()
										}
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
					super::push(out, 3811);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3812);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4617);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3813);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3814);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3815);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4619);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3816);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4620);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5096);
									s.is_empty()
								}
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
			super::push(out, 2633);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3817);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3818);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3819);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4622);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5097);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5343);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5098);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5344);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 5434);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3820);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4623);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4624);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5099);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 4625);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 5100);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5345);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4626);
							s.is_empty()
						}
						['t', 'é', s @ ..] => {
							super::push(out, 4627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4628);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4629);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4630);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4631);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5101);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5346);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 4632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n3953(s, out),
				['n', s @ ..] => super::t1::n3954(s, out),
				['p', s @ ..] => {
					super::push(out, 3823);
					s.is_empty()
				}
				['r', s @ ..] => super::t1::n3956(s, out),
				['s', s @ ..] => {
					super::push(out, 3824);
					match s {
						[] => true,
						['i', s @ ..] => super::t1::n4805(s, out),
						['y', s @ ..] => super::t1::n4806(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t1::n3958(s, out),
				['v', s @ ..] => super::t1::n3959(s, out),
				['x', s @ ..] => super::t1::n3960(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n2716(s, out),
		['n', s @ ..] => super::t1::n2717(s, out),
		['o', s @ ..] => super::t1::n2718(s, out),
		['r', s @ ..] => super::t1::n2719(s, out),
		['s', s @ ..] => super::t1::n2720(s, out),
		['u', s @ ..] => super::t1::n2721(s, out),
		['y', s @ ..] => super::t1::n2722(s, out),
		['è', s @ ..] => super::t1::n2723(s, out),
		['ê', s @ ..] => super::t1::n2724(s, out),
		_ => false,
	}
}

/// Words ending -ilon (3 syllables).
pub(crate) fn n2847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2761);
	match s {
		[] => true,
		['p', '-', 'u', 'a', 'e', 't', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3934);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3935);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4713);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 5159);
							s.is_empty()
						}
						['u', '\u{1}', s @ ..] => {
							super::push(out, 5160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -llon (3 syllables).
pub(crate) fn n2848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2762);
	match s {
		[] => true,
		['i', 'r', s @ ..] => {
			super::push(out, 3936);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4714);
					match s {
						[] => true,
						['-', 'e', 'p', 'u', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 5161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olon (3 syllables).
pub(crate) fn n2849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2763);
	match s {
		[] => true,
		['i', 'v', '-', 'e', 'd', '-', 'e', 't', 'ê', 't', '\u{1}', s @ ..] => {
			super::push(out, 3937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mon (3 syllables).
pub(crate) fn n1603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1569);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2764);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 2765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -non (3 syllables).
pub(crate) fn n1604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1570);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2766);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3938);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word cheval-jupon (3 syllables).
pub(crate) fn n1606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1572);
	s.is_empty()
}

/// Words ending -ron (3 syllables).
pub(crate) fn n1607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1573);
	match s {
		[] => true,
		['c', 'i', 'm', 'o', '\u{1}', s @ ..] => {
			super::push(out, 2767);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2768);
			s.is_empty()
		}
		['r', 'a', 'm', '-', 'r', 'i', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 2769);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2770);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					super::push(out, 3939);
					match s {
						[] => true,
						['-', 'e', 's', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -son (3 syllables).
pub(crate) fn n1608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1574);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2771);
			match s {
				[] => true,
				['e', 'd', 'n', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3940);
					s.is_empty()
				}
				['t', 'a', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 2772);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2773);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3943);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4716);
							match s {
								[] => true,
								['-', 'e', s @ ..] => {
									super::push(out, 5162);
									match s {
										[] => true,
										['t', 'r', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 5371);
											s.is_empty()
										}
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
					super::push(out, 3944);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2774);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2775);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ton (3 syllables).
pub(crate) fn n1609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1575);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2776);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2777);
			s.is_empty()
		}
		['u', 'o', 'm', '-', 'e', 't', 'u', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 2778);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 2779);
			match s {
				[] => true,
				['i', 'p', '-', 'a', 'r', 'é', 'm', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -von (3 syllables).
pub(crate) fn n1610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1576);
	s.is_empty()
}

/// Words ending -yon (3 syllables).
pub(crate) fn n1611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1577);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2780);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3947);
					match s {
						[] => true,
						['c', '-', 'e', 'p', 'u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 4718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2781);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -çon (3 syllables).
pub(crate) fn n1612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1578);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3948);
					match s {
						[] => true,
						['-', 's', 'n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3949);
					match s {
						[] => true,
						['-', 'c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2782);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2783);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3950);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -éon (3 syllables).
pub(crate) fn n1613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1579);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 2784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tre-nageur (3 syllables).
pub(crate) fn n5632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5444);
	s.is_empty()
}

/// Words ending -trailleur (3 syllables).
pub(crate) fn n5633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5445);
	match s {
		[] => true,
		['i', 'm', '-', s @ ..] => {
			super::push(out, 5466);
			match s {
				[] => true,
				['l', 'i', 's', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5472);
					s.is_empty()
				}
				['t', 'e', 'l', 'o', 't', 's', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5473);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word bombardier-torpilleur (3 syllables).
pub(crate) fn n5634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5446);
	s.is_empty()
}

/// Words ending -seigneur (3 syllables).
pub(crate) fn n5366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5180);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5383);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5447);
					match s {
						[] => true,
						['m', '-', 'e', 'c', 'n', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nneur (3 syllables).
pub(crate) fn n4132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3993);
	s.is_empty()
}

/// Words ending -oeur (3 syllables).
pub(crate) fn n2918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2830);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 3994);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4749);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -peur (3 syllables).
pub(crate) fn n2919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2831);
	match s {
		[] => true,
		['a', 'v', '-', 'l', 'a', 'v', 'e', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -reur (3 syllables).
pub(crate) fn n2920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2832);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 3996);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4750);
					match s {
						[] => true,
						['-', 'c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -seur (3 syllables).
pub(crate) fn n2921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2833);
	match s {
		[] => true,
		['s', 'i', 'a', s @ ..] => {
			super::push(out, 3997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teur (3 syllables).
pub(crate) fn n2922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2834);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3998);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4751);
					match s {
						[] => true,
						['u', 'g', s @ ..] => {
							super::push(out, 5182);
							match s {
								[] => true,
								['é', s @ ..] => {
									super::push(out, 5384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4752);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3999);
			match s {
				[] => true,
				['u', 'd', 'n', s @ ..] => {
					super::push(out, 4753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4000);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4754);
					match s {
						[] => true,
						['o', 'p', 'm', s @ ..] => {
							super::push(out, 5184);
							match s {
								[] => true,
								['o', 'c', '-', 'r', 'u', 'e', 't', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5385);
									s.is_empty()
								}
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
			super::push(out, 4001);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4002);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4755);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 5185);
							match s {
								[] => true,
								['c', '-', s @ ..] => {
									super::push(out, 5386);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5448);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5468);
													s.is_empty()
												}
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
							super::push(out, 5186);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4003);
			match s {
				[] => true,
				['e', 'c', 'é', 'r', '-', 'r', 'u', 'e', 't', 't', 'e', 'm', 'é', '\u{1}', s @ ..] => {
					super::push(out, 4757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4004);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4758);
					match s {
						[] => true,
						['p', '-', 'à', '-', 'e', 's', 'r', 'u', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ueur (3 syllables).
pub(crate) fn n2923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2835);
	match s {
		[] => true,
		['g', 'n', s @ ..] => {
			super::push(out, 4005);
			s.is_empty()
		}
		['q', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 4759);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4760);
					match s {
						[] => true,
						['p', '-', 'u', 'a', 'e', 't', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -veur (3 syllables).
pub(crate) fn n2924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2836);
	s.is_empty()
}

/// Words ending -our (3 syllables).
pub(crate) fn n1656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1622);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2837);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4761);
							match s {
								[] => true,
								['s', 's', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2838);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4007);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4762);
							match s {
								[] => true,
								['d', '-', 'e', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 5190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4763);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 5191);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 5388);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2839);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2840);
			match s {
				[] => true,
				['e', 'r', '-', 'r', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4008);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -œur (3 syllables).
pub(crate) fn n1657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1623);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2841);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', '-', 't', 'u', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5192);
									s.is_empty()
								}
								['v', 'è', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 5193);
									s.is_empty()
								}
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
			super::push(out, 2842);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4010);
					match s {
						[] => true,
						['e', 'l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4764);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -photographe (3 syllables).
pub(crate) fn n5643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5455);
	s.is_empty()
}

/// The word nid-de-poule (3 syllables).
pub(crate) fn n5645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5457);
	s.is_empty()
}

/// Words ending -on-citerne (3 syllables).
pub(crate) fn n5646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5458);
	s.is_empty()
}

/// Words ending -énaire (3 syllables).
pub(crate) fn n4544(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 4959);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4960);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -saire (3 syllables).
pub(crate) fn n3621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3506);
	match s {
		[] => true,
		['s', 'i', 'm', s @ ..] => {
			super::push(out, 4394);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4962);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5290);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', '-', 'c', 'u', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -taire (3 syllables).
pub(crate) fn n3622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3507);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4395);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4964);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4965);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 4966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4396);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4967);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 4969);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4397);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4398);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', 'i', s @ ..] => {
			super::push(out, 4399);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vaire (3 syllables).
pub(crate) fn n3624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3509);
	s.is_empty()
}

/// Words ending -éaire (3 syllables).
pub(crate) fn n3625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3510);
	s.is_empty()
}

/// Words ending -dire (3 syllables).
pub(crate) fn n2513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2441);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 3511);
			match s {
				[] => true,
				['ï', 'u', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oire (3 syllables).
pub(crate) fn n2514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2442);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3512);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3513);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3514);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3515);
			match s {
				[] => true,
				['é', 'm', '-', 'e', 'd', 'i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3516);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 's', s @ ..] => {
			super::push(out, 3517);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3518);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4974);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4975);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 4976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4404);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4405);
					match s {
						[] => true,
						['i', 'h', s @ ..] => {
							super::push(out, 4978);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pire (3 syllables).
pub(crate) fn n2515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2443);
	s.is_empty()
}

/// Words ending -rire (3 syllables).
pub(crate) fn n2516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2444);
	match s {
		[] => true,
		['-', 's', 'n', 'a', 's', '-', 'e', 'c', 'n', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e-barrière (3 syllables).
pub(crate) fn n5647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5459);
	match s {
		[] => true,
		['t', 's', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 5469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -e-frontière (3 syllables).
pub(crate) fn n5648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5460);
	match s {
		[] => true,
		['d', 'r', 'a', 'g', '\u{1}', s @ ..] => {
			super::push(out, 5470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tère (3 syllables).
pub(crate) fn n2572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2497);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3606);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3607);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3608);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3609);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3610);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4466);
					match s {
						[] => true,
						['g', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tre-nageuse (3 syllables).
pub(crate) fn n5649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5461);
	s.is_empty()
}

/// Words ending -yste (3 syllables).
pub(crate) fn n2667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2585);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3766);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3767);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3768);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4576);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mique (3 syllables).
pub(crate) fn n3953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3821);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4633);
			s.is_empty()
		}
		['o', 'c', s @ ..] => {
			super::push(out, 4634);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 4635);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5102);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5347);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 5435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nique (3 syllables).
pub(crate) fn n3954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3822);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4636);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4637);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4638);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5103);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 5348);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5436);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5465);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5104);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5105);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 5349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rique (3 syllables).
pub(crate) fn n3956(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4639);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4640);
			s.is_empty()
		}
		['t', 'p', s @ ..] => {
			super::push(out, 4641);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4642);
			s.is_empty()
		}
		['é', s @ ..] => {
			super::push(out, 4643);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 5107);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5108);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -isique (3 syllables).
pub(crate) fn n4805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4644);
	s.is_empty()
}

/// Words ending -ysique (3 syllables).
pub(crate) fn n4806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4645);
	match s {
		[] => true,
		['h', 'p', 'o', s @ ..] => {
			super::push(out, 5109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tique (3 syllables).
pub(crate) fn n3958(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4646);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5110);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5351);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5111);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 5352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5112);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5113);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4647);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5114);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5354);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5356);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5357);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'c', 'o', s @ ..] => {
					super::push(out, 5115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'a', 'm', s @ ..] => {
			super::push(out, 4648);
			s.is_empty()
		}
		['o', 'i', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5116);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4649);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5118);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5358);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 5439);
									s.is_empty()
								}
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
			super::push(out, 4650);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5119);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4651);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5120);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5360);
							match s {
								[] => true,
								['p', 'o', s @ ..] => {
									super::push(out, 5440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5121);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5122);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'o', 'c', s @ ..] => {
					super::push(out, 5123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5124);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4652);
			s.is_empty()
		}
		['é', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5126);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5127);
					match s {
						[] => true,
						['é', s @ ..] => {
							super::push(out, 5363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5128);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vique (3 syllables).
pub(crate) fn n3959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3825);
	s.is_empty()
}

/// Words ending -xique (3 syllables).
pub(crate) fn n3960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3826);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lque (3 syllables).
pub(crate) fn n2716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2634);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3827);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nque (3 syllables).
pub(crate) fn n2717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2635);
	match s {
		[] => true,
		['a', 'b', 'o', s @ ..] => {
			super::push(out, 3828);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 3829);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oque (3 syllables).
pub(crate) fn n2718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2636);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3830);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4655);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3831);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rque (3 syllables).
pub(crate) fn n2719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2637);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4657);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4658);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5130);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 5364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4659);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4660);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4661);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sque (3 syllables).
pub(crate) fn n2720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2638);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3833);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'r', 'o', s @ ..] => {
			super::push(out, 3834);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3835);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4664);
					s.is_empty()
				}
				['r', '-', 'l', 'a', 't', 'i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', s @ ..] => {
			super::push(out, 3836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uque (3 syllables).
pub(crate) fn n2721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2639);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3837);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3838);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yque (3 syllables).
pub(crate) fn n2722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2640);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 3840);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -èque (3 syllables).
pub(crate) fn n2723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2641);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 3841);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5131);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 5365);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -êque (3 syllables).
pub(crate) fn n2724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2642);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 3843);
			s.is_empty()
		}
		_ => false,
	}
}

