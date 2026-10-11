use flexion::Path;

/// Words ending -gern.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 949);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4608);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4609);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 4610);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4611);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 6357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4612);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4613);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6358);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6359);
									match s {
										[] => true,
										['h', 'c', 's', 'i', 'w', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6360);
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
							super::push(out, 4614);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 7341);
											match s {
												[] => true,
												['ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4615);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4616);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4617);
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
		['i', s @ ..] => {
			super::push(out, 951);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2332);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6361);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6362);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n14237(s, out),
														['s', 'g', 'n', 'a', 'w', 'z', '\u{1}', s @ ..] => super::t5::n14238(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6363);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 952);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2335);
					match s {
						[] => true,
						['r', 'p', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2336);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2337);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6365);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6366);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7342);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7343);
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', 'l', s @ ..] => {
					super::push(out, 2338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t5::n1021(s, out),
		['ä', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => super::t5::n1023(s, out),
		_ => false,
	}
}

/// Words ending -hern.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4624);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2342);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2343);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4625);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 6370);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['f', s @ ..] => {
													super::push(out, 7706);
													match s {
														[] => true,
														['u', 'a', '\u{1}', s @ ..] => super::t5::n14239(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 7707);
													match s {
														[] => true,
														['e', 'h', 'c', 's', 'i', 'w', 'z', '\u{1}', s @ ..] => super::t5::n14240(s, out),
														_ => false,
													}
												}
												['s', s @ ..] => {
													super::push(out, 7708);
													match s {
														[] => true,
														['u', 'a', '\u{1}', s @ ..] => super::t5::n14241(s, out),
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
									super::push(out, 6371);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 7345);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 7346);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 7347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 4626);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6372);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6373);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7348);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'u', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6374);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											super::push(out, 7349);
											s.is_empty()
										}
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
					super::push(out, 2344);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4628);
							match s {
								[] => true,
								['a', 'l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 4629);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6376);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2345);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4630);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6378);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6379);
									match s {
										[] => true,
										['e', 'b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 6380);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 4631);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7350);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 7351);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 7352);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 2346);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6381);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6382);
									match s {
										[] => true,
										['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
											super::push(out, 7353);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4632);
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
		_ => false,
	}
}

/// Words ending -iern.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4633);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4634);
							s.is_empty()
						}
						['n', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2348);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4636);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4637);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4638);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2349);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4639);
							s.is_empty()
						}
						['k', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4640);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4641);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kern.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4642);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 6384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4643);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4644);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 7354);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', 'c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4645);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6385);
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
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2351);
					match s {
						[] => true,
						['l', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4647);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4648);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6386);
									match s {
										[] => true,
										['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6387);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2352);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6388);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2353);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6389);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6390);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 959);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4649);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
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
							super::push(out, 4650);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 4651);
							match s {
								[] => true,
								['z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 7355);
											match s {
												[] => true,
												['z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 961);
			match s {
				[] => true,
				['e', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4652);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', 'h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lern.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2355);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6391);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6392);
									match s {
										[] => true,
										['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7356);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6393);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2356);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2357);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mern.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2358);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 4655);
							s.is_empty()
						}
						['l', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6394);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6395);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6396);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6397);
									match s {
										[] => true,
										['s', 'e', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2359);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4656);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 7357);
											match s {
												[] => true,
												['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', 'h', s @ ..] => {
							super::push(out, 4657);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 6398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2360);
					match s {
						[] => true,
						['l', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6399);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 7358);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6400);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2361);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4658);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6401);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 6402);
									match s {
										[] => true,
										['e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6403);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6404);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
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
								['\u{1}', s @ ..] => {
									super::push(out, 6405);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6406);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6407);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7359);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 2362);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6408);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nern.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['e', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 963);
			s.is_empty()
		}
		['h', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 964);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 965);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				['l', 'n', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 4659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2365);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 6409);
									match s {
										[] => true,
										['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2366);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'r', 'ä', 'g', '\u{1}', s @ ..] => {
			super::push(out, 967);
			s.is_empty()
		}
		['u', 'a', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pern.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6411);
									s.is_empty()
								}
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
			super::push(out, 970);
			match s {
				[] => true,
				['e', 'l', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 6412);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 7360);
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
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 7361);
											match s {
												[] => true,
												['c', 'a', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 7362);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['o', 'l', 'f', 'u', 'a', 'r', 'd', '\u{1}', s @ ..] => {
													super::push(out, 7709);
													s.is_empty()
												}
												['u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 7710);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'd', 'r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2373);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2374);
					match s {
						[] => true,
						['n', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6413);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['ö', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sern.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2377);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2378);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6414);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6415);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6416);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6417);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2379);
					s.is_empty()
				}
				['u', 'ä', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4662);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2380);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6418);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 977);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 978);
			match s {
				[] => true,
				['m', 'a', 'l', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4663);
							match s {
								[] => true,
								['e', 'd', 'n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tern.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 979);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 2381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 981);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 6419);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6420);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7711);
													s.is_empty()
												}
												['n', 'i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 7712);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ü', 'h', s @ ..] => {
							super::push(out, 4665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4666);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4667);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6421);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 6422);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7363);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4668);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4669);
							match s {
								[] => true,
								['r', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['o', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4670);
							match s {
								[] => true,
								['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 984);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2382);
					match s {
						[] => true,
						['w', 'h', 'c', 'a', 's', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'e', s @ ..] => {
					super::push(out, 2383);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2384);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6424);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'a', 'r', '\u{1}', s @ ..] => {
									super::push(out, 6425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'w', s @ ..] => {
					super::push(out, 2387);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2388);
					match s {
						[] => true,
						['m', 'f', s @ ..] => {
							super::push(out, 4673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4674);
							s.is_empty()
						}
						['r', 'e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2389);
					match s {
						[] => true,
						['l', 'f', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6426);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2390);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 6427);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 7364);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 7365);
											s.is_empty()
										}
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
					super::push(out, 2391);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2392);
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
							super::push(out, 4675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6428);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6429);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => super::t5::n2852(s, out),
				['ü', s @ ..] => super::t5::n2853(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t5::n1061(s, out),
		['u', s @ ..] => super::t5::n1062(s, out),
		_ => false,
	}
}

