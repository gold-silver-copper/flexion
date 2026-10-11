use flexion::Path;

/// Words ending -убиться.
pub(crate) fn n13466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10866);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18840);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 18841);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26395);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26396);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26397);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32827);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26398);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26400);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32828);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									super::push(out, 37373);
									s.is_empty()
								}
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
			super::push(out, 18842);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26401);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26402);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26403);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26404);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32829);
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
		['к', s @ ..] => {
			super::push(out, 18843);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26405);
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
		['л', s @ ..] => {
			super::push(out, 18844);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26406);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32830);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32831);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32832);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26408);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32834);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37374);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 37375);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40390);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18845);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32835);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18846);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26410);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26411);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 26412);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32836);
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
				['в', s @ ..] => {
					super::push(out, 26413);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 26414);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32837);
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
				['д', s @ ..] => {
					super::push(out, 26415);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26416);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26417);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26418);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26419);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32838);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32839);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37376);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 37377);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32840);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => super::t27::n42726(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбиться.
pub(crate) fn n13467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10867);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18847);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26424);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26425);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26426);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26427);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32842);
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
		['з', s @ ..] => {
			super::push(out, 18849);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 18850);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18851);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбиться.
pub(crate) fn n13468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10868);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26428);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26429);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябиться.
pub(crate) fn n13469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10869);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18853);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 26430);
					match s {
						[] => true,
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18854);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26431);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26432);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26433);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -абавиться.
pub(crate) fn n42749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26435);
	match s {
		[] => true,
		['з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37379);
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -збавиться.
pub(crate) fn n42751(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -прибавиться.
pub(crate) fn n42752(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обавиться.
pub(crate) fn n42753(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сбавиться.
pub(crate) fn n42754(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -убавиться.
pub(crate) fn n42755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26436);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вавиться.
pub(crate) fn n26684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18856);
	match s {
		[] => true,
		['о', 'р', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32843);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32844);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32846);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -давиться.
pub(crate) fn n26685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18857);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26437);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26438);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32848);
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
		['в', s @ ..] => {
			super::push(out, 26439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26440);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 26441);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26442);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26443);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32849);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32850);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26445);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 26446);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32851);
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

/// The word завиться.
pub(crate) fn n42767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26447);
	s.is_empty()
}

/// Words ending -кавиться.
pub(crate) fn n26687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18859);
	match s {
		[] => true,
		['е', 'р', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32852);
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'л', 'з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лавиться.
pub(crate) fn n26688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18860);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 26448);
			match s {
				[] => true,
				['а', 'з', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
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
				['\u{1}', s @ ..] => {
					super::push(out, 32853);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32854);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 32857);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37382);
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
		['с', s @ ..] => {
			super::push(out, 26449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32858);
					s.is_empty()
				}
				['е', 'щ', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37383);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40391);
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -навиться.
pub(crate) fn n26689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18861);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -равиться.
pub(crate) fn n26690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18862);
	match s {
		[] => true,
		['д', 'з', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 26451);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32860);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32861);
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32862);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37384);
							match s {
								[] => true,
								['п', 'у', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 26452);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32863);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 32865);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37385);
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
		['т', s @ ..] => {
			super::push(out, 26453);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32866);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32867);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 37386);
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32868);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 32870);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37387);
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
		['у', s @ ..] => {
			super::push(out, 26454);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37388);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37389);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37390);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37391);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -савиться.
pub(crate) fn n26691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18863);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 26455);
			match s {
				[] => true,
				['н', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тавиться.
pub(crate) fn n26692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18864);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32871);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32872);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 32873);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32874);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 37392);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40392);
									s.is_empty()
								}
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 32875);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40393);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40394);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42078);
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

/// Words ending -чавиться.
pub(crate) fn n26693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18865);
	match s {
		[] => true,
		['р', 'у', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32876);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32877);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шавиться.
pub(crate) fn n26694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18866);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26456);
			match s {
				[] => true,
				['е', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бвиться.
pub(crate) fn n13472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10872);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18867);
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

/// Words ending -ввиться.
pub(crate) fn n13473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10873);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -двиться.
pub(crate) fn n13474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10874);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18868);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26457);
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

/// Words ending -евиться.
pub(crate) fn n13475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10875);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 18869);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32878);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32879);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18870);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26458);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32880);
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
		['ш', s @ ..] => {
			super::push(out, 18871);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26459);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'д', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32881);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 37394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -звиться.
