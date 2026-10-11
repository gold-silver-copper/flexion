use flexion::Path;

/// Words ending -rtować.
pub(crate) fn n2809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2607);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4768);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7945);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6757);
					match s {
						[] => true,
						['w', 'ć', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8406);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7948);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7949);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8571);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8572);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 7950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 6759);
					s.is_empty()
				}
				['ż', s @ ..] => {
					super::push(out, 6760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7951);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4769);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6761);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 6762);
					s.is_empty()
				}
				['z', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4770);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4771);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6764);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6765);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 'a', s @ ..] => {
							super::push(out, 7953);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7954);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8408);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7955);
							s.is_empty()
						}
						['m', 'i', 'a', s @ ..] => {
							super::push(out, 7956);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['k', 'e', 'y', s @ ..] => {
									super::push(out, 8410);
									s.is_empty()
								}
								['n', 'a', 'r', 't', 'e', s @ ..] => {
									super::push(out, 8411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'o', s @ ..] => {
					super::push(out, 6766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4772);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stować.
pub(crate) fn n2810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2608);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4773);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6767);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 7957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6768);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7958);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 7959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6769);
					s.is_empty()
				}
				['p', 'a', s @ ..] => {
					super::push(out, 6770);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 7960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 'n', 'o', 'k', 's', s @ ..] => {
					super::push(out, 6771);
					s.is_empty()
				}
				['w', 'e', 'd', 'z', s @ ..] => {
					super::push(out, 6772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4774);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6773);
					match s {
						[] => true,
						['i', 'n', 'a', 'm', 'a', s @ ..] => {
							super::push(out, 7961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6774);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6775);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7962);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7963);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7964);
							match s {
								[] => true,
								['r', 'p', 'a', s @ ..] => {
									super::push(out, 8412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6776);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7965);
							match s {
								[] => true,
								['r', 't', 's', s @ ..] => {
									super::push(out, 8413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 7966);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7967);
							match s {
								[] => true,
								['i', 'a', s @ ..] => {
									super::push(out, 8414);
									s.is_empty()
								}
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
			super::push(out, 4775);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4776);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 6777);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6778);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8415);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6779);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 7968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4777);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7969);
							s.is_empty()
						}
						['e', 'd', 'z', s @ ..] => {
							super::push(out, 7970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6780);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 6781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4778);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6782);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 4779);
			match s {
				[] => true,
				['z', 'c', 'o', s @ ..] => {
					super::push(out, 6784);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utować.
pub(crate) fn n2811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2609);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4780);
			match s {
				[] => true,
				['k', 'o', 'n', 'z', s @ ..] => {
					super::push(out, 6785);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4781);
			match s {
				[] => true,
				['b', 'e', 'd', 'a', s @ ..] => {
					super::push(out, 6787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4782);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6788);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8417);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6789);
					match s {
						[] => true,
						['y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4783);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6790);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6791);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8419);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4784);
			match s {
				[] => true,
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6793);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 6794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'y', 'w', '\u{1}', s @ ..] => {
			super::push(out, 4785);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4786);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7973);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4787);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7975);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7977);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8573);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8574);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7978);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8575);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 8576);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 8577);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 6796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7980);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4788);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 4789);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 4790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ytować.