/// Words ending -uern.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4691);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4692);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4693);
							match s {
								[] => true,
								['e', 'b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4694);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4695);
							match s {
								[] => true,
								['r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2404);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6454);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6455);
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
				['k', s @ ..] => {
					super::push(out, 2405);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4696);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4697);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4698);
							s.is_empty()
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4699);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4700);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['t', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2408);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6458);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 6459);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7373);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 991);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2409);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4701);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4702);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4703);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2410);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4704);
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
				['t', s @ ..] => {
					super::push(out, 2411);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6460);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6461);
									match s {
										[] => true,
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 7374);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6462);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7375);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 7376);
											match s {
												[] => true,
												['e', 'f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6463);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', 'n', 'u', '\u{1}', s @ ..] => super::t5::n13839(s, out),
												['v', '\u{1}', s @ ..] => super::t5::n13840(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => super::t5::n9853(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => super::t5::n1065(s, out),
		_ => false,
	}
}

/// The word verpulvern.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	s.is_empty()
}

/// Words ending -wern.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', 'l', 'a', 'b', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2412);
					s.is_empty()
				}
				['p', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zern.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['t', 'e', s @ ..] => {
			super::push(out, 993);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßern.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['u', 'ä', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2414);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -heben.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['c', 'o', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4720);
							s.is_empty()
						}
						['b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 4721);
							s.is_empty()
						}
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 4723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2442);
					match s {
						[] => true,
						['p', 'm', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ieben.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4724);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4725);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4726);
							match s {
								[] => true,
								['t', 'i', 'e', 's', 'i', 'e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4727);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4728);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4729);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 4730);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4731);
							match s {
								[] => true,
								['a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6465);
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
							super::push(out, 4732);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6466);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 7381);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 7382);
											match s {
												[] => true,
												['n', 'u', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 7383);
											match s {
												[] => true,
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
									super::push(out, 6467);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4733);
							match s {
								[] => true,
								['u', 'a', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4734);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1040);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2443);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 2444);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 's', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2445);
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2446);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4736);
							s.is_empty()
						}
						['r', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -leben.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 2448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4738);
							match s {
								[] => true,
								['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1045);
			match s {
				[] => true,
				['u', 'a', 'r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4739);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2449);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2450);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2451);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['s', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2453);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4742);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7384);
											match s {
												[] => true,
												['w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 7385);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2454);
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
		['t', s @ ..] => {
			super::push(out, 1051);
			match s {
				[] => true,
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reben.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4743);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4744);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4745);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6468);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6469);
									match s {
										[] => true,
										['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4746);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											match s {
												[] => true,
												['i', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 7713);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'p', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4747);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -weben.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1053);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4748);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4749);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4750);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1055);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bleiben.
pub(crate) fn n2974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2455);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4751);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4752);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4753);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6472);
					match s {
						[] => true,
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
			super::push(out, 4754);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6473);
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
		['g', s @ ..] => {
			super::push(out, 4755);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6474);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4756);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4757);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'f', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'e', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4758);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7386);
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 7387);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4759);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6475);
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
		['t', 'r', 'o', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4760);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 6476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reiben.