pub(crate) fn n13476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10876);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18872);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32882);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18873);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18874);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32883);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32884);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32885);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37395);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37396);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37397);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40395);
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
		['и', s @ ..] => {
			super::push(out, 18875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 18876);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26463);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 26464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 26465);
					match s {
						[] => true,
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивиться.
pub(crate) fn n13477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10877);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18877);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26466);
					match s {
						[] => true,
						['е', 'ч', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18878);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26467);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 32888);
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
				['о', s @ ..] => {
					super::push(out, 26468);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26470);
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
		['ж', s @ ..] => {
			super::push(out, 18879);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26471);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18880);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26473);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32889);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40396);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 40397);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26474);
					match s {
						[] => true,
						['с', 'а', 'ч', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37398);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40398);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 40399);
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
		['р', s @ ..] => {
			super::push(out, 18881);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32890);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32891);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32892);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 32893);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37399);
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
				['п', s @ ..] => {
					super::push(out, 26476);
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
		['с', s @ ..] => {
			super::push(out, 18882);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26477);
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37400);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37401);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26478);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37402);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37403);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18883);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26479);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37404);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37405);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37406);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40400);
											match s {
												[] => true,
												['в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => super::t27::n70954(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t27::n42816(s, out),
				['с', s @ ..] => super::t27::n42817(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лвиться.
pub(crate) fn n13478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10878);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18884);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32895);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32896);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 32897);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37409);
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
						['е', s @ ..] => {
							super::push(out, 32898);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32899);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37410);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40401);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 37411);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 40402);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 32900);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 32902);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37412);
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

/// Words ending -овиться.
pub(crate) fn n13479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10879);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18885);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 18886);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18887);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26483);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32903);
							match s {
								[] => true,
								['т', 'с', 'е', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26484);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32904);
							match s {
								[] => true,
								['ц', 'о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26485);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26486);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 26487);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32905);
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
				['д', s @ ..] => {
					super::push(out, 26488);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32906);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26489);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32907);
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
				['о', s @ ..] => {
					super::push(out, 26490);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32908);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32909);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37413);
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
				['с', s @ ..] => {
					super::push(out, 26491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32910);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32911);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37414);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 40403);
											match s {
												[] => true,
												['л', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 37415);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 40404);
											match s {
												[] => true,
												['л', 'б', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 37416);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 40405);
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
						['у', s @ ..] => {
							super::push(out, 32912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26492);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32913);
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
				['у', s @ ..] => {
					super::push(out, 26493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26494);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32914);
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
		['н', s @ ..] => {
			super::push(out, 18889);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26495);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32915);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40406);
											s.is_empty()
										}
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40407);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['и', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 40408);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => super::t27::n42833(s, out),
				['д', s @ ..] => {
					super::push(out, 26497);
					match s {
						[] => true,
						['о', s @ ..] => super::t27::n58079(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t27::n42835(s, out),
				['х', s @ ..] => super::t27::n42836(s, out),
				['ы', s @ ..] => super::t27::n42837(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t27::n26719(s, out),
		['р', s @ ..] => super::t27::n26720(s, out),
		['с', s @ ..] => super::t27::n26721(s, out),
		['т', s @ ..] => super::t27::n26722(s, out),
		['ц', s @ ..] => super::t27::n26723(s, out),
		_ => false,
	}
}

/// Words ending -рвиться.
pub(crate) fn n13480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10880);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 18895);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 26509);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -свиться.
pub(crate) fn n13481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10881);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -твиться.
pub(crate) fn n13482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10882);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18897);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32928);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32929);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18898);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18899);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18900);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26511);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32930);
							match s {
								[] => true,
								['з', 'ы', s @ ..] => {
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 40413);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 42081);
													match s {
														[] => true,
														['б', 'о', '\u{1}', s @ ..] => super::t27::n87902(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'ж', 'о', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ж', 'о', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 32931);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37421);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 40414);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 32932);
							match s {
								[] => true,
								['б', 'о', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'в', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', 'с', s @ ..] => {
									match s {
										[] => true,
										['е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26512);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32933);
							match s {
								[] => true,
								['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26513);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32934);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 37422);
									match s {
										[] => true,
										['у', 'с', 'о', 'г', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'ч', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увиться.
pub(crate) fn n13483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10883);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывиться.
pub(crate) fn n13484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10884);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18902);
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

/// Words ending -явиться.
pub(crate) fn n13485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10885);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18903);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18904);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18905);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 26515);
					match s {
						[] => true,
						['х', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18906);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26516);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18907);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 26517);
					match s {
						[] => true,
						['у', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37423);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37424);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18908);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 26518);
					match s {
						[] => true,
						['л', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37425);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18909);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18910);
			match s {
				[] => true,
				['д', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32935);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32936);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32937);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'ч', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32938);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32939);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32940);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32941);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32942);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18911);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26519);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 26520);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26521);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32943);
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
				['т', s @ ..] => {
					super::push(out, 26522);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32944);
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
		['ы', s @ ..] => {
			super::push(out, 18912);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26523);
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

/// Words ending -гладиться.
pub(crate) fn n42885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26532);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32947);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 32948);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 32949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37427);
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

/// Words ending -иладиться.
pub(crate) fn n42888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26534);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сладиться.
pub(crate) fn n42890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26535);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32950);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 32951);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37428);
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
		['у', s @ ..] => {
			super::push(out, 32952);
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

/// The word отладиться.
pub(crate) fn n42891(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word уладиться.
pub(crate) fn n42892(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -хладиться.
pub(crate) fn n42893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26536);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мадиться.
pub(crate) fn n26747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18918);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26537);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37429);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37430);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 37431);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32955);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 37432);
							match s {
								[] => true,
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -радиться.
pub(crate) fn n26748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18919);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26538);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32956);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 26539);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26540);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 32957);
					match s {
						[] => true,
						['и', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -садиться.
pub(crate) fn n26749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18920);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26541);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26542);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32958);
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 26545);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32959);
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

