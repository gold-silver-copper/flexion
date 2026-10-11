use flexion::Path;

/// Words ending -ельница.
pub(crate) fn n81456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80612);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 103777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119760);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 119761);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 129705);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 135593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 119762);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 129706);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 135594);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 139027);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 129707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 119763);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 129708);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135595);
									s.is_empty()
								}
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
			super::push(out, 103778);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 119764);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 129709);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 135596);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 139028);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 141013);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 119765);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129710);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 135597);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 139029);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 141014);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 103779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119766);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 129711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 119767);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 129712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 103780);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 119768);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 129713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 103781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 119769);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 119770);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 129714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 119771);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 129716);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t54::n104877(s, out),
		['ф', s @ ..] => {
			super::push(out, 103783);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119776);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135618);
									s.is_empty()
								}
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
			super::push(out, 103784);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 103785);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 119778);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 129737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вольница.
pub(crate) fn n104890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103795);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 119790);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 119791);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 129748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 119792);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 129749);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 135624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игольница.
pub(crate) fn n121056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119793);
	s.is_empty()
}

/// Words ending -угольница.
pub(crate) fn n121057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119794);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 129750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зольница.
pub(crate) fn n104892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103797);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 119795);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кольница.
pub(crate) fn n104893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103798);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 119796);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 129751);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 135625);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 139056);
									s.is_empty()
								}
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
			super::push(out, 119797);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 119798);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 129752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 135626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 119799);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 129753);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мольница.
pub(crate) fn n104894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103799);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 119800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тольница.
pub(crate) fn n104895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103800);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 119801);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 129754);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 135627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульница.
pub(crate) fn n81459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80615);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 103801);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 119802);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 119803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 103802);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119804);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 129756);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135628);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыльница.
pub(crate) fn n81460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80616);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 103803);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 103804);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119805);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 129757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 119806);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 129758);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135629);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139058);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сопереживание.
pub(crate) fn n142516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141071);
	s.is_empty()
}

/// Words ending -ливание.
pub(crate) fn n82860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82012);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104683);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120296);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 120297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 120298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120299);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 120300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'а', s @ ..] => {
			super::push(out, 104684);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 104685);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120301);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130022);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135769);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 120302);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 120303);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130023);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 135770);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139126);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 130024);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135771);
									s.is_empty()
								}
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
			super::push(out, 104686);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120304);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 130025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 120305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130026);
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
				['п', s @ ..] => {
					super::push(out, 120306);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 120307);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135772);
									s.is_empty()
								}
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
			super::push(out, 104687);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 104688);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120308);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130028);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 130029);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 135773);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139127);
											s.is_empty()
										}
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
					super::push(out, 120309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 120310);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 130031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 104689);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 104690);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 104691);
			s.is_empty()
		}
		['ы', 'в', s @ ..] => {
			super::push(out, 104692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 104693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ривание.
pub(crate) fn n82863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82015);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104706);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120320);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 120321);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 120322);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 120323);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130033);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135774);
									s.is_empty()
								}
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
			super::push(out, 104707);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120324);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 130034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 120325);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130035);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 135775);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139128);
											s.is_empty()
										}
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
					super::push(out, 120326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 120327);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 120328);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 120329);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 130037);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135776);
									s.is_empty()
								}
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
			super::push(out, 104708);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 104709);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 120330);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 120331);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 130038);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135777);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 139129);
											s.is_empty()
										}
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
					super::push(out, 120332);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130039);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 135778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 130040);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 135779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 130041);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 135780);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 135781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'у', s @ ..] => {
					super::push(out, 120333);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 130042);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135782);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 139130);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104710);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 104711);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 120334);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 120335);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 130043);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135783);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 139131);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141072);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n143661(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 104712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чивание.
pub(crate) fn n82865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82017);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104713);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 104714);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120336);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 130044);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135784);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 139132);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141073);
													match s {
														[] => true,
														['у', s @ ..] => super::t54::n143662(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 120337);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 120338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 104715);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шивание.
pub(crate) fn n82866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82018);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104716);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120339);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 120340);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 120341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 104717);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 104718);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 104719);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120343);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 130048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 120344);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135786);
									s.is_empty()
								}
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
			super::push(out, 104720);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120345);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 120346);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 120347);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130050);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 135787);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139133);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104721);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 120348);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135788);
									s.is_empty()
								}
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
			super::push(out, 104722);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'с', s @ ..] => {
					super::push(out, 120350);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130053);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 120351);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130054);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135790);
									s.is_empty()
								}
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
			super::push(out, 104723);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120352);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 130055);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135791);
									s.is_empty()
								}
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
			super::push(out, 104724);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120353);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 120354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 104725);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ствование.
pub(crate) fn n121627(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 130063);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 130064);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 135793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 130065);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 130066);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 135794);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139134);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 135795);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 135796);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 139135);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141075);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 142212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 139136);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141076);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142213);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142857);
													match s {
														[] => true,
														['п', s @ ..] => super::t54::n144653(s, out),
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
									super::push(out, 141077);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 142214);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142858);
													s.is_empty()
												}
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
							super::push(out, 139137);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 141078);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 142215);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142859);
													match s {
														[] => true,
														['т', s @ ..] => super::t54::n144654(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 135797);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139138);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 139139);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 141079);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142216);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142860);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 142217);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 130067);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 130068);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 135798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 130069);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 139140);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 141080);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142218);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 142861);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n144655(s, out),
														_ => false,
													}
												}
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
							super::push(out, 139141);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141081);
									s.is_empty()
								}
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
			super::push(out, 130070);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 130071);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 130072);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 130073);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 135799);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139142);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 141082);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142219);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 139143);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 141083);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 139144);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 141084);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142221);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 139145);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141085);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142222);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 142862);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дование.
pub(crate) fn n82871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82023);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 104733);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120364);
					match s {
						[] => true,
						['с', 'с', s @ ..] => {
							super::push(out, 130075);
							match s {
								[] => true,
								['и', '-', s @ ..] => {
									super::push(out, 135801);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 139146);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141086);
													match s {
														[] => true,
														['м', s @ ..] => super::t54::n143674(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 120365);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130076);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 135802);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139147);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104734);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 120366);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 130077);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 135803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 120367);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 130078);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135804);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139148);
											s.is_empty()
										}
										_ => false,
									}
								}
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
					super::push(out, 120368);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 120369);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 130079);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135805);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139149);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104735);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 120370);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130080);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 135806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 120371);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 120372);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 130081);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 130082);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135808);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 141087);
													s.is_empty()
												}
												['з', s @ ..] => {
													super::push(out, 141088);
													match s {
														[] => true,
														['и', s @ ..] => super::t54::n143675(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 104736);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 120373);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 130083);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139150);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104737);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 120374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 104738);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120375);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130084);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 135810);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -образование.
pub(crate) fn n131437(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135811);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 139152);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 141089);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 142225);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 142865);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 143204);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 135812);
			match s {
				[] => true,
				['д', 'е', 'л', 'о', 'л', 'о', 'г', '\u{1}', s @ ..] => {
					super::push(out, 139153);
					s.is_empty()
				}
				['к', 'н', 'е', 'л', 'п', '\u{1}', s @ ..] => {
					super::push(out, 139154);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ользование.
pub(crate) fn n131438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130085);
	s.is_empty()
}

/// Words ending -кование.
pub(crate) fn n82873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82025);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 104742);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 104743);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 104744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лование.
pub(crate) fn n82874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82026);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104745);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120378);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 130086);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135813);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139155);
											s.is_empty()
										}
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
					super::push(out, 120379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 104746);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 120380);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 120381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 104747);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 120382);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 120383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 104748);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 120384);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 130087);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 120385);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 130088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 104749);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 120386);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 130089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 104750);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 120387);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 130090);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 135815);
									s.is_empty()
								}
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
			super::push(out, 104751);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 120388);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130091);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 135816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мование.