pub(crate) fn n1153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1058);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2459);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2460);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4765);
					match s {
						[] => true,
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
			super::push(out, 2461);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4766);
					match s {
						[] => true,
						['u', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2462);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4767);
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
		['h', s @ ..] => {
			super::push(out, 2463);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6477);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6478);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7388);
									match s {
										[] => true,
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
							super::push(out, 6479);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', 'c', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 6480);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 7390);
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
						['k', s @ ..] => {
							super::push(out, 6481);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6482);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 7391);
									match s {
										[] => true,
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
							super::push(out, 6483);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7392);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7393);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7715);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['l', 'k', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 7716);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 6484);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7394);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 7717);
											match s {
												[] => true,
												['ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 7718);
											match s {
												[] => true,
												['e', 'i', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 7719);
											match s {
												[] => true,
												['n', 'u', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 7720);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 7395);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6485);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6486);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 7396);
									match s {
										[] => true,
										['c', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 7721);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6487);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 7397);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 6488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2464);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4768);
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
			super::push(out, 2465);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t5::n2986(s, out),
		_ => false,
	}
}

/// The word dahinsterben.
pub(crate) fn n13873(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word unterscheiden.
pub(crate) fn n13874(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -verscheiden.
pub(crate) fn n13875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7722);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hneiden.
pub(crate) fn n3072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2516);
	match s {
		[] => true,
		['c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6510);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 6511);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6512);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 6513);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6514);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6515);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6516);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6517);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 7402);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6518);
					match s {
						[] => true,
						['i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6519);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -reiden.
pub(crate) fn n1220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 2517);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -weiden.
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1111);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2518);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word aussenden.
pub(crate) fn n12604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7403);
	s.is_empty()
}

/// The word voraussenden.
pub(crate) fn n12605(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -tsenden.
pub(crate) fn n3116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2551);
	match s {
		[] => true,
		['n', 'e', '\u{1}', s @ ..] => {
			super::push(out, 4819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usenden.
pub(crate) fn n3117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2552);
	match s {
		[] => true,
		['z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wenden.
pub(crate) fn n1237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1123);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2553);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2554);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2555);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2556);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2557);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 4821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2559);
			match s {
				[] => true,
				['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2560);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					super::push(out, 4823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2561);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4825);
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
			super::push(out, 2562);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6524);
							match s {
								[] => true,
								['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2563);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2564);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					super::push(out, 4826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hnden.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -inden.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1125);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2565);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2566);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2567);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2569);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4828);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 4829);
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
					super::push(out, 2570);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['t', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 6525);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2571);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4830);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4831);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2572);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1126);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2573);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2574);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2575);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2576);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2577);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2578);
					match s {
						[] => true,
						['i', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2579);
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6526);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6527);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2580);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6528);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 6529);
									match s {
										[] => true,
										['e', 'i', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4833);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2581);
					match s {
						[] => true,
						['u', 'a', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2582);
					match s {
						[] => true,
						['h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'a', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2583);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6530);
									s.is_empty()
								}
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
			super::push(out, 1129);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2585);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2586);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6531);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6532);
									match s {
										[] => true,
										['i', 'h', 'a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 7404);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2587);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 6533);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2588);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -onden.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unden.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1132);
			match s {
				[] => true,
				['r', 'f', 'n', s @ ..] => {
					super::push(out, 2589);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1133);
			s.is_empty()
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 1134);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2590);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2591);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2592);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -änden.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['f', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2593);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ünden.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2594);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1137);
			s.is_empty()
		}
		['r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2595);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2596);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2597);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word verpfeifen.
pub(crate) fn n13884(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -reifen.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1164);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2632);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2633);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4868);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4869);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7409);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 4870);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4871);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 4872);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4873);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6541);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6542);
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
					super::push(out, 4874);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6543);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 7410);
									match s {
										[] => true,
										['ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6544);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4875);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4876);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2635);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4877);
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
		['t', s @ ..] => {
			super::push(out, 2636);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bschweifen.
pub(crate) fn n10080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6546);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sschweifen.
pub(crate) fn n10081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6547);
	match s {
		[] => true,
		['u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word zusammenschrumpfen.
pub(crate) fn n13887(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ämpfen.
pub(crate) fn n1301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1171);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4894);
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
		['k', s @ ..] => {
			super::push(out, 2659);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4895);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4896);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4897);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7413);
									match s {
										[] => true,
										['i', 'e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4898);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -opfen.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 1173);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4899);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4900);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4901);
							match s {
								[] => true,
								['d', 'a', 'r', 'e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4902);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4903);
							match s {
								[] => true,
								['c', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4904);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6553);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6554);
									match s {
										[] => true,
										['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6555);
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
							super::push(out, 4905);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6556);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4906);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4907);
							match s {
								[] => true,
								['h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6557);
									match s {
										[] => true,
										['e', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1175);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4908);
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
		['t', s @ ..] => {
			super::push(out, 1176);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2660);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4909);
							match s {
								[] => true,
								['l', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4910);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4911);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4912);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -upfen.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1177);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2662);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2663);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1178);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2664);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2665);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1179);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2666);
					match s {
						[] => true,
						['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öpfen.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4913);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4914);
							match s {
								[] => true,
								['l', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4915);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4916);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7414);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1181);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4917);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4918);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4919);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4920);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üpfen.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1183);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2667);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6558);
									match s {
										[] => true,
										['e', 't', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6559);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6560);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1185);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4921);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4922);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4923);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ürfen.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2679);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2680);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2681);
					match s {
						[] => true,
						['i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1188);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1189);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word weiterverkaufen.
pub(crate) fn n13891(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ausverkaufen.
pub(crate) fn n13892(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -laufen.
pub(crate) fn n1323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1191);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2691);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2692);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4943);
					match s {
						[] => true,
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
			super::push(out, 2693);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2694);
			match s {
				[] => true,
				['e', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4944);
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
		['g', s @ ..] => {
			super::push(out, 2695);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4945);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2696);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'e', 'b', 'r', 'o', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', 'l', 'o', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2697);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2698);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4946);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4947);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2699);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4948);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6567);
							match s {
								[] => true,
								['ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6568);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6569);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6570);
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 6571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 6572);
							match s {
								[] => true,
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
					super::push(out, 4949);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2700);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4950);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4951);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2701);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4952);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 4953);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4954);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2702);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 4955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -saufen.