pub(crate) fn n2812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2610);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 4791);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4792);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6797);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6798);
					s.is_empty()
				}
				['e', 'r', 'y', s @ ..] => {
					super::push(out, 6799);
					s.is_empty()
				}
				['i', 'l', 'a', s @ ..] => {
					super::push(out, 6800);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4793);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7982);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7983);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8578);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8579);
											match s {
												[] => true,
												['y', 'd', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9617(s, out),
														['z', s @ ..] => super::t3::n9618(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4794);
			match s {
				[] => true,
				['i', 'z', s @ ..] => {
					super::push(out, 6802);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6804);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6805);
					match s {
						[] => true,
						['w', 'z', s @ ..] => {
							super::push(out, 7984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 4795);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ztować.
pub(crate) fn n2813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2611);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4796);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6807);
					match s {
						[] => true,
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6808);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8421);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8422);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8423);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 8424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ętować.
pub(crate) fn n2814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2612);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4797);
			s.is_empty()
		}
		['z', 'c', 'e', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łtować.
pub(crate) fn n2815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2613);
	match s {
		[] => true,
		['a', 't', 'z', 's', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6810);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -izować.
pub(crate) fn n2831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2627);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4821);
			match s {
				[] => true,
				['h', 'c', 'r', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 4822);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6830);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 't', 'a', 'z', s @ ..] => {
			super::push(out, 4823);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4824);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6832);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6834);
					s.is_empty()
				}
				['r', 'a', 'n', 'a', 'z', s @ ..] => {
					super::push(out, 6835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4825);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6836);
					match s {
						[] => true,
						['b', 'r', 'e', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 7999);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 8000);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'i', 'y', s @ ..] => {
									super::push(out, 8436);
									s.is_empty()
								}
								['r', 'z', s @ ..] => {
									super::push(out, 8437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'e', 'l', 'a', s @ ..] => {
							super::push(out, 8001);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 't', 'a', 'm', 'z', s @ ..] => {
											super::push(out, 8583);
											s.is_empty()
										}
										['t', 's', 'u', 'd', 'n', 'i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 8584);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', 'y', 'r', 't', 's', s @ ..] => {
									super::push(out, 8438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', 'p', 's', 'y', s @ ..] => {
											super::push(out, 8585);
											s.is_empty()
										}
										['o', 's', 'e', 'r', 'z', s @ ..] => {
											super::push(out, 8586);
											s.is_empty()
										}
										['r', 'e', 'm', 'o', 'k', 's', s @ ..] => {
											super::push(out, 8587);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'l', 'z', s @ ..] => {
									super::push(out, 8439);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'z', s @ ..] => {
									super::push(out, 8440);
									s.is_empty()
								}
								['o', 'l', 'z', s @ ..] => {
									super::push(out, 8441);
									s.is_empty()
								}
								['y', 'd', 'a', 'r', 'z', s @ ..] => {
									super::push(out, 8442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 8002);
							match s {
								[] => true,
								['i', 'n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 8588);
											s.is_empty()
										}
										['i', 'm', 'z', s @ ..] => {
											super::push(out, 8589);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', s @ ..] => super::t3::n9363(s, out),
								['y', s @ ..] => super::t3::n9364(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t3::n8877(s, out),
						['r', s @ ..] => super::t3::n8878(s, out),
						['s', 'a', 'w', 'z', s @ ..] => {
							super::push(out, 8005);
							s.is_empty()
						}
						['t', s @ ..] => super::t3::n8880(s, out),
						['u', s @ ..] => super::t3::n8881(s, out),
						['w', s @ ..] => {
							super::push(out, 8008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n7602(s, out),
				['g', 'n', 'a', 'z', s @ ..] => super::t3::n7603(s, out),
				['i', s @ ..] => super::t3::n7604(s, out),
				['o', s @ ..] => super::t3::n7605(s, out),
				['y', s @ ..] => super::t3::n7606(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t3::n5344(s, out),
		['n', s @ ..] => super::t3::n5345(s, out),
		['u', s @ ..] => super::t3::n5346(s, out),
		['w', s @ ..] => super::t3::n5347(s, out),
		_ => false,
	}
}

/// Words ending -yzować.
pub(crate) fn n2837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2633);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', 'p', 's', s @ ..] => {
					super::push(out, 6868);
					s.is_empty()
				}
				['i', 'a', 'l', 'z', s @ ..] => {
					super::push(out, 6869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4844);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6870);
					s.is_empty()
				}
				['o', 'n', s @ ..] => {
					super::push(out, 6871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4845);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6872);
					match s {
						[] => true,
						['b', 'r', 'a', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8481);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 8049);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8483);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8484);
									match s {
										[] => true,
										['a', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 8627);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', 'l', 'u', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8485);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 8050);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8487);
									s.is_empty()
								}
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8628);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8629);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'p', 'o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8630);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8631);
											s.is_empty()
										}
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
					super::push(out, 6873);
					match s {
						[] => true,
						['p', 'u', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8051);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['k', 'a', 'r', 'a', 'h', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8632);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8633);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 8634);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', 'p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8663);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 8664);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 8635);
											match s {
												[] => true,
												['h', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9620(s, out),
														['z', s @ ..] => super::t3::n9621(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', 'p', 'm', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8636);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8637);
											s.is_empty()
										}
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
					super::push(out, 6874);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6875);
					match s {
						[] => true,
						['g', 'e', 't', 'a', 'k', 's', s @ ..] => {
							super::push(out, 8052);
							s.is_empty()
						}
						['l', 'o', 'k', 'd', s @ ..] => {
							super::push(out, 8053);
							s.is_empty()
						}
						['r', 'r', 'e', 't', 's', s @ ..] => {
							super::push(out, 8054);
							s.is_empty()
						}
						['t', 'u', 'a', 'z', s @ ..] => {
							super::push(out, 8055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 'b', 'z', '\u{1}', s @ ..] => {
									super::push(out, 8488);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 8489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'e', 'l', 'e', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8490);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 't', 'a', 'i', 'n', 'i', 'm', 'z', '\u{1}', s @ ..] => {
					super::push(out, 6876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4846);
			match s {
				[] => true,
				['a', s @ ..] => super::t3::n7651(s, out),
				['e', s @ ..] => super::t3::n7652(s, out),
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', 'r', 'a', 'n', 'z', '\u{1}', s @ ..] => {
							super::push(out, 8065);
							s.is_empty()
						}
						['n', 'p', 'i', 'h', 'a', s @ ..] => {
							super::push(out, 8066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'm', 'a', 'z', s @ ..] => {
					super::push(out, 6879);
					s.is_empty()
				}
				['u', 'l', 'o', 's', 'b', 'a', 'z', s @ ..] => super::t3::n7655(s, out),
				_ => false,
			}
		}
		['z', 's', 'a', 'f', 's', s @ ..] => super::t3::n5369(s, out),
		_ => false,
	}
}