pub(crate) fn n82875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82027);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 104752);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120389);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 130092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 104753);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120390);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нование.
pub(crate) fn n82876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82028);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 104754);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 120391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 104755);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 120392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 104756);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120393);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130094);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135817);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 139156);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104757);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 120394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 104758);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 120395);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 130095);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135818);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139157);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141090);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 104759);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 104760);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 104761);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 104762);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пование.
pub(crate) fn n82877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82029);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 104763);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 104764);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120398);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 130096);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 135819);
									s.is_empty()
								}
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
			super::push(out, 104765);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 120399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 104766);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рование.
pub(crate) fn n82878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82030);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104767);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120400);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 120401);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 120402);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 120403);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 104768);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120404);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 130098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 120405);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 130099);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135820);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 139158);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104769);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 120406);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130100);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 135821);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139159);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 141091);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 120407);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 130101);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135822);
									s.is_empty()
								}
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
			super::push(out, 104770);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130102);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 130103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 120408);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 120409);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130104);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 120410);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 120411);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 120412);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 120413);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 120414);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 130105);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 120415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t54::n105879(s, out),
		['о', s @ ..] => {
			super::push(out, 104772);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120433);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 130160);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135868);
									s.is_empty()
								}
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
			super::push(out, 104773);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 120434);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 130161);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 130162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 120435);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 120436);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 130164);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 135869);
									s.is_empty()
								}
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
			super::push(out, 104774);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120437);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130165);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 135870);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 139196);
											s.is_empty()
										}
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
					super::push(out, 120438);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 130166);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135871);
									s.is_empty()
								}
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
			super::push(out, 104775);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 120439);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 130167);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135872);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 139197);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 135873);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139198);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 141132);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 104776);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 120440);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130168);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135874);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139199);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141133);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сование.
pub(crate) fn n82879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82031);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104777);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 104778);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 104779);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120442);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 120443);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 130170);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 135875);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 139200);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 141134);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тование.
pub(crate) fn n82880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82032);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 104780);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 120444);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 120445);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130171);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135876);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139201);
											s.is_empty()
										}
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
					super::push(out, 120446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 104781);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 104782);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 120448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 104783);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 104784);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 104785);
			s.is_empty()
		}
		['с', 'е', 'п', s @ ..] => {
			super::push(out, 104786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 104787);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 104788);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 120450);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130173);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 135877);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фование.
pub(crate) fn n82881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82033);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104789);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 104790);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 104791);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120451);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 130174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 104792);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 104793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хование.
pub(crate) fn n82882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82034);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104794);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 120452);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 130175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ман-предупреждение.
pub(crate) fn n142594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141148);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142254);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -звал-схождение.
pub(crate) fn n142595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141149);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142255);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опрос-утверждение.
pub(crate) fn n142596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141150);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 142256);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обезоружение.
pub(crate) fn n142601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141155);
	s.is_empty()
}

/// The word обескровление.
pub(crate) fn n143711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142259);
	s.is_empty()
}

/// Words ending -обезлиствление.
pub(crate) fn n143712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142260);
	s.is_empty()
}

/// Words ending -осударствление.
pub(crate) fn n143713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142261);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 142883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обезземеление.
pub(crate) fn n142606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141160);
	s.is_empty()
}

/// Words ending -ния-поселение.
pub(crate) fn n142607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141161);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142262);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142884);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алон-уведомление.
pub(crate) fn n142608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141162);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142263);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word метаосмысление.
pub(crate) fn n142609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141163);
	s.is_empty()
}

/// The word обессмысление.
pub(crate) fn n142610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141164);
	s.is_empty()
}

/// Words ending -ог-размышление.
pub(crate) fn n142611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141165);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142264);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142885);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 143211);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ан-размышление.
pub(crate) fn n142612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141166);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 142265);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142886);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 143212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овозвеличение.
pub(crate) fn n142619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141173);
	s.is_empty()
}

/// Words ending -ассосредоточение.
pub(crate) fn n142620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141174);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -существованье.
pub(crate) fn n142652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141206);
	s.is_empty()
}

/// Words ending -нованье.
pub(crate) fn n83962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83084);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 105710);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 121132);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130608);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 136122);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 139341);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 105711);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 121133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 105712);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 121134);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 130609);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 136123);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139342);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141207);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 105713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рованье.
pub(crate) fn n83964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83086);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 105715);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 121135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 105716);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 121136);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 130610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 105717);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 105718);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 121137);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'н', s @ ..] => {
					super::push(out, 121138);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 130612);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 136124);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139343);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141208);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 121139);
					s.is_empty()
				}
				['м', 'р', s @ ..] => {
					super::push(out, 121140);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130613);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 136125);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 121141);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 121142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 105719);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 121143);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 121144);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 130614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 136126);
									s.is_empty()
								}
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
			super::push(out, 105720);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 121145);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 130615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тосованье.
pub(crate) fn n106847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105721);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 121146);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 130616);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 136127);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 139345);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тованье.
pub(crate) fn n83966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83088);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 105722);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 121147);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 121148);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 130617);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 136128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 121149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осредоточенье.
pub(crate) fn n142663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141217);
	s.is_empty()
}

/// Words ending -к-сахалинский.
pub(crate) fn n142681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141235);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ск-шахтинский.
pub(crate) fn n142682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141236);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 142289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирма-посредник.
pub(crate) fn n143755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142303);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 142897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -язык-посредник.
pub(crate) fn n143756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142304);
	s.is_empty()
}

/// Words ending -мен-международник.
pub(crate) fn n142724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141278);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142308);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142901);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 143222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-междугородник.
pub(crate) fn n142725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141279);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142309);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 142902);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143223);
							match s {
								[] => true,
								['ш', '\u{1}', s @ ..] => {
									super::push(out, 143381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -летка-предшественник.
pub(crate) fn n144906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143453);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 143490);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -белок-предшественник.
pub(crate) fn n144907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143454);
	s.is_empty()
}

/// Words ending -тырехсторонник.
pub(crate) fn n142743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141297);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142324);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 142911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жник-высотник.
pub(crate) fn n142751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141305);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142329);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142914);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нсамбль-памятник.
pub(crate) fn n144682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143229);
	s.is_empty()
}

/// Words ending -лиственничник.
pub(crate) fn n142765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141319);
	s.is_empty()
}

/// Words ending -трехстрелочник.
pub(crate) fn n142767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141321);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 142341);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -т-многостаночник.
pub(crate) fn n142768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141322);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142342);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 142922);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зык-источник.
pub(crate) fn n140995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139555);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 141323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оисточник.
pub(crate) fn n123973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122696);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 131435);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136549);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 139556);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141324);
									s.is_empty()
								}
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
			super::push(out, 131436);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 136550);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 139557);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 141325);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142343);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142923);
													match s {
														[] => true,
														['и', s @ ..] => super::t54::n144684(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осточник.
pub(crate) fn n109410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108259);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 122697);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 131437);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 136551);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 139558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсточник.
pub(crate) fn n109411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108260);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 122698);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 131438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 136552);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 139559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уточник.
pub(crate) fn n87425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86501);
	s.is_empty()
}

/// Words ending -ыточник.
pub(crate) fn n87426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86502);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 108261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яточник.
pub(crate) fn n87427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86503);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 108262);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 108263);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 122699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 108264);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 122700);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 131439);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 136553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -беспредельник.
pub(crate) fn n142776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141330);
	s.is_empty()
}

/// Words ending -чельник.
pub(crate) fn n87602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86676);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 108414);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 122797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 108415);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 108416);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 122798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шельник.
pub(crate) fn n87603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86677);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 108417);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 122799);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 131478);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 136570);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 139570);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 136571);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139571);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 141331);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n143799(s, out),
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
									super::push(out, 136572);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 139572);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 141332);
													match s {
														[] => true,
														['и', s @ ..] => super::t54::n143800(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъельник.
pub(crate) fn n87604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86678);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 108418);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 122800);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 131479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -каф-холодильник.
pub(crate) fn n142779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141333);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 142349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -луг-лущильник.
pub(crate) fn n142780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141334);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 142350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -автомат-упаковщик.
pub(crate) fn n144845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143392);
	s.is_empty()
}