pub(crate) fn n1326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1194);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2705);
			s.is_empty()
		}
		['b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2706);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2707);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2708);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ö', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 4956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 2709);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word umtaufen.
pub(crate) fn n6744(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rtaufen.
pub(crate) fn n3324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2711);
	match s {
		[] => true,
		['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hufen.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rufen.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1197);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2712);
					match s {
						[] => true,
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
			super::push(out, 1199);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4957);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4958);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1200);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2714);
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
		['h', s @ ..] => {
			super::push(out, 1201);
			match s {
				[] => true,
				['c', 'a', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1202);
			match s {
				[] => true,
				['e', 'b', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1203);
			match s {
				[] => true,
				['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1205);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2716);
					match s {
						[] => true,
						['d', 'i', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2717);
					match s {
						[] => true,
						['v', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1206);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2718);
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
		['u', s @ ..] => {
			super::push(out, 1207);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 2719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tufen.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1208);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2720);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2721);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', 'u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2722);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2723);
					match s {
						[] => true,
						['e', 't', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äufen.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1209);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2724);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2725);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2726);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2727);
					match s {
						[] => true,
						['e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', '\u{1}', s @ ..] => {
			super::push(out, 1210);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eschlagen.
pub(crate) fn n6774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4961);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fschlagen.
pub(crate) fn n6775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4962);
	match s {
		[] => true,
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hschlagen.
pub(crate) fn n6776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4963);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kschlagen.
pub(crate) fn n6777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4964);
	match s {
		[] => true,
		['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lschlagen.
pub(crate) fn n6778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4965);
	match s {
		[] => true,
		['h', 'e', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mschlagen.
pub(crate) fn n6779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4966);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nschlagen.
pub(crate) fn n6780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4967);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6576);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7418);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6577);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7419);
					match s {
						[] => true,
						['e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7420);
					match s {
						[] => true,
						['m', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6578);
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

/// Words ending -rschlagen.
pub(crate) fn n6781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4968);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6579);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'ü', '\u{1}', s @ ..] => {
					super::push(out, 7421);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7422);
					match s {
						[] => true,
						['e', 'i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 7423);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6580);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sschlagen.
pub(crate) fn n6782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4969);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 6581);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6582);
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

/// Words ending -tschlagen.
pub(crate) fn n6783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4970);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6583);
			s.is_empty()
		}
		['n', 'e', '\u{1}', s @ ..] => {
			super::push(out, 6584);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6585);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uschlagen.
pub(crate) fn n6784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4971);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 6586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -klagen.
pub(crate) fn n1355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1221);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2732);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2733);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2734);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2735);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 4972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2736);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plagen.
pub(crate) fn n1357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1223);
	s.is_empty()
}