/// The word dyskredytować.
pub(crate) fn n9617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8667);
	s.is_empty()
}

/// Words ending -zdyskredytować.
pub(crate) fn n9618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8668);
	s.is_empty()
}

/// Words ending -ormalizować.
pub(crate) fn n9363(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', 's', s @ ..] => {
			super::push(out, 8590);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8591);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 8652);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ymalizować.
pub(crate) fn n9364(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['s', 'k', 'a', 'm', 'z', s @ ..] => {
			super::push(out, 8592);
			s.is_empty()
		}
		['t', 'p', 'o', 'z', s @ ..] => {
			super::push(out, 8593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nalizować.
pub(crate) fn n8877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8003);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8594);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8595);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'y', 's', 'a', s @ ..] => {
			super::push(out, 8443);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8444);
			match s {
				[] => true,
				['f', 's', '\u{1}', s @ ..] => {
					super::push(out, 8597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'j', 'c', 'a', s @ ..] => {
			match s {
				[] => true,
				['n', 'z', s @ ..] => {
					super::push(out, 8598);
					s.is_empty()
				}
				['r', 'z', s @ ..] => {
					super::push(out, 8599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 't', 'n', 'i', 'z', s @ ..] => {
			super::push(out, 8445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ralizować.
pub(crate) fn n8878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8004);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'i', 'l', 'z', s @ ..] => {
					super::push(out, 8600);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', 'e', 'd', 'z', s @ ..] => {
			super::push(out, 8446);
			s.is_empty()
		}
		['t', 'u', 'e', 'n', 'z', s @ ..] => {
			super::push(out, 8447);
			s.is_empty()
		}
		['u', 't', 'a', 'n', 'z', s @ ..] => {
			super::push(out, 8448);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -talizować.
pub(crate) fn n8880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8006);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8449);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8653);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8604);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8450);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8606);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8655);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 8669);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ualizować.
pub(crate) fn n8881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8007);
	match s {
		[] => true,
		['t', 'k', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8657);
							s.is_empty()
						}
						['z', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 8452);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -elizować.
pub(crate) fn n7602(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8009);
			s.is_empty()
		}
		['m', 'r', 'a', 'k', 's', s @ ..] => {
			super::push(out, 8010);
			s.is_empty()
		}
		['t', 'a', 'g', 'a', 'b', 'z', '\u{1}', s @ ..] => {
			super::push(out, 8011);
			s.is_empty()
		}
		['w', 'o', 'n', 'z', s @ ..] => {
			super::push(out, 8012);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zanglizować.
pub(crate) fn n7603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6837);
	s.is_empty()
}