/// Words ending -к-постановщик.
pub(crate) fn n142820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141374);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 142388);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142947);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 143248);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143393);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 143457);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 143491);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-постановщик.
pub(crate) fn n142821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141375);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142389);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 142948);
					match s {
						[] => true,
						['и', 'р', 'и', 'д', '\u{1}', s @ ..] => {
							super::push(out, 143249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 142390);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сопостановщик.
pub(crate) fn n142822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141376);
	s.is_empty()
}

/// Words ending -декомпоновщик.
pub(crate) fn n142823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141377);
	s.is_empty()
}

/// Words ending --бомбардировщик.
pub(crate) fn n142824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141378);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142391);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 142392);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142951);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143250);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 143394);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 143458);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word моделировщик.
pub(crate) fn n142825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141379);
	s.is_empty()
}

/// Words ending -тировщик.
pub(crate) fn n109970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108810);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 123075);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 131638);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 136662);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 139635);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 141380);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142393);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144405(s, out),
														_ => false,
													}
												}
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
							super::push(out, 136663);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 139636);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141381);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142394);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 123076);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 123077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уровщик.
pub(crate) fn n88090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87151);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 108812);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 123079);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 131639);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 136664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139637);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -к-прорисовщик.
pub(crate) fn n142828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141382);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 142395);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -катер-тральщик.
pub(crate) fn n143856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142404);
	s.is_empty()
}

/// Words ending -ист-инструментальщик.
pub(crate) fn n143857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142405);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142957);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 143253);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щик-котельщик.
pub(crate) fn n142845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141399);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142406);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142958);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 143254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыба-удильщик.
pub(crate) fn n142846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141400);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 142407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -р-морозильщик.
pub(crate) fn n142847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141401);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142408);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142959);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word рокабилльщик.
pub(crate) fn n142848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141402);
	s.is_empty()
}

/// Words ending -ичество.
pub(crate) fn n91054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90094);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 110567);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 123977);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 132070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 110568);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 123978);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 132071);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 136871);
									s.is_empty()
								}
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
			super::push(out, 110569);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 123979);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 132072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 123980);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 132073);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 132074);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 136872);
									s.is_empty()
								}
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
			super::push(out, 110570);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 123981);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 123982);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132075);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 132076);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 136873);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 139751);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 141444);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n143879(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t54::n125267(s, out),
				['е', s @ ..] => super::t54::n125268(s, out),
				['ж', s @ ..] => {
					super::push(out, 123985);
					s.is_empty()
				}
				['з', s @ ..] => super::t54::n125270(s, out),
				['й', s @ ..] => {
					super::push(out, 123987);
					match s {
						[] => true,
						['о', s @ ..] => super::t54::n133445(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 123988);
					s.is_empty()
				}
				['н', s @ ..] => super::t54::n125273(s, out),
				['п', s @ ..] => super::t54::n125274(s, out),
				['р', s @ ..] => super::t54::n125275(s, out),
				['с', s @ ..] => super::t54::n125276(s, out),
				['т', s @ ..] => super::t54::n125277(s, out),
				['ч', s @ ..] => super::t54::n125278(s, out),
				['ш', s @ ..] => super::t54::n125279(s, out),
				['щ', s @ ..] => {
					super::push(out, 123996);
					s.is_empty()
				}
				['ь', s @ ..] => super::t54::n125281(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t54::n111740(s, out),
		['т', s @ ..] => super::t54::n111741(s, out),
		['щ', s @ ..] => super::t54::n111742(s, out),
		_ => false,
	}
}

/// The word имя-отчество.
pub(crate) fn n142920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141474);
	s.is_empty()
}

/// Words ending -ьтрамонтанство.
pub(crate) fn n143898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142446);
	s.is_empty()
}

/// Words ending -старейшинство.
pub(crate) fn n142926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141480);
	s.is_empty()
}

/// Words ending -стролерство.
pub(crate) fn n138375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136959);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мерство.
pub(crate) fn n91149(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 110709);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 124110);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 132182);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 136960);
									s.is_empty()
								}
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
			super::push(out, 110710);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 124111);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 132183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерство.
pub(crate) fn n91150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90185);
	match s {
		[] => true,
		['е', 'р', 'т', s @ ..] => {
			super::push(out, 110711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 124112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 110712);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 110713);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124113);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 132184);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 136961);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 139813);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 110714);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 124114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 110715);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 124115);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132185);
							match s {
								[] => true,
								['п', '-', s @ ..] => {
									super::push(out, 136962);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139814);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 141483);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n143900(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перство.
pub(crate) fn n91151(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 110716);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 124116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 110717);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 124117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 110718);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 110719);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 124119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 110720);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124120);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 132186);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 136963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рерство.
pub(crate) fn n91152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90186);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 110721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -серство.
pub(crate) fn n91153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90187);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 110722);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 110723);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 124121);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 132187);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136964);
									s.is_empty()
								}
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
			super::push(out, 110724);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 124122);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терство.
pub(crate) fn n91154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90188);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 110725);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 124123);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 132189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 110726);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 124124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 110727);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124125);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 132190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 110728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124126);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 124127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 110729);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 124128);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 132191);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 136965);
									s.is_empty()
								}
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
			super::push(out, 110730);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 124129);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 132192);
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
				['е', s @ ..] => {
					super::push(out, 124130);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 132193);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 124131);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 132194);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136967);
									s.is_empty()
								}
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
			super::push(out, 110731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124132);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 132195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 124133);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 132196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 124134);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 132197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 124135);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 132198);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 136968);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 139815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 132199);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 136969);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 139816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 132200);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 136970);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 139817);
											s.is_empty()
										}
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
					super::push(out, 124136);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132201);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 136971);
									s.is_empty()
								}
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
			super::push(out, 110732);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 124137);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ферство.
pub(crate) fn n91155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90189);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 110733);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 124138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херство.
pub(crate) fn n91156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90190);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 110734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -церство.
pub(crate) fn n91157(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 110735);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 124139);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 110736);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 124140);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -черство.
pub(crate) fn n91158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90191);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 110737);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шерство.
pub(crate) fn n91159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90192);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 110738);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 124141);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьерство.
pub(crate) fn n91160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90193);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 110739);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 124142);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 132205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вор-обязательство.
pub(crate) fn n144430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142977);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 143262);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 143398);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -копревосходительство.
pub(crate) fn n144431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142978);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 143263);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143399);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 143461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ь-сигнализатор.
pub(crate) fn n143926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142474);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142994);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 143268);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143401);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рационализатор.
pub(crate) fn n143927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142475);
	s.is_empty()
}

/// Words ending -рана-организатор.
pub(crate) fn n143928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142476);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142995);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 143402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знес-организатор.
pub(crate) fn n143929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142477);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 142996);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 143270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -систематизатор.
pub(crate) fn n143930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142478);
	s.is_empty()
}

/// Words ending -ункер-дозатор.
pub(crate) fn n142980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141534);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 142479);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ник-мультипликатор.
pub(crate) fn n144452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142999);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 143273);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 143405);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 143464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нт-провокатор.
pub(crate) fn n142987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141541);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142483);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 143000);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 143274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -циатор-терминатор.
pub(crate) fn n144859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143406);
	s.is_empty()
}

/// Words ending -юль-детонатор.
pub(crate) fn n142994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141548);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142490);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 143007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мобиль-рефрижератор.
pub(crate) fn n144918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143465);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 143492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атель-генератор.
pub(crate) fn n143947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142495);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 143011);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 143281);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 143408);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 143466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -макрогенератор.
pub(crate) fn n143948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142496);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 143012);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трана-оператор.
pub(crate) fn n143949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142497);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 143013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ехник-оператор.
pub(crate) fn n143950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142498);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 143014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -робот-оператор.
pub(crate) fn n143951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142499);
	s.is_empty()
}