/// Words ending -nagen.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1224);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1225);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1226);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1227);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1228);
			match s {
				[] => true,
				['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1229);
			match s {
				[] => true,
				['e', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ragen.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1230);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2738);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2739);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2740);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2741);
					match s {
						[] => true,
						['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2742);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2743);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2744);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2745);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4973);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4974);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 6587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2746);
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
		['r', s @ ..] => {
			super::push(out, 1231);
			match s {
				[] => true,
				['e', 'b', 'ü', '\u{1}', s @ ..] => {
					super::push(out, 2747);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2748);
					match s {
						[] => true,
						['p', 'm', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1232);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2749);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1233);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2750);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2751);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2752);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2753);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6588);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2754);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2755);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2756);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4975);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => super::t5::n3391(s, out),
				['n', s @ ..] => super::t5::n3392(s, out),
				['r', s @ ..] => super::t5::n3393(s, out),
				['s', s @ ..] => super::t5::n3394(s, out),
				['t', s @ ..] => super::t5::n3395(s, out),
				['u', s @ ..] => super::t5::n3396(s, out),
				['y', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sagen.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2764);
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
			super::push(out, 1235);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2765);
					match s {
						[] => true,
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
			super::push(out, 1236);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2766);
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
		['h', s @ ..] => {
			super::push(out, 1237);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2767);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1238);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1239);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1240);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2770);
					match s {
						[] => true,
						['d', 'i', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4983);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 6598);
									match s {
										[] => true,
										['e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2771);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2772);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 6599);
									s.is_empty()
								}
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
			super::push(out, 1241);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2773);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2774);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2775);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6600);
									match s {
										[] => true,
										['o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1242);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2776);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2777);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1243);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 2778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ß', 'i', 'e', 'w', '\u{1}', s @ ..] => {
			super::push(out, 1244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tagen.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1245);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1246);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wagen.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1247);
			match s {
				[] => true,
				['o', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zagen.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1248);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1249);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word übereinanderliegen.
pub(crate) fn n13902(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -terliegen.
pub(crate) fn n10271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6604);
	match s {
		[] => true,
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 7426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orliegen.
pub(crate) fn n6864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5001);
	match s {
		[] => true,
		['v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -miegen.
pub(crate) fn n1392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1254);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2796);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -riegen.
pub(crate) fn n1393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1255);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 2797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5002);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5003);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', 't', 'r', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5004);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'k', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'u', 'a', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siegen.
pub(crate) fn n1394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2798);
			s.is_empty()
		}
		['b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wiegen.
pub(crate) fn n1395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1257);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2799);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2800);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2801);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2802);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5005);
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 6606);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5006);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2803);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word beschuldigen.
pub(crate) fn n13907(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word gegenbeschuldigen.
pub(crate) fn n13908(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word anschuldigen.
pub(crate) fn n13909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7723);
	s.is_empty()
}

/// The word ankündigen.
pub(crate) fn n13910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7724);
	s.is_empty()
}

/// Words ending -rankündigen.
pub(crate) fn n13911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7725);
	match s {
		[] => true,
		['o', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eigen.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 1282);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1283);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2827);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1286);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5030);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5031);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5032);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5033);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5034);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 6617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5035);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5036);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 6618);
									match s {
										[] => true,
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
							super::push(out, 5037);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6619);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6620);
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
							super::push(out, 5038);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7430);
											s.is_empty()
										}
										['b', 'ü', '\u{1}', s @ ..] => {
											super::push(out, 7431);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'p', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5039);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 6622);
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
						['t', s @ ..] => {
							super::push(out, 5040);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6623);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2829);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6624);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6625);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 6626);
									match s {
										[] => true,
										['l', 'i', 't', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6627);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6628);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6629);
									match s {
										[] => true,
										['o', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5041);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5042);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5043);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => super::t5::n1434(s, out),
		_ => false,
	}
}

/// Words ending -illigen.
pub(crate) fn n1436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1289);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5046);
					s.is_empty()
				}
				['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5047);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5048);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5049);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nigen.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				['h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5050);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'u', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2839);
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

/// Words ending -tigen.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['f', 'ä', 'r', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2840);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['ä', 'n', s @ ..] => {
							super::push(out, 5051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1291);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['s', 'i', s @ ..] => {
							super::push(out, 5053);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6634);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 5054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1292);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5055);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5056);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5057);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5058);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5060);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'ä', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5061);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5062);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1294);
			match s {
				[] => true,
				['ä', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5063);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5064);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6638);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5065);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2842);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5066);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5067);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['m', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2843);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zigen.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['u', 'e', 'r', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2844);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word abverlangen.