/// Words ending -тадиться.
pub(crate) fn n26750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18921);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 26546);
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

/// Words ending -шадиться.
pub(crate) fn n26751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18922);
	match s {
		[] => true,
		['о', 'л', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щадиться.
pub(crate) fn n26752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18923);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26547);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26548);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32960);
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

/// Words ending -бдиться.
pub(crate) fn n13488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10888);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18924);
			match s {
				[] => true,
				['н', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едиться.
pub(crate) fn n13489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10889);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18925);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26549);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 32961);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 37433);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32962);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32963);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18926);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 26551);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32964);
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
		['л', s @ ..] => {
			super::push(out, 18927);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26552);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32965);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32966);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37434);
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
		['р', s @ ..] => {
			super::push(out, 18928);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32967);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32968);
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
				['в', s @ ..] => {
					super::push(out, 26554);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32969);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 37435);
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32970);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26555);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 37436);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 32971);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 32972);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 32973);
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
				['з', s @ ..] => {
					super::push(out, 26556);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26557);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26558);
					match s {
						[] => true,
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 26560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32974);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 32975);
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
		['с', s @ ..] => {
			super::push(out, 18929);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26561);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 18930);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26562);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26565);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32976);
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

/// Words ending -ждиться.
pub(crate) fn n13490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10890);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18931);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18932);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26567);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здиться.
pub(crate) fn n13491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18933);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26568);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32977);
							match s {
								[] => true,
								['г', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18934);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26569);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26570);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 26571);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32978);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37437);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37438);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 40415);
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
								['ы', s @ ..] => {
									super::push(out, 37439);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40416);
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
				['и', s @ ..] => {
					super::push(out, 26572);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26573);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37440);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26574);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 26576);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26577);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32980);
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
		['и', s @ ..] => {
			super::push(out, 18935);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32981);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32982);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37441);
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
						['с', s @ ..] => {
							super::push(out, 32983);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32984);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37442);
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
		['о', s @ ..] => {
			super::push(out, 18936);
			match s {
				[] => true,
				['в', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32985);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 32986);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 26579);
					match s {
						[] => true,
						['о', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37443);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37444);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 37445);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26580);
					match s {
						[] => true,
						['о', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37446);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 37447);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40417);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 40418);
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
								['о', s @ ..] => super::t27::n71101(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идиться.
pub(crate) fn n13492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10892);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 18937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26581);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26582);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32987);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26583);
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

/// Words ending -йдиться.
pub(crate) fn n13493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10893);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18938);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26584);
					match s {
						[] => true,
						['г', 'п', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндиться.
pub(crate) fn n13494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10894);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18939);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26585);
					match s {
						[] => true,
						['ф', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одиться.
pub(crate) fn n13495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10895);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18940);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26586);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37449);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37450);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37451);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40419);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18941);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26587);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 26588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 26589);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26590);
					match s {
						[] => true,
						['р', 'е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37452);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 40420);
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
				['з', s @ ..] => {
					super::push(out, 26591);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32988);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 37453);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40421);
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
						['и', s @ ..] => {
							super::push(out, 32989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'о', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26592);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 32991);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 37454);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40422);
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
						['п', s @ ..] => {
							super::push(out, 32992);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t27::n58235(s, out),
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26593);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => super::t27::n71122(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t27::n26771(s, out),
		['л', s @ ..] => super::t27::n26772(s, out),
		['р', s @ ..] => super::t27::n26773(s, out),
		['х', s @ ..] => super::t27::n26774(s, out),
		_ => false,
	}
}

/// Words ending -рдиться.
pub(crate) fn n13496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10896);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18946);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26617);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37484);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 37485);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 37486);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37487);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40443);
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
				['с', s @ ..] => {
					super::push(out, 26618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33025);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33026);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 37488);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 40444);
											match s {
												[] => true,
												['м', s @ ..] => {
													match s {
														[] => true,
														['с', '\u{1}', s @ ..] => super::t27::n87911(s, out),
														['у', '\u{1}', s @ ..] => super::t27::n87912(s, out),
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
									super::push(out, 37489);
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
						['с', s @ ..] => {
							super::push(out, 33028);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 33029);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18947);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33030);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33031);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 33032);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33033);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удиться.