/// Words ending -ик-иллюстратор.
pub(crate) fn n143004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141558);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 142500);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 143015);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -апитан-директор.
pub(crate) fn n143955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142503);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 143017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ик-архитектор.
pub(crate) fn n143012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141566);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 142504);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 143018);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орд-протектор.
pub(crate) fn n143013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141567);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 143019);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скоп-спектрометр.
pub(crate) fn n143025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141579);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142516);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 143028);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роед-типограф.
pub(crate) fn n143094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141648);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142553);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 143040);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-преподаватель.
pub(crate) fn n143116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141670);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142565);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 143048);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143294);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 143413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дообозреватель.
pub(crate) fn n143117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141671);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 142566);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 143049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тообозреватель.
pub(crate) fn n143118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141672);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 142567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 143050);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теватель.
pub(crate) fn n115712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114498);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 126277);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 133354);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137572);
							match s {
								[] => true,
								['з', 'и', 'т', 'н', 'а', 'о', s @ ..] => {
									super::push(out, 140134);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 141673);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 142568);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144504(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цеватель.
pub(crate) fn n115713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114499);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 126278);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 126279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чеватель.
pub(crate) fn n115714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126280);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 133355);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 126281);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 133356);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 137574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 126282);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 133357);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 137575);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеватель.
pub(crate) fn n115715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114501);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 126283);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 133358);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 137576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щеватель.
pub(crate) fn n115716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114502);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 126284);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133359);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 137577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иватель.
pub(crate) fn n97116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96089);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 114503);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 126285);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133360);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137578);
									s.is_empty()
								}
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
			super::push(out, 114504);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 126286);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133361);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 126287);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133362);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137580);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140136);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141674);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 114505);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 126288);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 133363);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137581);
									s.is_empty()
								}
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
			super::push(out, 114506);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 126289);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 133364);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 137582);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140137);
											s.is_empty()
										}
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
					super::push(out, 126290);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 133365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 114507);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126291);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133366);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137583);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 140138);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 141675);
													s.is_empty()
												}
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
							super::push(out, 133367);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137584);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 137585);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140139);
											s.is_empty()
										}
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
					super::push(out, 126292);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 133368);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 137586);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 140140);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 141676);
													s.is_empty()
												}
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
							super::push(out, 133369);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137587);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 140141);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141677);
													match s {
														[] => true,
														['с', s @ ..] => super::t54::n144021(s, out),
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
									super::push(out, 137588);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140142);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141678);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 126293);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 133370);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 137589);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140143);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141679);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 114508);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126294);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 133371);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 137590);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140144);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141680);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 126295);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133372);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 126296);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133373);
							match s {
								[] => true,
								['л', s @ ..] => super::t54::n139012(s, out),
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 133374);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => super::t54::n127597(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t54::n115723(s, out),
		['н', s @ ..] => super::t54::n115724(s, out),
		['р', s @ ..] => super::t54::n115725(s, out),
		['ч', s @ ..] => {
			super::push(out, 114512);
			match s {
				[] => true,
				['а', s @ ..] => super::t54::n127608(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t54::n115727(s, out),
		_ => false,
	}
}

/// Words ending -кватель.
pub(crate) fn n97117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96090);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 114514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 126311);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ователь.
pub(crate) fn n97118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96091);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 114515);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 126312);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133388);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'е', s @ ..] => {
							super::push(out, 133389);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 137606);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140152);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141684);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 137607);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 137608);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 140153);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 141685);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 114516);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 126313);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 133390);
							match s {
								[] => true,
								['с', 'с', s @ ..] => {
									super::push(out, 137609);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 140154);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 141686);
													match s {
														[] => true,
														['т', s @ ..] => super::t54::n144025(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 126314);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 133391);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137610);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 140155);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 114517);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126315);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133392);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 137611);
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141687);
													s.is_empty()
												}
												['е', 'р', 'п', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n144026(s, out),
														['о', s @ ..] => super::t54::n144027(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													match s {
														[] => true,
														['к', s @ ..] => super::t54::n144028(s, out),
														['н', s @ ..] => super::t54::n144029(s, out),
														['с', s @ ..] => super::t54::n144030(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 126316);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 133393);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137612);
									match s {
										[] => true,
										['п', s @ ..] => super::t54::n141599(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t54::n115732(s, out),
		['л', s @ ..] => super::t54::n115733(s, out),
		['н', s @ ..] => super::t54::n115734(s, out),
		['р', s @ ..] => super::t54::n115735(s, out),
		['с', s @ ..] => super::t54::n115736(s, out),
		['т', s @ ..] => super::t54::n115737(s, out),
		['х', s @ ..] => super::t54::n115738(s, out),
		_ => false,
	}
}

/// Words ending -уватель.
pub(crate) fn n97119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96092);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 114525);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 126332);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 126333);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 126334);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133413);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыватель.
pub(crate) fn n97120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96093);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 114526);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 126335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 114527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126336);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 133414);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137634);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140172);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 133415);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 137635);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140173);
											s.is_empty()
										}
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
					super::push(out, 126337);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 133416);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137636);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140174);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141696);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 114528);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126338);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133417);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137637);
									s.is_empty()
								}
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
			super::push(out, 114529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126339);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 133418);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 126340);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 133419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 114530);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 126341);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133420);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 126342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 114531);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 126343);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 133421);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137640);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140175);
											s.is_empty()
										}
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
					super::push(out, 126344);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133422);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 126345);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133423);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 126346);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 133424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 126347);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 133425);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137643);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 140176);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141697);
													s.is_empty()
												}
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
							super::push(out, 133426);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 137644);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140177);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 141698);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144035(s, out),
														_ => false,
													}
												}
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
							super::push(out, 133427);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137645);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 133428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 114532);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126348);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133429);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 140178);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141699);
													match s {
														[] => true,
														['п', s @ ..] => super::t54::n144036(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 140179);
											s.is_empty()
										}
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
					super::push(out, 126349);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133430);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137646);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 140180);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 137647);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 140181);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 114533);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126350);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 133431);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 137648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t54::n127651(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -колбодержатель.
pub(crate) fn n143153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141705);
	s.is_empty()
}

/// Words ending -укаводержатель.
pub(crate) fn n143154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141706);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142587);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word щеткодержатель.
pub(crate) fn n143155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141707);
	s.is_empty()
}

/// Words ending -предсказатель.
pub(crate) fn n143156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141708);
	s.is_empty()
}

/// Words ending -кладоискатель.
pub(crate) fn n143157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141709);
	s.is_empty()
}

/// Words ending -таллоискатель.
pub(crate) fn n143158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141710);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142588);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роискатель.
pub(crate) fn n134838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133467);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137680);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 140200);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тоискатель.
pub(crate) fn n134840(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137681);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 140201);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141712);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 142589);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140202);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141713);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 142590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 140203);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141714);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 142591);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 143062);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оскатель.
pub(crate) fn n115778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114561);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 126386);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 133469);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ускатель.
pub(crate) fn n115779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114562);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 126387);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 133470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 126388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыскатель.
pub(crate) fn n115780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114563);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 126389);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133471);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 133472);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 137682);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 140204);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141715);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 142592);
													match s {
														[] => true,
														['о', s @ ..] => super::t54::n144516(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укатель.
pub(crate) fn n97144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96117);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 114564);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкатель.
pub(crate) fn n97145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96118);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 114565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126390);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 133473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одонабиратель.
pub(crate) fn n143164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141716);
	s.is_empty()
}

/// Words ending -еронабиратель.
pub(crate) fn n143165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141717);
	s.is_empty()
}

/// Words ending -ик-собиратель.
pub(crate) fn n143166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141718);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 142593);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 143064);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енособиратель.
pub(crate) fn n143167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141719);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142594);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лаз-спасатель.
pub(crate) fn n143168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141720);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142595);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 143065);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143305);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 143417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ник-спасатель.
pub(crate) fn n143169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141721);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142596);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 143066);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 143306);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 143418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -самоспасатель.
pub(crate) fn n143170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141722);
	s.is_empty()
}

/// Words ending -р-воспитатель.
pub(crate) fn n143171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141723);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142597);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 143067);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 143307);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 143419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -испытатель.
pub(crate) fn n134865(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 137700);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 140219);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 141724);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 142598);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143068);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143308);
													match s {
														[] => true,
														['л', s @ ..] => super::t54::n144873(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 140220);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141725);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 142599);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 143069);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 143309);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n144874(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 140221);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 141726);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 142600);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143070);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 143310);
													match s {
														[] => true,
														['д', s @ ..] => super::t54::n144875(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 137701);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140222);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word летчик-истребитель.