pub(crate) fn n13912(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -imengen.
pub(crate) fn n3556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2866);
	match s {
		[] => true,
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nmengen.
pub(crate) fn n3557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2867);
	match s {
		[] => true,
		['i', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmengen.
pub(crate) fn n3558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2868);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nengen.
pub(crate) fn n1459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rengen.
pub(crate) fn n1460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1305);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2870);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6647);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6648);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2871);
			match s {
				[] => true,
				['s', 'n', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6649);
							s.is_empty()
						}
						['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sengen.
pub(crate) fn n1461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2872);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2873);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2874);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ingen.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 1307);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2875);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2876);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6650);
									s.is_empty()
								}
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
			super::push(out, 1309);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2877);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2878);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6651);
									s.is_empty()
								}
								['m', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5075);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5076);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5077);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5078);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 6652);
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5079);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5080);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6653);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5081);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 6654);
									match s {
										[] => true,
										['i', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 6655);
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
						['t', s @ ..] => {
							super::push(out, 5082);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', 'i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2881);
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
		['r', s @ ..] => super::t5::n1466(s, out),
		['s', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2892);
					match s {
						[] => true,
						['o', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2893);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2894);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2895);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5111);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2896);
					match s {
						[] => true,
						['i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2897);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6684);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6685);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6686);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6687);
									match s {
										[] => true,
										['i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 7443);
											s.is_empty()
										}
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
					super::push(out, 2898);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 2899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5112);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5113);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5114);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ängen.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1314);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2900);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2901);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2902);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2904);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2905);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2906);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5117);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2907);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5120);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2908);
					match s {
						[] => true,
						['s', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1315);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1316);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5121);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5122);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5123);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5124);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 5125);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5126);
							match s {
								[] => true,
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5127);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 2910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
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

/// Words ending -üngen.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1318);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 1319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -machen.
pub(crate) fn n1520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2955);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2956);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'n', 'u', 'k', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'r', 'r', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2957);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5150);
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
		['g', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5151);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 'r', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'a', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'e', 'r', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'l', 'o', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', 'u', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2959);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['a', 'l', 'k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'u', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 6696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2961);
			match s {
				[] => true,
				['o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5152);
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
		['t', s @ ..] => {
			super::push(out, 2962);
			match s {
				[] => true,
				['h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5153);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 7445);
									match s {
										[] => true,
										['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2963);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5154);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', 'r', 'u', 'k', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word aufwachen.
pub(crate) fn n7164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5156);
	s.is_empty()
}

/// The word durchwachen.
pub(crate) fn n3692(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rwachen.
pub(crate) fn n3693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2967);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -echen.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2968);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5157);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 5158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6697);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 7728);
													match s {
														[] => true,
														['h', '\u{1}', s @ ..] => super::t5::n14258(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 7729);
													match s {
														[] => true,
														['u', 'd', '\u{1}', s @ ..] => super::t5::n14259(s, out),
														_ => false,
													}
												}
												['s', s @ ..] => {
													super::push(out, 7730);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6698);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 7446);
											s.is_empty()
										}
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
					super::push(out, 2969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5159);
							s.is_empty()
						}
						['n', 'i', 'h', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2970);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5160);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5161);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6699);
									match s {
										[] => true,
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
							super::push(out, 5162);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 6700);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 6701);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5163);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 6703);
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
						['g', s @ ..] => {
							super::push(out, 5164);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6704);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5165);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5166);
							match s {
								[] => true,
								['e', 'w', 'z', 't', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5167);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 6705);
									match s {
										[] => true,
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
							super::push(out, 5168);
							match s {
								[] => true,
								['a', s @ ..] => super::t5::n10500(s, out),
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => super::t5::n10502(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t5::n7179(s, out),
						['s', s @ ..] => super::t5::n7180(s, out),
						['ß', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t5::n3698(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t5::n1527(s, out),
		['z', s @ ..] => super::t5::n1528(s, out),
		_ => false,
	}
}

/// Words ending -ichen.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1365);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2975);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2977);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6729);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6730);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7733);
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													super::push(out, 7734);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6731);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7456);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6732);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6733);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6734);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6735);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6736);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6737);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5180);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7457);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 7458);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 7735);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n14260(s, out),
														_ => false,
													}
												}
												['i', s @ ..] => {
													super::push(out, 7736);
													match s {
														[] => true,
														['e', '\u{1}', s @ ..] => super::t5::n14261(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 7459);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 7460);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2978);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => super::t5::n7196(s, out),
						['k', s @ ..] => super::t5::n7197(s, out),
						['n', s @ ..] => super::t5::n7198(s, out),
						['r', s @ ..] => super::t5::n7199(s, out),
						['s', s @ ..] => super::t5::n7200(s, out),
						['t', s @ ..] => super::t5::n7201(s, out),
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => super::t5::n3708(s, out),
				['w', s @ ..] => super::t5::n3709(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t5::n1531(s, out),
		['p', s @ ..] => super::t5::n1532(s, out),
		_ => false,
	}
}