pub(crate) fn n13497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10897);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18948);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18949);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26620);
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
		['б', s @ ..] => {
			super::push(out, 18950);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26621);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26622);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33034);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37490);
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
						['о', s @ ..] => {
							super::push(out, 33035);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26623);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18951);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26624);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33036);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26625);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33037);
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
		['к', s @ ..] => {
			super::push(out, 18952);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26626);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37491);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37492);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37493);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26627);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 26628);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26629);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33038);
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
		['н', s @ ..] => {
			super::push(out, 18954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26630);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 26631);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33039);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26632);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33040);
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
				['ы', s @ ..] => {
					super::push(out, 26633);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33041);
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
		['п', s @ ..] => {
			super::push(out, 18955);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18956);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26635);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33042);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33043);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26636);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 33044);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37495);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37496);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26637);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 33045);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37497);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26638);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33046);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26639);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33047);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33048);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33049);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => super::t27::n58315(s, out),
						['о', s @ ..] => super::t27::n58316(s, out),
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => super::t27::n58318(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t27::n26786(s, out),
		['т', s @ ..] => super::t27::n26787(s, out),
		['х', s @ ..] => super::t27::n26788(s, out),
		['ч', s @ ..] => super::t27::n26789(s, out),
		['ы', s @ ..] => super::t27::n26790(s, out),
		_ => false,
	}
}

/// Words ending -ыдиться.
pub(crate) fn n13498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10898);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 18962);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33064);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33065);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33066);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 33067);
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

/// Words ending -юдиться.
pub(crate) fn n13499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10899);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18963);
			match s {
				[] => true,
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядиться.
pub(crate) fn n13500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10900);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18964);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26658);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26659);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 37501);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 37502);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40448);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37503);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 40449);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
								['\u{1}', s @ ..] => {
									super::push(out, 37504);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 37505);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 37506);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40450);
											match s {
												[] => true,
												['п', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n87913(s, out),
														['о', 'п', '\u{1}', s @ ..] => super::t27::n87914(s, out),
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
									super::push(out, 37507);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26660);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 26661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33068);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26662);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26663);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37508);
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37509);
									match s {
										[] => true,
										['д', 'е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['м', 'а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26664);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26666);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26668);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33070);
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

/// Words ending -ежиться.
pub(crate) fn n13509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10909);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18976);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18977);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 18978);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26687);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26688);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18980);
			match s {
				[] => true,
				['ч', 'о', 'н', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 18981);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26689);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26690);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26691);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33078);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 33079);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37515);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37516);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 33080);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26692);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26693);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26694);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33081);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33082);
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
		['о', s @ ..] => {
			super::push(out, 18983);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26695);
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
		['р', s @ ..] => {
			super::push(out, 18984);
			match s {
				[] => true,
				['а', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26696);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33083);
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
				['о', s @ ..] => {
					super::push(out, 26697);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37518);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37519);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18985);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26698);
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

/// Words ending -брезжиться.
pub(crate) fn n58399(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37520);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 37521);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 40451);
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

/// Words ending -озжиться.
pub(crate) fn n26819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18989);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 26701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33085);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 33086);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37522);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 33087);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37523);
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

/// Words ending -ижиться.
pub(crate) fn n13511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10911);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18990);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26702);
					match s {
						[] => true,
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18991);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26703);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18992);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 18993);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26705);
					match s {
						[] => true,
						['н', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лжиться.
pub(crate) fn n13512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10912);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18994);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33089);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 33090);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 33091);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37524);
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

/// Words ending -нжиться.
pub(crate) fn n13513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10913);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18995);
			match s {
				[] => true,
				['х', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожиться.
pub(crate) fn n13514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10914);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26706);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26707);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26708);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26709);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 18997);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26710);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37525);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37526);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18999);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 26712);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37527);
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
		['л', s @ ..] => {
			super::push(out, 19000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26713);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26714);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26715);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 37528);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40452);
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
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26716);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26717);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37529);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 40453);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 42091);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t27::n87915(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 26718);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 26719);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33092);
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
		['м', s @ ..] => {
			super::push(out, 19001);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26720);
					match s {
						[] => true,
						['т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19002);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26721);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37530);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37531);
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
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => super::t27::n43132(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n26834(s, out),
		['т', s @ ..] => super::t27::n26835(s, out),
		_ => false,
	}
}

/// Words ending -сжиться.
pub(crate) fn n13515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10915);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тжиться.
pub(crate) fn n13516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10916);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19006);
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

/// Words ending -ужиться.
pub(crate) fn n13517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10917);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19007);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 19008);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26730);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37535);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37536);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 37537);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19009);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33103);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33104);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37538);
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
		['р', s @ ..] => {
			super::push(out, 19010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26732);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 26733);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33106);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33107);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 33108);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37539);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33109);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33110);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37540);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 37541);
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
						['е', s @ ..] => {
							super::push(out, 33111);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37542);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33112);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37543);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 37544);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 37545);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 37546);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33113);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 33114);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37547);
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
				['о', s @ ..] => {
					super::push(out, 26735);
					match s {
						[] => true,
						['з', s @ ..] => super::t27::n58466(s, out),
						['о', s @ ..] => super::t27::n58467(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t27::n43148(s, out),
				['т', s @ ..] => super::t27::n43149(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t27::n26842(s, out),
		['т', s @ ..] => super::t27::n26843(s, out),
		_ => false,
	}
}

/// Words ending -ыжиться.
pub(crate) fn n13518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10918);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 19013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26743);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26744);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 26745);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33122);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
				['к', s @ ..] => {
					super::push(out, 26746);
					match s {
						[] => true,
						['е', 'ч', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33123);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33124);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19014);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26747);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -южиться.