pub(crate) fn n144947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143494);
	s.is_empty()
}

/// Words ending -ь-соправитель.
pub(crate) fn n143179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141731);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142604);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 143073);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 143313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ор-составитель.
pub(crate) fn n143180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141732);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142605);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 143074);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 143314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отовитель.
pub(crate) fn n127756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126452);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 133518);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 137718);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 140232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 137719);
					match s {
						[] => true,
						['и', '-', s @ ..] => {
							super::push(out, 140233);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 141733);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142606);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 143075);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144768(s, out),
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
									super::push(out, 141734);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 142607);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 143076);
													match s {
														[] => true,
														['я', s @ ..] => super::t54::n144769(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твитель.
pub(crate) fn n97192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96165);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 114626);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 126453);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 133519);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явитель.
pub(crate) fn n97193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96166);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 114627);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 126454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 114628);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 126455);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 114629);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 126456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 114630);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 126457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 133521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уд-охладитель.
pub(crate) fn n143183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141735);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142608);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 143077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -од-победитель.
pub(crate) fn n143184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141736);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142609);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 143078);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 143317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ен-учредитель.
pub(crate) fn n143185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141737);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142610);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 143079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одитель.
pub(crate) fn n97197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96170);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 114636);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 126467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 114637);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 126468);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 133527);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137727);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 140237);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141738);
													match s {
														[] => true,
														['х', s @ ..] => super::t54::n144064(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 126469);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 133528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 126470);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133529);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 133530);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137728);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 140238);
											s.is_empty()
										}
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
					super::push(out, 126471);
					match s {
						[] => true,
						['и', 'о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => super::t54::n139151(s, out),
								['з', s @ ..] => {
									super::push(out, 137730);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140244);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 137731);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140245);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 140246);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 141745);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144071(s, out),
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
									super::push(out, 137732);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140247);
											s.is_empty()
										}
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
					super::push(out, 126472);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 133531);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => super::t54::n134908(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t54::n127777(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 114638);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 114639);
			match s {
				[] => true,
				['с', s @ ..] => super::t54::n127778(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тор-распорядитель.
pub(crate) fn n143196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141748);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 142621);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 143088);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --размножитель.
pub(crate) fn n143197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141749);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142622);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 143089);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143323);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 143430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ь-округлитель.
pub(crate) fn n143199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141751);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142624);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word распределитель.
pub(crate) fn n143200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141752);
	s.is_empty()
}

/// Words ending --распределитель.
pub(crate) fn n143201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141753);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 142625);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 143091);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 143324);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 143431);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 143478);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 143497);
													match s {
														[] => true,
														['р', 'п', '\u{1}', s @ ..] => super::t54::n144954(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ораспределитель.
pub(crate) fn n143202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141754);
	s.is_empty()
}

/// Words ending -духоотделитель.
pub(crate) fn n143203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141755);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 142626);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 143092);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 143325);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 143432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слитель.
pub(crate) fn n97218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96191);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 114670);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 126507);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 133559);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137753);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 140258);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141756);
													s.is_empty()
												}
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
							super::push(out, 133560);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 137754);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140259);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 114671);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 126508);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хулитель.
pub(crate) fn n115893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114673);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 126510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хлитель.
pub(crate) fn n97221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96194);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 114674);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 126511);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылитель.
pub(crate) fn n97222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96195);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 114675);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 126512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 133561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 133562);
							match s {
								[] => true,
								['н', 'т', 'с', 'е', 'р', 'к', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 137755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 126513);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133563);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 140260);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141757);
													match s {
														[] => true,
														['к', s @ ..] => super::t54::n144080(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 140261);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гел-хранитель.
pub(crate) fn n143206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141758);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 142628);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 143094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -редохранитель.
pub(crate) fn n143207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141759);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 142629);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -телохранитель.
pub(crate) fn n143208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141760);
	s.is_empty()
}

/// Words ending -лнитель.
pub(crate) fn n97232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96204);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 114694);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133578);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 137765);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 140266);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 141761);
													match s {
														[] => true,
														['о', s @ ..] => super::t54::n144083(s, out),
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
									super::push(out, 137766);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140267);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 140268);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 141762);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144084(s, out),
														_ => false,
													}
												}
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
							super::push(out, 133579);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137767);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 140269);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 141763);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144085(s, out),
														_ => false,
													}
												}
												['к', s @ ..] => {
													super::push(out, 141764);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 141765);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 141766);
													match s {
														[] => true,
														['о', s @ ..] => super::t54::n144086(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 140270);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 133580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лонитель.
pub(crate) fn n115917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114697);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 126533);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 133581);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронитель.
pub(crate) fn n115918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114698);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 126534);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133582);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пнитель.
pub(crate) fn n97235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96206);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 114699);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 126535);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнитель.
pub(crate) fn n97236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96207);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 114700);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -снитель.
pub(crate) fn n97237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96208);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 126536);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133584);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 126537);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 133585);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 133586);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 133587);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 137770);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140271);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 114701);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 126538);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнитель.
pub(crate) fn n97238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96209);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 114702);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 126539);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133588);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --машиностроитель.
pub(crate) fn n143215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141767);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142634);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 143099);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 143331);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 143438);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 143483);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -афопостроитель.
pub(crate) fn n143216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141768);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142635);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 143100);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кер-накопитель.
pub(crate) fn n143217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141769);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 142636);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 143101);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 143332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омкоговоритель.
pub(crate) fn n143218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141770);
	s.is_empty()
}

/// Words ending -лаготворитель.
pub(crate) fn n143219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141771);
	s.is_empty()
}

/// Words ending -ик-смотритель.
pub(crate) fn n143220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141772);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 142637);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 143102);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ран-смеситель.
pub(crate) fn n143221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141773);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 142638);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 143103);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровосмеситель.
pub(crate) fn n143222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141774);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 142639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -носитель.
pub(crate) fn n115952(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 126563);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133616);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137790);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140287);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 141775);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142640);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144557(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 133617);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 137791);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140288);
									s.is_empty()
								}
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
			super::push(out, 126564);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 133618);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 137792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 126565);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 133619);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 137793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 126566);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133620);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 137794);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 140289);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141776);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142641);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 133621);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 133622);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137796);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 140290);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 141777);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 137797);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 137798);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 133623);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 137799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 133624);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 137800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 133625);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 137801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 133626);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 137802);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 140293);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 141778);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142642);
													s.is_empty()
												}
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
							super::push(out, 137803);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140294);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 141779);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 142643);
													match s {
														[] => true,
														['ь', s @ ..] => super::t54::n144558(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 133627);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 133628);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 137805);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 140295);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 141780);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 142644);
													s.is_empty()
												}
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
							super::push(out, 137806);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 140296);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 141781);
											s.is_empty()
										}
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
					super::push(out, 133629);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 137807);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140297);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 141782);
											s.is_empty()
										}
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
					super::push(out, 133630);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -й-искуситель.
pub(crate) fn n141743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140298);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141783);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142645);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 143106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бруситель.
pub(crate) fn n127883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126570);
	s.is_empty()
}