/// Words ending -lchen.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	s.is_empty()
}

/// Words ending -nchen.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['ü', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5196);
							match s {
								[] => true,
								['e', 'b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ochen.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 1370);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1371);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2987);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2988);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2989);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2990);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5197);
							match s {
								[] => true,
								['i', 'e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2991);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2992);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 2993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2994);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2995);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2996);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2997);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1373);
			s.is_empty()
		}
		['t', 's', '\u{1}', s @ ..] => {
			super::push(out, 1374);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rchen.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1375);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1376);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1377);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5198);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5199);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5200);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5201);
							match s {
								[] => true,
								['i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5202);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 2998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2999);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3000);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -schen.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1378);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5203);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3002);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5204);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5205);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 3005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5206);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5207);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5208);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5209);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5210);
							match s {
								[] => true,
								['i', 'e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5211);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5212);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											super::push(out, 7468);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1379);
			match s {
				[] => true,
				['ü', 'h', 'f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['l', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 3007);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3008);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6744);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5214);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6746);
									match s {
										[] => true,
										['a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6747);
									match s {
										[] => true,
										['o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1381);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 3009);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3010);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6748);
									s.is_empty()
								}
								['r', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5216);
							s.is_empty()
						}
						['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3012);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5217);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5218);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => super::t5::n7249(s, out),
						['n', s @ ..] => super::t5::n7250(s, out),
						['r', s @ ..] => super::t5::n7251(s, out),
						['t', s @ ..] => super::t5::n7252(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t5::n3745(s, out),
				['t', s @ ..] => {
					super::push(out, 3015);
					s.is_empty()
				}
				['w', s @ ..] => super::t5::n3747(s, out),
				['z', s @ ..] => super::t5::n3748(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t5::n1547(s, out),
		['m', 'a', 'r', 'r', 'e', 'v', '\u{1}', s @ ..] => super::t5::n1548(s, out),
		['n', s @ ..] => super::t5::n1549(s, out),
		['o', s @ ..] => super::t5::n1550(s, out),
		['p', s @ ..] => super::t5::n1551(s, out),
		['r', s @ ..] => super::t5::n1552(s, out),
		['t', s @ ..] => super::t5::n1553(s, out),
		['u', s @ ..] => super::t5::n1554(s, out),
		['ö', s @ ..] => super::t5::n1555(s, out),
		['ü', s @ ..] => super::t5::n1556(s, out),
		_ => false,
	}
}

/// Words ending -tchen.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	s.is_empty()
}

/// Words ending -uchen.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1392);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5260);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5261);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5262);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6792);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6793);
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
							super::push(out, 5263);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5264);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3036);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3038);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5265);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6794);
									s.is_empty()
								}
								['e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6795);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7486);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ß', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5267);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5268);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 5269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5270);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5271);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6797);
									match s {
										[] => true,
										['e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 7487);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5272);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5273);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 6798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3039);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5274);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5275);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5276);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6799);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6800);
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
							super::push(out, 5277);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6801);
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											super::push(out, 7488);
											s.is_empty()
										}
										['t', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 6802);
									match s {
										[] => true,
										['v', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6803);
									match s {
										[] => true,
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6804);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1393);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t5::n3774(s, out),
				['b', s @ ..] => super::t5::n3775(s, out),
				['m', s @ ..] => super::t5::n3776(s, out),
				['r', s @ ..] => super::t5::n3777(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t5::n1559(s, out),
		['l', s @ ..] => super::t5::n1560(s, out),
		['r', s @ ..] => super::t5::n1561(s, out),
		['s', s @ ..] => super::t5::n1562(s, out),
		['t', s @ ..] => super::t5::n1563(s, out),
		['ä', 'r', '\u{1}', s @ ..] => super::t5::n1564(s, out),
		_ => false,
	}
}

/// Words ending -ächen.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['w', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3054);
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