pub(crate) fn n13519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10919);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 19015);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33125);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33126);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37553);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 37554);
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
						['з', s @ ..] => {
							super::push(out, 33127);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33128);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 33129);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33130);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37555);
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
		['ь', s @ ..] => {
			super::push(out, 19016);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33131);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33132);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжиться.
pub(crate) fn n13520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10920);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19017);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26748);
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
		['р', s @ ..] => {
			super::push(out, 19018);
			match s {
				[] => true,
				['в', 'е', 'ч', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33133);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19019);
			match s {
				[] => true,
				['у', 'с', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word обеззаразиться.
pub(crate) fn n85290(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -безобразиться.
pub(crate) fn n85291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42092);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43001);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 43002);
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

/// Words ending -знообразиться.
pub(crate) fn n85292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42093);
	match s {
		[] => true,
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лезиться.
pub(crate) fn n26858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19026);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26761);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 33136);
					match s {
						[] => true,
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26762);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33137);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33138);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33139);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -резиться.
pub(crate) fn n26859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19027);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 26763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33140);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 33141);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33142);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26764);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33143);
					match s {
						[] => true,
						['е', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изиться.
pub(crate) fn n13524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10924);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19028);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33144);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 33145);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37560);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33146);
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
				['е', s @ ..] => {
					super::push(out, 26765);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26766);
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
		['м', s @ ..] => {
			super::push(out, 19029);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33147);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33148);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37561);
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
						['б', s @ ..] => {
							super::push(out, 33149);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19030);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26767);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19031);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26768);
					match s {
						[] => true,
						['а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мзиться.
pub(crate) fn n13525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10925);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 19032);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26769);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онзиться.
pub(crate) fn n26865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19033);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -озиться.
pub(crate) fn n13527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10927);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26770);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33150);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 33151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37563);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37564);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 37565);
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
				['б', s @ ..] => {
					super::push(out, 26771);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 26772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 26773);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26774);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33152);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26775);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33153);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 33154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33155);
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
				['и', s @ ..] => {
					super::push(out, 26776);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26777);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37566);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37567);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37568);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 26778);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 33157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 33158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 33159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26780);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26781);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26782);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33160);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37569);
									s.is_empty()
								}
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
			super::push(out, 19035);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33161);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33162);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37570);
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
		['л', s @ ..] => {
			super::push(out, 19036);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19037);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26785);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37571);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26786);
					match s {
						[] => true,
						['о', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37572);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19038);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26787);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n26871(s, out),
		_ => false,
	}
}

/// Words ending -рзиться.
pub(crate) fn n13528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10928);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19040);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26790);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19041);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26791);
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

/// Words ending -узиться.
pub(crate) fn n13529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10929);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19042);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19043);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33166);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33167);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33168);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19044);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26795);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33169);
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
		['г', s @ ..] => {
			super::push(out, 19045);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26796);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37576);
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19046);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26797);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19047);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 26798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33170);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33171);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37577);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40464);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 40465);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 42094);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n87921(s, out),
														_ => false,
													}
												}
												['м', s @ ..] => {
													super::push(out, 42095);
													match s {
														[] => true,
														['а', 'с', '\u{1}', s @ ..] => super::t27::n87922(s, out),
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
									super::push(out, 37578);
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
						['д', s @ ..] => {
							super::push(out, 33172);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 33173);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 33174);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37579);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 37580);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 37581);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 37582);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33176);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 33177);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33178);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37583);
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
				['д', s @ ..] => {
					super::push(out, 26799);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33179);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19048);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19049);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26800);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 26801);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33180);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 26802);
					match s {
						[] => true,
						['о', s @ ..] => super::t27::n58576(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t27::n43233(s, out),
				['у', s @ ..] => super::t27::n43234(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t27::n26882(s, out),
		['ц', s @ ..] => super::t27::n26883(s, out),
		_ => false,
	}
}

/// Words ending -юзиться.
pub(crate) fn n13530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10930);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 19052);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26807);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язиться.
pub(crate) fn n13531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10931);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19053);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алиться.
pub(crate) fn n13533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10933);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26808);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37585);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37586);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37587);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19055);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26809);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26810);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 26811);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 26812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26813);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26814);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26815);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26816);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26818);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 26819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33184);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33185);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 37588);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40466);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 40467);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 40468);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 42096);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t27::n87923(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 40469);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 37589);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 37590);
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
						['е', s @ ..] => {
							super::push(out, 33186);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 37591);
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
						['о', s @ ..] => super::t27::n58599(s, out),
						['с', s @ ..] => super::t27::n58600(s, out),
						['ы', s @ ..] => super::t27::n58601(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t27::n43254(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t27::n26888(s, out),
		['д', s @ ..] => super::t27::n26889(s, out),
		['ж', s @ ..] => super::t27::n26890(s, out),
		['з', s @ ..] => super::t27::n26891(s, out),
		['и', s @ ..] => super::t27::n26892(s, out),
		['к', s @ ..] => super::t27::n26893(s, out),
		['м', s @ ..] => super::t27::n26894(s, out),
		['н', s @ ..] => super::t27::n26895(s, out),
		['п', s @ ..] => super::t27::n26896(s, out),
		['р', s @ ..] => super::t27::n26897(s, out),
		['с', s @ ..] => super::t27::n26898(s, out),
		['ч', s @ ..] => super::t27::n26899(s, out),
		['ш', s @ ..] => super::t27::n26900(s, out),
		_ => false,
	}
}