/// Words ending -ка-заместитель.
pub(crate) fn n143232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141784);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 142646);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 143107);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143335);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 143440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтр-осушитель.
pub(crate) fn n143233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141785);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142647);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 143108);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 143336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ливость.
pub(crate) fn n98007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96972);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115161);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 126808);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 133753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 115162);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 126809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 115163);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 126810);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 126811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 115164);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 126812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 115165);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 133754);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137870);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 137871);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 140326);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 133755);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 133756);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137873);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140327);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141799);
													s.is_empty()
												}
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
							super::push(out, 133757);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 137874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 133758);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 133759);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 133760);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 137877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 126813);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133761);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 137878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 133762);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 133763);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 133764);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 137880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 133765);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 133766);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137882);
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
						['л', s @ ..] => {
							super::push(out, 133767);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 137883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 133768);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 133769);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 126814);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 133770);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 137886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 126815);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 115166);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 126816);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133772);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137887);
									s.is_empty()
								}
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
			super::push(out, 115167);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 126817);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 133773);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137888);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 140328);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 133774);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137889);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 140329);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 133775);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 137890);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 140330);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 133776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 115168);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => super::t54::n135156(s, out),
						['м', s @ ..] => super::t54::n135157(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t54::n128137(s, out),
				['о', s @ ..] => super::t54::n128138(s, out),
				['у', s @ ..] => super::t54::n128139(s, out),
				['я', s @ ..] => super::t54::n128140(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t54::n116399(s, out),
		['к', s @ ..] => super::t54::n116400(s, out),
		['л', s @ ..] => super::t54::n116401(s, out),
		['м', s @ ..] => super::t54::n116402(s, out),
		['н', s @ ..] => super::t54::n116403(s, out),
		['п', s @ ..] => super::t54::n116404(s, out),
		['р', s @ ..] => super::t54::n116405(s, out),
		['с', s @ ..] => super::t54::n116406(s, out),
		['т', s @ ..] => super::t54::n116407(s, out),
		['ч', s @ ..] => super::t54::n116408(s, out),
		['ш', s @ ..] => super::t54::n116409(s, out),
		_ => false,
	}
}

/// Words ending -малоречивость.
pub(crate) fn n143260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141812);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 142656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олодоустойчивость.
pub(crate) fn n143261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141813);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 142657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рейтинговость.
pub(crate) fn n143262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141814);
	s.is_empty()
}

/// Words ending -хверковость.
pub(crate) fn n135228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133846);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -черковость.
pub(crate) fn n128206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126884);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133847);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сковость.
pub(crate) fn n116447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115216);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126885);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 133848);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тковость.
pub(crate) fn n116448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115217);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133849);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137940);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 133850);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137941);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 140365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чковость.
pub(crate) fn n116449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115218);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 126886);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 133851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыковость.
pub(crate) fn n116450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115219);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 126887);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 133852);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яковость.
pub(crate) fn n116451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115220);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 126888);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 133853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловость.
pub(crate) fn n98020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96984);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 115221);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 126889);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 115222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 126890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мовость.
pub(crate) fn n98021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96985);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 115223);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 126891);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 133855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новость.
pub(crate) fn n98022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96986);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 115224);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 115225);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 126892);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 115226);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 115227);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 126893);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 133857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 126894);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 133858);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 137943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повость.
pub(crate) fn n98023(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 115228);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 126895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 115229);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126896);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 133859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровость.
pub(crate) fn n98024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96987);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 115230);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 115231);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 115232);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126897);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 133860);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137944);
									s.is_empty()
								}
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
			super::push(out, 115233);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 126898);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 133861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 115234);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 126899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 115235);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -совость.
pub(crate) fn n98025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96988);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 115236);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 126900);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137945);
									s.is_empty()
								}
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
			super::push(out, 115237);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126901);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товость.
pub(crate) fn n98026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96989);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115238);
			s.is_empty()
		}
		['и', 'х', s @ ..] => {
			super::push(out, 115239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 126902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 115240);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 115241);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 126904);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133863);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 137946);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 140366);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 141815);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 115242);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цовость.
pub(crate) fn n98027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96990);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 115243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 115244);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126906);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word скрещиваемость.
pub(crate) fn n144566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143113);
	s.is_empty()
}

/// Words ending -адиовидимость.
pub(crate) fn n143274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141826);
	s.is_empty()
}

/// Words ending -ктропроводимость.
pub(crate) fn n143275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141827);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142662);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 143114);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 143339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азинедвижимость.
pub(crate) fn n143276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141828);
	s.is_empty()
}

/// Words ending -шимость.
pub(crate) fn n98084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97047);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115317);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 115318);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 126960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 115319);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 126961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 115320);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 126962);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 133918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прозападность.
pub(crate) fn n143280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141832);
	s.is_empty()
}

/// Words ending -древовидность.
pub(crate) fn n143281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141833);
	s.is_empty()
}

/// The word термоэлектропроводность.
pub(crate) fn n143282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141834);
	s.is_empty()
}

/// Words ending -выходность.
pub(crate) fn n135329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133944);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 138006);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140409);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 141835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдность.
pub(crate) fn n98108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97070);
	s.is_empty()
}

/// Words ending -удность.
pub(crate) fn n98109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97071);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 115353);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 127003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдность.
pub(crate) fn n98110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97072);
	s.is_empty()
}

/// Words ending -ядность.
pub(crate) fn n98111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97073);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 115354);
			match s {
				[] => true,
				['г', 'а', s @ ..] => {
					super::push(out, 127004);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 133945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 115355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -безденежность.
pub(crate) fn n143284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141836);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 142665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сновельможность.
pub(crate) fn n143285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141837);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 142666);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word судорожность.
pub(crate) fn n143286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141838);
	s.is_empty()
}

/// Words ending -многоатомность.
pub(crate) fn n143287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141839);
	s.is_empty()
}

/// Words ending -анность.
pub(crate) fn n98138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97099);
	match s {
		[] => true,
		['б', 'е', s @ ..] => {
			super::push(out, 115384);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 127028);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 133966);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138020);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140416);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 141840);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 115385);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127029);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 133967);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 133968);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 133969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 127030);
					match s {
						[] => true,
						['б', s @ ..] => super::t54::n135355(s, out),
						['д', s @ ..] => super::t54::n135356(s, out),
						['з', s @ ..] => super::t54::n135357(s, out),
						['к', s @ ..] => super::t54::n135358(s, out),
						['л', s @ ..] => super::t54::n135359(s, out),
						['н', s @ ..] => super::t54::n135360(s, out),
						['п', s @ ..] => super::t54::n135361(s, out),
						['р', s @ ..] => super::t54::n135362(s, out),
						['с', s @ ..] => super::t54::n135363(s, out),
						['т', s @ ..] => super::t54::n135364(s, out),
						['ф', s @ ..] => super::t54::n135365(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t54::n128357(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t54::n116620(s, out),
		['д', s @ ..] => super::t54::n116621(s, out),
		['ж', s @ ..] => super::t54::n116622(s, out),
		['з', s @ ..] => super::t54::n116623(s, out),
		['к', s @ ..] => super::t54::n116624(s, out),
		['л', s @ ..] => super::t54::n116625(s, out),
		['м', s @ ..] => super::t54::n116626(s, out),
		['н', s @ ..] => super::t54::n116627(s, out),
		['п', s @ ..] => super::t54::n116628(s, out),
		['р', s @ ..] => super::t54::n116629(s, out),
		['с', s @ ..] => super::t54::n116630(s, out),
		['т', s @ ..] => super::t54::n116631(s, out),
		['х', s @ ..] => super::t54::n116632(s, out),
		['ш', s @ ..] => super::t54::n116633(s, out),
		_ => false,
	}
}

/// Words ending -енность.
pub(crate) fn n98139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97100);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115400);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 127071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127072);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134017);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 138082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 134018);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 138083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 127073);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 134019);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138084);
									s.is_empty()
								}
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
			super::push(out, 115401);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 127074);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134020);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 138085);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140470);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 141869);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144140(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 127075);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134021);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 138086);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 140471);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 134022);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 138087);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140472);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 141870);
													match s {
														[] => true,
														['м', s @ ..] => super::t54::n144141(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 127076);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 134023);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138088);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 138089);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140473);
											match s {
												[] => true,
												['к', 'и', 'р', 'п', 'е', s @ ..] => {
													super::push(out, 141871);
													s.is_empty()
												}
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
							super::push(out, 134024);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138090);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t54::n128404(s, out),
				['ч', s @ ..] => {
					super::push(out, 127078);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134026);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 138103);
									s.is_empty()
								}
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
			super::push(out, 115402);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t54::n116638(s, out),
		['ж', s @ ..] => super::t54::n116639(s, out),
		['з', s @ ..] => super::t54::n116640(s, out),
		['л', s @ ..] => super::t54::n116641(s, out),
		['м', s @ ..] => super::t54::n116642(s, out),
		['н', s @ ..] => super::t54::n116643(s, out),
		['о', s @ ..] => super::t54::n116644(s, out),
		['п', s @ ..] => super::t54::n116645(s, out),
		['р', s @ ..] => super::t54::n116646(s, out),
		['с', s @ ..] => super::t54::n116647(s, out),
		['т', s @ ..] => {
			super::push(out, 115413);
			match s {
				[] => true,
				['е', s @ ..] => super::t54::n128479(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t54::n116649(s, out),
		['ц', s @ ..] => super::t54::n116650(s, out),
		['ч', s @ ..] => super::t54::n116651(s, out),
		['ш', s @ ..] => super::t54::n116652(s, out),
		['щ', s @ ..] => super::t54::n116653(s, out),
		_ => false,
	}
}

/// The word посюсторонность.
pub(crate) fn n143423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141972);
	s.is_empty()
}