/// Words ending -ilizować.
pub(crate) fn n7604(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', 'z', '\u{1}', s @ ..] => {
							super::push(out, 8608);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', 'z', s @ ..] => {
					super::push(out, 8453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8013);
			s.is_empty()
		}
		['w', 'y', 'c', 'u', s @ ..] => {
			super::push(out, 8014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olizować.
pub(crate) fn n7605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6838);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8015);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8016);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylizować.
pub(crate) fn n7606(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'e', 't', 's', 'y', s @ ..] => {
			super::push(out, 8018);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8454);
					s.is_empty()
				}
				['u', 'z', s @ ..] => {
					super::push(out, 8455);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mizować.
pub(crate) fn n5344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4826);
	match s {
		[] => true,
		['a', 'n', 'y', 'd', 'z', '\u{1}', s @ ..] => {
			super::push(out, 6839);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6840);
			match s {
				[] => true,
				['d', 'a', 'k', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8019);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6841);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6842);
			match s {
				[] => true,
				['n', 'o', 't', 'u', 'a', 'z', s @ ..] => {
					super::push(out, 8021);
					s.is_empty()
				}
				['t', 'a', 'z', s @ ..] => {
					super::push(out, 8022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'y', 'r', 'o', 'g', 'l', 'a', 'z', s @ ..] => {
			super::push(out, 6843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nizować.
pub(crate) fn n5345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4827);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6844);
			match s {
				[] => true,
				['g', 'r', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8456);
							s.is_empty()
						}
						['z', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8610);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'c', 'e', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8457);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 's', 'y', 'r', 'h', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8459);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'y', 'r', 'e', 'm', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8461);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['r', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8612);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8613);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8614);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8615);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8616);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8617);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8463);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'k', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8465);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 8466);
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
				['c', 's', 'n', 'i', 'a', s @ ..] => {
					super::push(out, 8023);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6845);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8025);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8026);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8027);
					match s {
						[] => true,
						['e', 'f', 's', s @ ..] => {
							super::push(out, 8467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6846);
			match s {
				[] => true,
				['g', 'a', 't', 'n', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8029);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 8030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8468);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 8469);
							match s {
								[] => true,
								['u', 'l', 'o', 'w', 'e', 'r', 'z', s @ ..] => {
									super::push(out, 8618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 8470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8031);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8659);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8660);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8661);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 8662);
											s.is_empty()
										}
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
					super::push(out, 8032);
					match s {
						[] => true,
						['e', 'd', 'z', '\u{1}', s @ ..] => {
							super::push(out, 8471);
							s.is_empty()
						}
						['r', 'a', 'h', 'z', s @ ..] => {
							super::push(out, 8472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8033);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8034);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8473);
							match s {
								[] => true,
								['c', 'n', 'y', 's', 'z', s @ ..] => {
									super::push(out, 8619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8474);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 8475);
							s.is_empty()
						}
						['t', 'e', 'd', 'z', '\u{1}', s @ ..] => {
							super::push(out, 8476);
							s.is_empty()
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
				['d', 'o', 'm', 'z', s @ ..] => {
					super::push(out, 8035);
					s.is_empty()
				}
				['t', 's', 'e', 'w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 8036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6847);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8477);
							s.is_empty()
						}
						['o', 'k', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', 'z', s @ ..] => {
									super::push(out, 8620);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uizować.
pub(crate) fn n5346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4828);
	s.is_empty()
}

/// Words ending -wizować.
pub(crate) fn n5347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4829);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'h', 'c', 'r', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8038);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 8039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'p', 'm', 'i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8040);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 8041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 't', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8478);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 8479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8622);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8623);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'i', 'd', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8625);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 8626);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word histeryzować.
pub(crate) fn n9620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8670);
	s.is_empty()
}

/// Words ending -zhisteryzować.
pub(crate) fn n9621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8671);
	s.is_empty()
}

/// Words ending -atyzować.
pub(crate) fn n7651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6877);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 8056);
			match s {
				[] => true,
				['a', 'r', 'd', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8638);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 's', 'y', 's', 'u', s @ ..] => {
					super::push(out, 8492);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8493);
					match s {
						[] => true,
						['o', 'd', 'z', s @ ..] => {
							super::push(out, 8640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8494);
					match s {
						[] => true,
						['l', 'k', 'a', 'a', s @ ..] => {
							super::push(out, 8641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8495);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8642);
							match s {
								[] => true,
								['a', 'z', s @ ..] => {
									super::push(out, 8665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8643);
							match s {
								[] => true,
								['u', 'a', 'z', s @ ..] => {
									super::push(out, 8666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8057);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8058);
			match s {
				[] => true,
				['k', 'o', 'r', 'u', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'y', 'r', 'p', 's', s @ ..] => {
			super::push(out, 8059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -etyzować.
pub(crate) fn n7652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6878);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8060);
			s.is_empty()
		}
		['m', 'r', 'e', 'h', 'z', s @ ..] => {
			super::push(out, 8061);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8498);
					s.is_empty()
				}
				['o', 'm', 'z', s @ ..] => {
					super::push(out, 8499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8062);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 8500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8063);
			s.is_empty()
		}
		['t', 'n', 'y', 's', 'z', s @ ..] => {
			super::push(out, 8064);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zabsolutyzować.
pub(crate) fn n7655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6880);
	s.is_empty()
}

/// Words ending -sfaszyzować.
pub(crate) fn n5369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4847);
	s.is_empty()
}