/// The word выскоблиться.
pub(crate) fn n85305(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выкруглиться.
pub(crate) fn n85306(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -одлиться.
pub(crate) fn n26909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19077);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 26880);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33220);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33221);
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
		['р', s @ ..] => {
			super::push(out, 26881);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33222);
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

/// Words ending -елиться.
pub(crate) fn n13538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10938);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19078);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26882);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26883);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 26884);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33223);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26885);
					match s {
						[] => true,
						['т', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26886);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33224);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 33225);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 33226);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37615);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 33227);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 33228);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 26888);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26889);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26890);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33229);
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
		['в', s @ ..] => {
			super::push(out, 19079);
			match s {
				[] => true,
				['е', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33230);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33231);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 37616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33232);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33233);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37617);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19080);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26891);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26892);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 33235);
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
				['е', s @ ..] => {
					super::push(out, 26893);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 37618);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40479);
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
								['п', s @ ..] => {
									super::push(out, 37619);
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													match s {
														[] => true,
														['д', '\u{1}', s @ ..] => super::t27::n87925(s, out),
														['м', 'а', 'с', '\u{1}', s @ ..] => super::t27::n87926(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', 'а', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26894);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37620);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 37621);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40480);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26895);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 33236);
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
				['т', s @ ..] => super::t27::n43336(s, out),
				['у', s @ ..] => super::t27::n43337(s, out),
				['ы', s @ ..] => super::t27::n43338(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t27::n26913(s, out),
		['м', s @ ..] => super::t27::n26914(s, out),
		['п', s @ ..] => super::t27::n26915(s, out),
		['р', s @ ..] => super::t27::n26916(s, out),
		['с', s @ ..] => super::t27::n26917(s, out),
		['т', s @ ..] => super::t27::n26918(s, out),
		['ц', s @ ..] => super::t27::n26919(s, out),
		['ш', s @ ..] => super::t27::n26920(s, out),
		['щ', s @ ..] => super::t27::n26921(s, out),
		_ => false,
	}
}

/// Words ending -злиться.
pub(crate) fn n13539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10939);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19090);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19091);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19092);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19093);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илиться.
pub(crate) fn n13540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10940);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19094);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26934);
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
		['ж', s @ ..] => {
			super::push(out, 19095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26935);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26936);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26937);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33267);
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
				['у', s @ ..] => {
					super::push(out, 26938);
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
		['м', s @ ..] => {
			super::push(out, 19096);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 26939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26940);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26941);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 33269);
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
				['в', s @ ..] => {
					super::push(out, 26942);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26943);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33270);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26944);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 26945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33271);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33272);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33273);
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
				['т', s @ ..] => {
					super::push(out, 26946);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33274);
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
				['у', s @ ..] => {
					super::push(out, 26947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 26948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33275);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26949);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33276);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									super::push(out, 37637);
									s.is_empty()
								}
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
			super::push(out, 19098);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26950);
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
		['с', s @ ..] => {
			super::push(out, 19099);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26951);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 26953);
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
		['ф', s @ ..] => {
			super::push(out, 19100);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26954);
					match s {
						[] => true,
						['т', 'с', 'о', 'р', 'п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 19101);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26955);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 26956);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33277);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26957);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 33278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клиться.
pub(crate) fn n13541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10941);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19102);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26958);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37638);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37639);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37640);
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
		['и', s @ ..] => {
			super::push(out, 19103);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 26959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19104);
			match s {
				[] => true,
				['к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млиться.
pub(crate) fn n13542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10942);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19105);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26960);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 26961);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 19106);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26962);
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