/// The word herangehen.
pub(crate) fn n10709(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word vorangehen.
pub(crate) fn n10710(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -engehen.
pub(crate) fn n3808(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', 'c', 's', 'i', 'w', 'z', 'a', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ingehen.
pub(crate) fn n3809(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word davongehen.
pub(crate) fn n3810(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rgehen.
pub(crate) fn n1578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1408);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5285);
					s.is_empty()
				}
				['b', 'ü', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6807);
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 5286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5287);
					match s {
						[] => true,
						['i', 'e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7489);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 7739);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 5289);
					match s {
						[] => true,
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
			super::push(out, 3060);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sgehen.
pub(crate) fn n1579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1409);
	match s {
		[] => true,
		['o', 'l', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tgehen.
pub(crate) fn n1580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1410);
	match s {
		[] => true,
		['i', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3061);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 5290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3062);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 5291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ugehen.
pub(crate) fn n1581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1411);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 3063);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hehen.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1412);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iehen.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1413);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5292);
							s.is_empty()
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1414);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3065);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5294);
							match s {
								[] => true,
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
					super::push(out, 3067);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 6808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3068);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5296);
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
				['g', s @ ..] => {
					super::push(out, 3069);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5297);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3070);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'l', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3071);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3072);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3073);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5298);
							match s {
								[] => true,
								['o', 'v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7490);
											s.is_empty()
										}
										['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7491);
											s.is_empty()
										}
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
					super::push(out, 3074);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5299);
							match s {
								[] => true,
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
					super::push(out, 3075);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5300);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5301);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 6809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3076);
					match s {
						[] => true,
						['e', s @ ..] => super::t5::n7398(s, out),
						['o', s @ ..] => {
							super::push(out, 5303);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3077);
					match s {
						[] => true,
						['o', s @ ..] => super::t5::n7400(s, out),
						['u', s @ ..] => super::t5::n7401(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t5::n3835(s, out),
				['u', s @ ..] => super::t5::n3836(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lehen.
pub(crate) fn n465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1415);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3080);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3081);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3082);
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

/// Words ending -rehen.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3085);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['c', 'ü', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3090);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5309);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5310);
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
					super::push(out, 3091);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3092);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sehen.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1417);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1418);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3093);
					match s {
						[] => true,
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
			super::push(out, 1419);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1420);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3094);
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
		['g', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['e', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1422);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 6817);
									match s {
										[] => true,
										['n', 'h', 'ä', '\u{1}', s @ ..] => {
											super::push(out, 7495);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3095);
					match s {
						[] => true,
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
			super::push(out, 1424);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3098);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5311);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5312);
							match s {
								[] => true,
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
					super::push(out, 3099);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1425);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 5313);
							s.is_empty()
						}
						['h', 'r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5314);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3101);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1426);
			match s {
				[] => true,
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1427);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1428);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', 'r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tehen.
pub(crate) fn n468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3105);
					match s {
						[] => true,
						['a', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 3106);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5317);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6818);
									s.is_empty()
								}
								['r', 'e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'r', 'o', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5318);
							match s {
								[] => true,
								['a', 'r', 'e', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6819);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 6820);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5320);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3107);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3108);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3109);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3110);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3111);
					match s {
						[] => true,
						['l', 'i', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3112);
					match s {
						[] => true,
						['u', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3113);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5321);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5322);
							match s {
								[] => true,
								['f', 'f', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5323);
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
					super::push(out, 3114);
					match s {
						[] => true,
						['e', s @ ..] => super::t5::n7457(s, out),
						['o', s @ ..] => {
							super::push(out, 5325);
							match s {
								[] => true,
								['v', s @ ..] => {
									super::push(out, 6826);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ü', 'f', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3115);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5327);
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
				['t', s @ ..] => super::t5::n3878(s, out),
				['u', s @ ..] => {
					super::push(out, 3117);
					match s {
						[] => true,
						['z', s @ ..] => super::t5::n7464(s, out),
						_ => false,
					}
				}
				['y', 'e', 'b', '\u{1}', s @ ..] => super::t5::n3880(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wehen.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1430);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1431);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ecken.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 488);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3174);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3175);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3176);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3178);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 6830);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3179);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6831);
									match s {
										[] => true,
										['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3180);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1475);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1476);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5349);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5350);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6832);
									match s {
										[] => true,
										['g', 'e', 'g', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6833);
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
							super::push(out, 5351);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5352);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3183);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5353);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5354);
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
				['g', s @ ..] => {
					super::push(out, 3185);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3186);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6834);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 6835);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 6836);
									match s {
										[] => true,
										['e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6837);
									match s {
										[] => true,
										['e', 't', 'n', 'u', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6838);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3187);
					match s {
						[] => true,
						['u', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3188);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3189);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5355);
							match s {
								[] => true,
								['t', 'n', 'u', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3190);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['h', s @ ..] => super::t5::n4004(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t5::n1666(s, out),
		['p', s @ ..] => super::t5::n1667(s, out),
		['r', s @ ..] => super::t5::n1668(s, out),
		['t', s @ ..] => super::t5::n1669(s, out),
		['w', s @ ..] => super::t5::n1670(s, out),
		['z', '\u{1}', s @ ..] => {
			super::push(out, 1484);
			s.is_empty()
		}
		_ => false,
	}
}