/// Words ending -бигендерность.
pub(crate) fn n143424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141973);
	s.is_empty()
}

/// Words ending -безъядерность.
pub(crate) fn n143425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141974);
	s.is_empty()
}

/// Words ending -конвейерность.
pub(crate) fn n143426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141975);
	s.is_empty()
}

/// Words ending -отерность.
pub(crate) fn n128544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127212);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 134232);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютерность.
pub(crate) fn n128545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127213);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 134233);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 138308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уферность.
pub(crate) fn n116688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115451);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 127214);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 134234);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орность.
pub(crate) fn n98149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97110);
	match s {
		[] => true,
		['б', 'ы', s @ ..] => {
			super::push(out, 115452);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 127215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 115453);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 127216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 115454);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 115455);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 115456);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127218);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134236);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 138309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140656);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 115457);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127219);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 134237);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 127220);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 127221);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134238);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 134239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 115458);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 127222);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134240);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 138311);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140657);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 141976);
													match s {
														[] => true,
														['п', s @ ..] => super::t54::n144197(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 115459);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127223);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134241);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 138312);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140658);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134242);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 138313);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140659);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 141977);
													match s {
														[] => true,
														['о', s @ ..] => super::t54::n144198(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'в', s @ ..] => {
							super::push(out, 134243);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138314);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 140660);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141978);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n144199(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 127224);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 127225);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 134245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урность.
pub(crate) fn n98150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97111);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 115460);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 115461);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 127226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134246);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 138316);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140661);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['к', s @ ..] => {
					super::push(out, 127227);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 127228);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134247);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138317);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140662);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 115462);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 127229);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 134248);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярность.
pub(crate) fn n98152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97113);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 115463);
			match s {
				[] => true,
				['у', 'п', s @ ..] => {
					super::push(out, 127230);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лмазоносность.
pub(crate) fn n143430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141979);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ервоклассность.
pub(crate) fn n143431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141980);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 142748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усность.
pub(crate) fn n98160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97121);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 115480);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 127243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 127244);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 127245);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 134265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 115481);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 127246);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134266);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138331);
									s.is_empty()
								}
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
			super::push(out, 115482);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127247);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 127248);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 115483);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127249);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 134268);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 138332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -люсность.
pub(crate) fn n116723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115484);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 127250);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 134269);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясность.
pub(crate) fn n98162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97123);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 115485);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 115486);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 127251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбивалентность.
pub(crate) fn n143432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141981);
	s.is_empty()
}

/// Words ending -пераментность.
pub(crate) fn n143433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141982);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 142749);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 143152);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опроцентность.
pub(crate) fn n143434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141983);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142750);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отность.
pub(crate) fn n98168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97129);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 115501);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127264);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 134279);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 138343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 127265);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 127266);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138344);
									s.is_empty()
								}
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
			super::push(out, 115502);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127267);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 134281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'ш', s @ ..] => {
			super::push(out, 115503);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127268);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 134282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 115504);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 127269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 115505);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127270);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134283);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 138345);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 140672);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 141984);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 127271);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 134284);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 134285);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 138346);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140673);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 115506);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127272);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 134286);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 138347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 134287);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 138348);
									s.is_empty()
								}
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
			super::push(out, 115507);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127273);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 134288);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 138349);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140674);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141985);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 115508);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 127274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', 'о', 'р', s @ ..] => {
			super::push(out, 115509);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 127275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -портность.
pub(crate) fn n128611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127276);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 134290);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138350);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 140675);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 141986);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142751);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 143154);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стность.
pub(crate) fn n98170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97131);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115511);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 127278);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 134291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 134292);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138351);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140676);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141987);
													s.is_empty()
												}
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
							super::push(out, 134293);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 138352);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 140677);
											match s {
												[] => true,
												['р', 'п', 'с', s @ ..] => {
													super::push(out, 141988);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 127279);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 134294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 115512);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 127280);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 134295);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138353);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 140678);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 138354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 134296);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 138355);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140679);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 141989);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 127281);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134297);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 138356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 134298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 127282);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 134299);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 127283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 115513);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 127284);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 115514);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 127285);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 127286);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 127287);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 127288);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 127289);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134302);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 138358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 134303);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 138359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 134304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 127290);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 127291);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 127292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 115515);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127293);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 134305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 115516);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 115517);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 127294);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытность.
pub(crate) fn n98172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97133);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 115518);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127295);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 127296);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 134307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 115519);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127297);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 134308);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 138360);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 140680);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 134309);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 138361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 134310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 115520);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 127298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 115521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятность.
pub(crate) fn n98174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97135);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 115522);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 127299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 115523);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127300);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 134311);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138362);
									s.is_empty()
								}
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
			super::push(out, 115524);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 115525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьтизадачность.
pub(crate) fn n143441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141990);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142752);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 143155);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 143355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word призрачность.
pub(crate) fn n143442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141991);
	s.is_empty()
}

/// The word сумеречность.
pub(crate) fn n143443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141992);
	s.is_empty()
}

/// The word игрушечность.
pub(crate) fn n143444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141993);
	s.is_empty()
}

/// Words ending -дничность.
pub(crate) fn n128649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127312);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 134322);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138373);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 140686);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141994);
									s.is_empty()
								}
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
			super::push(out, 134323);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 138374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 140687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фичность.
pub(crate) fn n116783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115540);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 127314);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 134324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лчность.
pub(crate) fn n98180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97141);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 115541);
			match s {
				[] => true,
				['м', 'у', 'з', s @ ..] => {
					super::push(out, 127315);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134325);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 138375);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140688);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очность.
pub(crate) fn n98181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115542);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 127316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 115543);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127317);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 134326);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138376);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 140689);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 141995);
													match s {
														[] => true,
														['б', s @ ..] => super::t54::n144206(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 127318);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 134327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138377);
									s.is_empty()
								}
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
			super::push(out, 115544);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127319);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 127320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 115545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127321);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 127322);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 115546);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 115547);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127323);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 127324);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 134329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 134330);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 138378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 134331);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138379);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 140690);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 115548);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127325);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134332);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 138380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 127326);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 134333);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 138381);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140691);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141996);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 115549);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127327);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 127328);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 115550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134335);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 134336);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 134337);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138382);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 140692);
											s.is_empty()
										}
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
					super::push(out, 127330);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 134338);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 138383);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140693);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141997);
													s.is_empty()
												}
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
							super::push(out, 134339);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138384);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 140694);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141998);
													match s {
														[] => true,
														['з', s @ ..] => super::t54::n144207(s, out),
														_ => false,
													}
												}
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
							super::push(out, 134340);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 138385);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140695);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141999);
													s.is_empty()
												}
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
							super::push(out, 134341);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 127331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134342);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 138387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 134343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 127332);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127333);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134345);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t54::n128672(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t54::n116795(s, out),
		_ => false,
	}
}

/// Words ending -учность.
pub(crate) fn n98182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97143);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 115552);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 127340);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138395);
									s.is_empty()
								}
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
			super::push(out, 115553);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ычность.
pub(crate) fn n98183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97144);
	s.is_empty()
}

/// Words ending -взбалмошность.
pub(crate) fn n143452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142001);
	s.is_empty()
}