/// Words ending -олиться.
pub(crate) fn n13543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10943);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19107);
			match s {
				[] => true,
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'у', 'ф', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19108);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26963);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37641);
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26964);
					match s {
						[] => true,
						['е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33280);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33281);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37642);
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
				['т', s @ ..] => {
					super::push(out, 26965);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33282);
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
				['у', s @ ..] => {
					super::push(out, 26966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19109);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26967);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26968);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26969);
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
		['д', s @ ..] => {
			super::push(out, 19110);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26970);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26971);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26972);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33283);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19111);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26973);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 26974);
					match s {
						[] => true,
						['о', 'п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26975);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33284);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26976);
					match s {
						[] => true,
						['е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26977);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 33285);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37643);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26978);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33286);
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
		['к', s @ ..] => {
			super::push(out, 19112);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 26979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33287);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33288);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37644);
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
		['м', s @ ..] => {
			super::push(out, 19113);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26980);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26981);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 26982);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33290);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33291);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37645);
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
				['о', s @ ..] => {
					super::push(out, 26983);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 33292);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t27::n43435(s, out),
				['т', s @ ..] => {
					super::push(out, 26985);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33298);
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
				['у', s @ ..] => {
					super::push(out, 26986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 26987);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33299);
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
		['п', s @ ..] => {
			super::push(out, 19114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t27::n43439(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n26947(s, out),
		['с', s @ ..] => super::t27::n26948(s, out),
		['т', s @ ..] => super::t27::n26949(s, out),
		['ф', s @ ..] => super::t27::n26950(s, out),
		['х', s @ ..] => super::t27::n26951(s, out),
		_ => false,
	}
}

/// Words ending -плиться.
pub(crate) fn n13544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10944);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19120);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33317);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33318);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37656);
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
						['о', s @ ..] => {
							super::push(out, 33319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33320);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37657);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 33321);
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
		['и', s @ ..] => {
			super::push(out, 19121);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 27004);
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
		['о', s @ ..] => {
			super::push(out, 19122);
			match s {
				[] => true,
				['с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19123);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33322);
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлиться.
pub(crate) fn n13545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10945);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19124);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33323);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33324);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33325);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 37658);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 37659);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33326);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33327);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37660);
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
		['у', 'б', 'з', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слиться.
pub(crate) fn n13546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10946);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19125);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19126);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33328);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33329);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37661);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 37662);
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
						['б', s @ ..] => {
							super::push(out, 33330);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37663);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 33331);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37664);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 37665);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33332);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33333);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37666);
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
		['е', s @ ..] => {
			super::push(out, 19127);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27006);
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
		['и', s @ ..] => {
			super::push(out, 19128);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33334);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 33335);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37667);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33336);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37668);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 27008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33337);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33338);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 33339);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33340);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33342);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37669);
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
		['у', s @ ..] => {
			super::push(out, 19129);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27009);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33343);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33344);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33345);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37670);
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
				['с', s @ ..] => {
					super::push(out, 27010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33346);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33347);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => super::t27::n71684(s, out),
								['н', '\u{1}', s @ ..] => super::t27::n71685(s, out),
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => super::t27::n58881(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t27::n26963(s, out),
		_ => false,
	}
}

/// Words ending -тлиться.
pub(crate) fn n13547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10947);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19131);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19132);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33356);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33357);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37675);
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
				['ч', s @ ..] => {
					super::push(out, 27012);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33358);
							match s {
								[] => true,
								['п', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19133);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27013);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37676);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37677);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37678);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37679);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40490);
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
		['о', s @ ..] => {
			super::push(out, 19134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27015);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27016);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улиться.
pub(crate) fn n13548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10948);
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
				['р', s @ ..] => {
					super::push(out, 27017);
					match s {
						[] => true,
						['а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19139);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19140);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27018);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19141);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27019);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33359);
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
				['у', s @ ..] => {
					super::push(out, 27020);
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
		['р', s @ ..] => {
			super::push(out, 19142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27021);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 27022);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33360);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27023);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33361);
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
		['с', s @ ..] => {
			super::push(out, 19143);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27024);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27025);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33362);
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
		['т', s @ ..] => {
			super::push(out, 19144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27026);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 27027);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33363);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 37680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27028);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37681);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37682);
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
		['х', s @ ..] => {
			super::push(out, 19145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27029);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27030);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33365);
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
		['ш', s @ ..] => {
			super::push(out, 19146);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27031);
					match s {
						[] => true,
						['м', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлиться.
pub(crate) fn n13549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10949);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19147);
			match s {
				[] => true,
				['р', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19148);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33366);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19149);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 27032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33367);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33368);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19150);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33369);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 33370);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлиться.
pub(crate) fn n13550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10950);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19151);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27033);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылиться.
pub(crate) fn n13551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10951);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27035);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27036);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 27037);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27038);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27039);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33371);
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
		['п', s @ ..] => {
			super::push(out, 19154);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27040);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27041);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27042);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27043);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33372);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 33373);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 37683);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19155);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33374);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37684);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40491);
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
								['о', s @ ..] => {
									super::push(out, 37685);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элиться.
pub(crate) fn n13552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10952);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 19156);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27044);
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

/// Words ending -юлиться.
pub(crate) fn n13553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10953);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19157);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27045);
					match s {
						[] => true,
						['и', 'п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 19158);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27046);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялиться.
pub(crate) fn n13554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10954);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27047);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27048);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 27049);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27050);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33375);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27051);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27052);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 33377);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27053);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33378);
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
		['п', s @ ..] => {
			super::push(out, 19160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27054);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27055);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27057);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27058);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27059);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33379);
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

/// Words ending -омиться.
pub(crate) fn n13561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10961);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19173);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27073);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33392);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27074);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33393);
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
		['к', s @ ..] => {
			super::push(out, 19174);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27075);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37686);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37687);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 37688);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37689);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 33395);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37690);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40492);
											s.is_empty()
										}
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40493);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19175);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27076);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27077);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33396);
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
				['б', s @ ..] => {
					super::push(out, 27078);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27080);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33397);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 37691);
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
						['о', s @ ..] => {
							super::push(out, 33398);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 27081);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33399);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 37692);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40494);
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
								['п', s @ ..] => {
									super::push(out, 37693);
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
						['ш', s @ ..] => {
							super::push(out, 33400);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37694);
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
				['з', s @ ..] => {
					super::push(out, 27082);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27083);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27085);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33401);
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
				['ы', s @ ..] => {
					super::push(out, 27086);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33402);
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
		['н', s @ ..] => {
			super::push(out, 19176);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27087);
					match s {
						[] => true,
						['к', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37695);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n27012(s, out),
		['т', s @ ..] => super::t27::n27013(s, out),
		_ => false,
	}
}