/// Words ending -проигрышность.
pub(crate) fn n143453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142002);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142755);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 143157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нергомощность.
pub(crate) fn n143454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142003);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 142756);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -беспомощность.
pub(crate) fn n143455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142004);
	s.is_empty()
}

/// Words ending -пиальность.
pub(crate) fn n135770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134368);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 138404);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 140705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мальность.
pub(crate) fn n128693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127354);
	s.is_empty()
}

/// Words ending -нальность.
pub(crate) fn n128694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127355);
	match s {
		[] => true,
		['и', 'г', s @ ..] => {
			super::push(out, 134370);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 138405);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 140706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'и', 'ц', 'а', 'р', s @ ..] => {
			super::push(out, 134371);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ральность.
pub(crate) fn n128695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127356);
	s.is_empty()
}

/// Words ending -сальность.
pub(crate) fn n128696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127357);
	s.is_empty()
}

/// Words ending -тальность.
pub(crate) fn n128697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127358);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 134372);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 134373);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 138406);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							super::push(out, 140707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уальность.
pub(crate) fn n128698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127359);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 134374);
			match s {
				[] => true,
				['к', 'е', 'с', 'о', s @ ..] => {
					super::push(out, 138407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 134375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ельность.
pub(crate) fn n116813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115569);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 127360);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134376);
					match s {
						[] => true,
						['т', 'н', s @ ..] => {
							super::push(out, 138408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 127361);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134377);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 134378);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138410);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 134379);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138411);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 134380);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 127362);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 134381);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138413);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140710);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142006);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142757);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127363);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 134382);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138414);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140711);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142007);
											s.is_empty()
										}
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
					super::push(out, 134383);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138415);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140712);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 142008);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142758);
													match s {
														[] => true,
														['к', s @ ..] => super::t54::n144611(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127364);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134384);
					match s {
						[] => true,
						['в', s @ ..] => super::t54::n139851(s, out),
						['г', s @ ..] => {
							super::push(out, 138417);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140714);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 140715);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 142014);
											match s {
												[] => true,
												['т', s @ ..] => super::t54::n144217(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t54::n139853(s, out),
						['ж', s @ ..] => super::t54::n139854(s, out),
						['з', s @ ..] => super::t54::n139855(s, out),
						['к', s @ ..] => super::t54::n139856(s, out),
						['л', s @ ..] => super::t54::n139857(s, out),
						['м', s @ ..] => super::t54::n139858(s, out),
						['н', s @ ..] => super::t54::n139859(s, out),
						['р', s @ ..] => super::t54::n139860(s, out),
						['т', s @ ..] => super::t54::n139861(s, out),
						['ц', s @ ..] => super::t54::n139862(s, out),
						['ч', s @ ..] => super::t54::n139863(s, out),
						['щ', s @ ..] => super::t54::n139864(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t54::n135787(s, out),
				['о', s @ ..] => super::t54::n135788(s, out),
				['р', s @ ..] => super::t54::n135789(s, out),
				['с', s @ ..] => super::t54::n135790(s, out),
				['я', s @ ..] => super::t54::n135791(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t54::n128704(s, out),
		_ => false,
	}
}

/// Words ending -ильность.
pub(crate) fn n116814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115570);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 127366);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134393);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 138446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 134394);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 134395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 127367);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134396);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138448);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140731);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142025);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142768);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127368);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 127369);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 127370);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 127371);
			match s {
				[] => true,
				['о', 'р', s @ ..] => {
					super::push(out, 134398);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138449);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 138450);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 138451);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140732);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 142026);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 140733);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 142027);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лльность.
pub(crate) fn n116815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115571);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127372);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольность.
pub(crate) fn n116816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115572);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 127373);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 127374);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 127375);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134400);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138452);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140734);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142028);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142769);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n144620(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульность.
pub(crate) fn n116817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115573);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 127376);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134401);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 138453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 127377);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 134403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 134404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыльность.
pub(crate) fn n116818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115574);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 127378);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 134405);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 127379);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яльность.
pub(crate) fn n116819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115575);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 127380);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 134406);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -адистость.
pub(crate) fn n128745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127405);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 134421);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 138466);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еистость.
pub(crate) fn n116846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115601);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 127406);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 134422);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жистость.
pub(crate) fn n116847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115602);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127407);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 127408);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зистость.
pub(crate) fn n116848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115603);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127409);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134423);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138467);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140740);
									s.is_empty()
								}
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
			super::push(out, 127410);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 127411);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 127412);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -листость.
pub(crate) fn n116849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115604);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127413);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134424);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138469);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 140741);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142031);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 138470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 127414);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 127415);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 134425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 127416);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 134426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 127417);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134427);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 134428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 127418);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134430);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 138471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'о', s @ ..] => {
					super::push(out, 134431);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 138472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 134432);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 127419);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 134433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 127420);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 127421);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134435);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138474);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140742);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142032);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142772);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мистость.
pub(crate) fn n116850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115605);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127422);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 127423);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134436);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 138475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нистость.
pub(crate) fn n116851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115606);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127424);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 134437);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 138476);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 140743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 134438);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134439);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 134440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 127425);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 134441);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 138479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 134442);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 134443);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 134444);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 127426);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134445);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 138483);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140744);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 142033);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127427);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 127428);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 134447);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 138484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 127429);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оистость.
pub(crate) fn n116852(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 127430);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 134449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 127431);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пистость.
pub(crate) fn n116853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115607);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ристость.
pub(crate) fn n116854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115608);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127433);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 134451);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138485);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140745);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142034);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127434);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134452);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 127435);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 134453);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 134454);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 127436);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 127437);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134456);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 138488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 127438);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134457);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 134458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 134459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 127439);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -систость.
pub(crate) fn n116855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115609);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134460);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 138490);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 134461);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 134462);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 138491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 134463);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134464);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138493);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 140747);
									s.is_empty()
								}
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
			super::push(out, 127440);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134465);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тистость.
pub(crate) fn n116856(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134466);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138494);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 134467);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138495);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140749);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134468);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 138496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 134469);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138497);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140750);
									s.is_empty()
								}
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
			super::push(out, 127441);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138498);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 138499);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134470);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 127442);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чистость.
pub(crate) fn n116857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115610);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шистость.
pub(crate) fn n116858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115611);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127444);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 134472);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138501);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140752);
									s.is_empty()
								}
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
			super::push(out, 127445);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 127446);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 127447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -остость.
pub(crate) fn n98218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97178);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 115612);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -устость.
pub(crate) fn n98219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97179);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 115613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ястость.
pub(crate) fn n98220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97180);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 115614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -астрополяриметрия.
pub(crate) fn n144239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142786);
	s.is_empty()
}

/// Words ending -омплексонометрия.
pub(crate) fn n144240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142787);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 143172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ферометрия.
pub(crate) fn n136149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134744);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138630);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140819);
					match s {
						[] => true,
						['т', 'н', 'и', 'о', s @ ..] => {
							super::push(out, 142069);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 142788);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 143173);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трометрия.
pub(crate) fn n129278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127928);
	match s {
		[] => true,
		['к', 'е', 'п', 'с', 'а', s @ ..] => {
			super::push(out, 134746);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138631);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 140820);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142070);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 142789);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 143174);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144817(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 134747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урометрия.
pub(crate) fn n129279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127929);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 134748);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138632);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140821);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 142071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хрометрия.
pub(crate) fn n129280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127930);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 134749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эрометрия.
pub(crate) fn n129281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127931);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 134750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сометрия.
pub(crate) fn n117633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116379);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 127932);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134751);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 138633);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140822);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142072);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142790);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тометрия.
pub(crate) fn n117634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116380);
	match s {
		[] => true,
		['а', 'м', s @ ..] => {
			super::push(out, 127933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134752);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138634);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140823);
									s.is_empty()
								}
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
			super::push(out, 127934);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 127935);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 127936);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134755);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 127937);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 134756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138637);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 138638);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 140824);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142073);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 138639);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140825);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 142074);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127938);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хометрия.
pub(crate) fn n117635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116381);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 127939);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134758);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 138641);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