/// Words ending -рмиться.
pub(crate) fn n13562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10962);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19179);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33405);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 33406);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37696);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33407);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37697);
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33408);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33409);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37698);
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
				['ф', s @ ..] => {
					super::push(out, 27093);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33410);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умиться.
pub(crate) fn n13564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10964);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 19182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27095);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27096);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 37699);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40496);
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
								['в', s @ ..] => {
									super::push(out, 37700);
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
				['е', s @ ..] => {
					super::push(out, 27097);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 33411);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27098);
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
		['к', s @ ..] => {
			super::push(out, 19183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27099);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27100);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27101);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19184);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33412);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33413);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33414);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19185);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27102);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 19186);
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

/// Words ending -дымиться.
pub(crate) fn n27022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19187);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27103);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27104);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27105);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьмиться.
pub(crate) fn n13566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10966);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19188);
			match s {
				[] => true,
				['у', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33415);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмиться.
pub(crate) fn n13567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10967);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 27106);
					match s {
						[] => true,
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27107);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33416);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямиться.
pub(crate) fn n13568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10968);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19190);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33417);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37701);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 33419);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37702);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37703);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 40497);
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
								['о', s @ ..] => {
									super::push(out, 37704);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40498);
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
						['ы', s @ ..] => {
							super::push(out, 33420);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37705);
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

/// Words ending -абарабаниться.
pub(crate) fn n80433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40500);
	match s {
		[] => true,
		['н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обарабаниться.
pub(crate) fn n80434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40501);
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тбарабаниться.
pub(crate) fn n80435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40502);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тарабаниться.
pub(crate) fn n71782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37706);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обаниться.
pub(crate) fn n43599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27110);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 33421);
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

/// Words ending -ваниться.
pub(crate) fn n27027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19192);
	match s {
		[] => true,
		['л', 'о', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 27111);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33422);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ганиться.
pub(crate) fn n27028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19193);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27112);
			match s {
				[] => true,
				['л', 'у', 'х', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 40503);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42102);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27113);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37707);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27114);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37708);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 37709);
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
		['ы', 'ц', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -заниться.
pub(crate) fn n27029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19194);
	match s {
		[] => true,
		['у', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33423);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33424);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ианиться.
pub(crate) fn n27030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19195);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 27115);
			match s {
				[] => true,
				['с', 'и', 'р', 'х', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -каниться.
pub(crate) fn n27031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19196);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27116);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27117);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37710);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37711);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 37712);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 37713);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40504);
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
		['и', s @ ..] => {
			super::push(out, 27118);
			match s {
				[] => true,
				['р', 'е', 'м', 'а', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27119);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37714);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 37715);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ланиться.
pub(crate) fn n27032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19197);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27120);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27121);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33426);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маниться.
pub(crate) fn n27033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19198);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27122);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27123);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33427);
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
		['д', s @ ..] => {
			super::push(out, 27124);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33428);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27125);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 33429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37716);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37717);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40505);
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
						['о', s @ ..] => {
							super::push(out, 37718);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40506);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42103);
											s.is_empty()
										}
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
					super::push(out, 33430);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37719);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27126);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33431);
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
		['и', s @ ..] => {
			super::push(out, 27127);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33432);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27128);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33433);
					match s {
						[] => true,
						['р', 'а', 'н', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33434);
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
		['р', s @ ..] => {
			super::push(out, 27129);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33435);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40507);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40508);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 40509);
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
						['с', s @ ..] => {
							super::push(out, 37721);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40510);
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27131);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37722);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27132);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33437);
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

/// Words ending -раниться.
pub(crate) fn n27034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19199);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27133);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27134);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37723);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37724);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40511);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33439);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33440);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 33441);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 33442);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37725);
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
		['г', s @ ..] => {
			super::push(out, 27136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33443);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33444);
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
		['з', s @ ..] => {
			super::push(out, 27137);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33445);
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
		['и', s @ ..] => {
			super::push(out, 27138);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33446);
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
		['о', s @ ..] => {
			super::push(out, 27139);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33447);
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
		['т', s @ ..] => {
			super::push(out, 27140);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33448);
					match s {
						[] => true,
						['о', 'н', s @ ..] => {
							super::push(out, 37726);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 40512);
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27141);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33449);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -саниться.
pub(crate) fn n27035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19200);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таниться.
pub(crate) fn n27036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19201);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27143);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33451);
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

/// Words ending -фаниться.
pub(crate) fn n27037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19202);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27144);
			match s {
				[] => true,
				['р', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37727);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щаниться.
pub(crate) fn n27038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19203);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27145);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

