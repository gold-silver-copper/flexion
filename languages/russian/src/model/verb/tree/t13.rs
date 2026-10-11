use flexion::Path;

/// Words ending -товить.
pub(crate) fn n2849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2661);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 7405);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22847);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22848);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30677);
									match s {
										[] => true,
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
							super::push(out, 22849);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 22850);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 22851);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22852);
							match s {
								[] => true,
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
			super::push(out, 7406);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22853);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22854);
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
							super::push(out, 22855);
							match s {
								[] => true,
								['о', 'п', s @ ..] => {
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
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7407);
			match s {
				[] => true,
				['а', 'в', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22857);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22858);
							match s {
								[] => true,
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
			super::push(out, 7408);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14753);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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

/// The word разгосударствить.
pub(crate) fn n76737(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word огосударствить.
pub(crate) fn n76738(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -бредить.
pub(crate) fn n9103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7484);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14817);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14818);
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
			super::push(out, 14819);
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
			super::push(out, 14820);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вредить.
pub(crate) fn n9104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7485);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14821);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14822);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22892);
					match s {
						[] => true,
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
			super::push(out, 14823);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14824);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22894);
					match s {
						[] => true,
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

/// Words ending -одредить.
pub(crate) fn n20290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14825);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 22895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ередить.
pub(crate) fn n9106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7487);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14826);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22896);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22897);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30696);
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
		['в', s @ ..] => {
			super::push(out, 14827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22898);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22899);
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
		['п', s @ ..] => {
			super::push(out, 14828);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22900);
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22901);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30697);
							match s {
								[] => true,
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
			super::push(out, 14829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22902);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22903);
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

/// Words ending -зредить.
pub(crate) fn n9107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7488);
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

/// Words ending -оредить.
pub(crate) fn n9108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7489);
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

/// Words ending -предить.
pub(crate) fn n9109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7490);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 14830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word уредить.
pub(crate) fn n20300(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -чредить.
pub(crate) fn n9111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7492);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14831);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 14832);
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

/// Words ending -седить.
pub(crate) fn n2885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2696);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7493);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14833);
					match s {
						[] => true,
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

/// Words ending -цедить.
pub(crate) fn n2886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2697);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7494);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7495);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7496);
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
			super::push(out, 7497);
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
			super::push(out, 7498);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14834);
					match s {
						[] => true,
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

/// The word испиздить.
pub(crate) fn n35416(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тпиздить.
pub(crate) fn n20343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14857);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word упиздить.
pub(crate) fn n20344(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыпиздить.
pub(crate) fn n20345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14858);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22915);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оздить.
pub(crate) fn n2893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2704);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7516);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22916);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22917);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['м', s @ ..] => {
			super::push(out, 7517);
			match s {
				[] => true,
				['о', 'р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22918);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22919);
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
							super::push(out, 22920);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7518);
			match s {
				[] => true,
				['о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22921);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22922);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30700);
									match s {
										[] => true,
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
							super::push(out, 22923);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30701);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 36275);
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
							super::push(out, 22924);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30702);
									match s {
										[] => true,
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
							super::push(out, 22925);
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
						['ы', s @ ..] => {
							super::push(out, 22926);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30703);
									match s {
										[] => true,
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

/// Words ending -уздить.
pub(crate) fn n2894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2705);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7519);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14860);
					match s {
						[] => true,
						['б', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юздить.
pub(crate) fn n2895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2706);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7520);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 14861);
					match s {
						[] => true,
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

/// Words ending -апереводить.
pub(crate) fn n65969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36277);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 39751);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теводить.
pub(crate) fn n20378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14882);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 22944);
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

/// Words ending -зводить.
pub(crate) fn n9171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7545);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14883);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30710);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 36278);
									match s {
										[] => true,
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
		['в', s @ ..] => {
			super::push(out, 14884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14885);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22945);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14886);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 30712);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39752);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 41626);
													match s {
														[] => true,
														['а', 'с', '\u{1}', s @ ..] => super::t20::n86800(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14887);
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

/// Words ending -иводить.
pub(crate) fn n9172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7546);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14888);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 22946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оводить.
pub(crate) fn n9173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7547);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14890);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22947);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36279);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36280);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39753);
											match s {
												[] => true,
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
		['л', s @ ..] => {
			super::push(out, 14891);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22948);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 30713);
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
		['н', s @ ..] => {
			super::push(out, 14892);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22949);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 30714);
							match s {
								[] => true,
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
			super::push(out, 14893);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14894);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22951);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 30715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36281);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36282);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 39754);
											match s {
												[] => true,
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
				['п', s @ ..] => {
					super::push(out, 22952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30716);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 30717);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 36283);
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
							super::push(out, 30718);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 30719);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 36284);
									match s {
										[] => true,
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
		['т', s @ ..] => {
			super::push(out, 14895);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22953);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 30720);
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
		['х', s @ ..] => {
			super::push(out, 14896);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22954);
					match s {
						[] => true,
						['е', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36285);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 39755);
											match s {
												[] => true,
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

/// Words ending -сводить.
pub(crate) fn n9174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7548);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14897);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22955);
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
		_ => false,
	}
}

/// Words ending -тводить.
pub(crate) fn n9175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7549);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14898);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уводить.
pub(crate) fn n9176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7550);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыводить.
pub(crate) fn n9177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7551);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22956);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22957);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 30722);
							match s {
								[] => true,
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

/// Words ending -годить.
pub(crate) fn n2910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2721);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7552);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7553);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14900);
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
			super::push(out, 7554);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22958);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22959);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36286);
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
						['о', s @ ..] => {
							super::push(out, 22960);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22961);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30725);
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
		['у', s @ ..] => {
			super::push(out, 7555);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кодить.
pub(crate) fn n2911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2722);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7556);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7557);
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
		['ш', s @ ..] => {
			super::push(out, 7558);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14901);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14902);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22962);
							match s {
								[] => true,
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

/// Words ending -лодить.
pub(crate) fn n2912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2723);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7559);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14903);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14904);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22963);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22964);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22965);
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
						['о', s @ ..] => {
							super::push(out, 22966);
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
				['с', s @ ..] => {
					super::push(out, 14906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22967);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22968);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30727);
									match s {
										[] => true,
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
							super::push(out, 22969);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30728);
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
							super::push(out, 22970);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22971);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30729);
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
							super::push(out, 22972);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30730);
									match s {
										[] => true,
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
				['х', s @ ..] => {
					super::push(out, 14907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22973);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22974);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30731);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 30732);
									match s {
										[] => true,
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
							super::push(out, 22975);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22976);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30733);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30734);
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
							super::push(out, 22977);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22978);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30735);
									match s {
										[] => true,
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
		['п', s @ ..] => {
			super::push(out, 7560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14908);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14909);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22979);
							match s {
								[] => true,
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
					super::push(out, 14910);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22980);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 22981);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30736);
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
				['ы', s @ ..] => {
					super::push(out, 14911);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22982);
							match s {
								[] => true,
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

/// Words ending -родить.
pub(crate) fn n2913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2724);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7561);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7562);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14912);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22983);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22984);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30737);
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
		['б', s @ ..] => {
			super::push(out, 7563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14914);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14915);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22985);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30738);
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
				['е', s @ ..] => {
					super::push(out, 14916);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22986);
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
					super::push(out, 14917);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22987);
							match s {
								[] => true,
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
					super::push(out, 14918);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 22989);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30739);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39756);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 39757);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t20::n83299(s, out),
														['н', '\u{1}', s @ ..] => super::t20::n83300(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 39758);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t20::n83301(s, out),
														_ => false,
													}
												}
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
							super::push(out, 22990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22991);
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
					super::push(out, 14919);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22992);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22993);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 30740);
									match s {
										[] => true,
										['у', 'с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 39759);
													match s {
														[] => true,
														['н', s @ ..] => super::t20::n83302(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14920);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22994);
							match s {
								[] => true,
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
					super::push(out, 14921);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22995);
							match s {
								[] => true,
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
			super::push(out, 7564);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14922);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22996);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36287);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36288);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['б', s @ ..] => {
											super::push(out, 36289);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 39760);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83303(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 36290);
											match s {
												[] => true,
												['е', 'б', 'о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14923);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22997);
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
			super::push(out, 7565);
			match s {
				[] => true,
				['и', s @ ..] => super::t20::n20426(s, out),
				['о', s @ ..] => super::t20::n20427(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t20::n9192(s, out),
		['о', s @ ..] => super::t20::n9193(s, out),
		['т', s @ ..] => super::t20::n9194(s, out),
		['у', s @ ..] => super::t20::n9195(s, out),
		['ы', s @ ..] => super::t20::n9196(s, out),
		_ => false,
	}
}

/// Words ending -ходить.
pub(crate) fn n2914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2725);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7571);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14934);
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
				['н', s @ ..] => {
					super::push(out, 14935);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7572);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14936);
					match s {
						[] => true,
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
			super::push(out, 7573);
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
		['д', s @ ..] => {
			super::push(out, 7574);
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
			super::push(out, 7575);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14937);
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7576);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14938);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14939);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30748);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36292);
											match s {
												[] => true,
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
			super::push(out, 7577);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14942);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30749);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30750);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36293);
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
			super::push(out, 7578);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14943);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23015);
							match s {
								[] => true,
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
					super::push(out, 14944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23016);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 23017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23018);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30751);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30752);
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
					super::push(out, 14946);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23019);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30753);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36294);
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
						['п', s @ ..] => {
							super::push(out, 23020);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30754);
									s.is_empty()
								}
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
			super::push(out, 7579);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14948);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14949);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23022);
							match s {
								[] => true,
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
			super::push(out, 7581);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23023);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23024);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30755);
									match s {
										[] => true,
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

/// The word напрокудить.
pub(crate) fn n76764(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word напаскудить.
pub(crate) fn n76765(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word испаскудить.
pub(crate) fn n76766(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лудить.
pub(crate) fn n2925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2736);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7606);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7607);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14970);
					match s {
						[] => true,
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
			super::push(out, 7608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14971);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14972);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23040);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23041);
							match s {
								[] => true,
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
					super::push(out, 14973);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23042);
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
					super::push(out, 14974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23043);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 23044);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30767);
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
						['п', s @ ..] => {
							super::push(out, 23045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23046);
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
					super::push(out, 14975);
					match s {
						[] => true,
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
			super::push(out, 7609);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14976);
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
			super::push(out, 7610);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14977);
					match s {
						[] => true,
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
			super::push(out, 7611);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14978);
					match s {
						[] => true,
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
			super::push(out, 7612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14979);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14980);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23048);
							match s {
								[] => true,
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
			super::push(out, 7613);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14981);
					match s {
						[] => true,
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

/// Words ending -инудить.
pub(crate) fn n9244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7617);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14983);
			match s {
				[] => true,
				['п', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -понудить.
pub(crate) fn n20501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14984);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ынудить.
pub(crate) fn n9246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7619);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оудить.
pub(crate) fn n2928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2739);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 7620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пудить.
pub(crate) fn n2929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2740);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7621);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рудить.
pub(crate) fn n2930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2741);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7622);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 14986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23049);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23050);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30768);
									match s {
										[] => true,
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
			super::push(out, 7623);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7624);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14988);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14989);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['д', s @ ..] => {
					super::push(out, 14990);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14991);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14992);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14993);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14994);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23052);
							match s {
								[] => true,
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
			super::push(out, 7625);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14995);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14996);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23053);
							match s {
								[] => true,
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
					super::push(out, 14997);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23054);
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
					super::push(out, 14998);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23055);
							match s {
								[] => true,
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
					super::push(out, 14999);
					match s {
						[] => true,
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

/// Words ending -судить.
pub(crate) fn n2931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2742);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7626);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7627);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15000);
					match s {
						[] => true,
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
			super::push(out, 7628);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23056);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 23057);
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							super::push(out, 23058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7629);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15002);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23059);
							match s {
								[] => true,
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
			super::push(out, 7630);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15003);
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
		['и', s @ ..] => {
			super::push(out, 7631);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15004);
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
			super::push(out, 7632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15005);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15007);
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
			super::push(out, 7633);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15008);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30769);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 36298);
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
									super::push(out, 30770);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 36299);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 39763);
													match s {
														[] => true,
														['л', 'б', 'а', 'з', '\u{1}', s @ ..] => super::t20::n83305(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 36300);
											match s {
												[] => true,
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
					super::push(out, 15009);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23060);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30771);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30772);
									match s {
										[] => true,
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
		['т', s @ ..] => {
			super::push(out, 7634);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15010);
					match s {
						[] => true,
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
			super::push(out, 7635);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15011);
					match s {
						[] => true,
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

/// Words ending -тудить.
pub(crate) fn n2932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2743);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7636);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15012);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15013);
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
						['р', 'п', s @ ..] => {
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
				['ы', s @ ..] => {
					super::push(out, 15014);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23061);
							match s {
								[] => true,
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

/// Words ending -худить.
pub(crate) fn n2933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2744);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7637);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чудить.
pub(crate) fn n2934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2745);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7638);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7639);
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
			super::push(out, 7640);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15015);
					match s {
						[] => true,
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
			super::push(out, 7641);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15016);
					match s {
						[] => true,
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
			super::push(out, 7642);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыудить.
pub(crate) fn n2935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2746);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7643);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word недозарядить.
pub(crate) fn n76770(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word принарядить.
pub(crate) fn n76771(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word попринарядить.
pub(crate) fn n76772(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word заподрядить.
pub(crate) fn n76773(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word рукоположить.
pub(crate) fn n76774(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -уложить.
pub(crate) fn n9369(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ыложить.
pub(crate) fn n9370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7720);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15070);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -можить.
pub(crate) fn n2978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2788);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7721);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ножить.
pub(crate) fn n2979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2789);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7722);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23101);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23102);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7723);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15071);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15072);
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
				['у', s @ ..] => {
					super::push(out, 15073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -пожить.
pub(crate) fn n2980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2790);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рожить.
pub(crate) fn n2981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2791);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 7724);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15074);
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
			super::push(out, 7725);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23103);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23104);
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
							super::push(out, 23105);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23106);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30786);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30787);
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
							super::push(out, 23107);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30788);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30789);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30790);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23108);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23109);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30791);
									match s {
										[] => true,
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
							super::push(out, 23110);
							match s {
								[] => true,
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
					super::push(out, 15077);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30792);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30793);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 36304);
											match s {
												[] => true,
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
									super::push(out, 30794);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36305);
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
									super::push(out, 30795);
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
								['у', s @ ..] => {
									super::push(out, 30796);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36306);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7727);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23112);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23113);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23114);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 23115);
							match s {
								[] => true,
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

/// Words ending -тожить.
pub(crate) fn n2982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2792);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7729);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 7730);
			match s {
				[] => true,
				['и', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23116);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23117);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 23118);
							match s {
								[] => true,
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
			super::push(out, 7731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -идружить.
pub(crate) fn n20688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15098);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 23130);
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

/// Words ending -одружить.
pub(crate) fn n20689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15099);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 23131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сдружить.
pub(crate) fn n20690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15100);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удружить.
pub(crate) fn n20691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15101);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кружить.
pub(crate) fn n9390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7738);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15102);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15103);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23133);
					match s {
						[] => true,
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
			super::push(out, 15104);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23134);
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
			super::push(out, 15105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 23135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23136);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30797);
							match s {
								[] => true,
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
			super::push(out, 15106);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23137);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30798);
							match s {
								[] => true,
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
					super::push(out, 23138);
					match s {
						[] => true,
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
			super::push(out, 15107);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23139);
					match s {
						[] => true,
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
			super::push(out, 15108);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23140);
					match s {
						[] => true,
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

/// Words ending -оружить.
pub(crate) fn n9391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7739);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 15109);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23141);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23142);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 30799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15110);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -пружить.
pub(crate) fn n9392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7740);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15111);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15112);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тружить.
pub(crate) fn n9393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7741);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 15114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23143);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23144);
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

/// Words ending -тужить.
pub(crate) fn n2989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2799);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7742);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7743);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23145);
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
		['и', s @ ..] => {
			super::push(out, 7744);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7745);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7746);
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

/// The word побродяжить.
pub(crate) fn n76777(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -набезобразить.
pub(crate) fn n83306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41629);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обезобразить.
pub(crate) fn n83307(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -благообразить.
pub(crate) fn n76782(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41630);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41631);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нообразить.
pub(crate) fn n52045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30814);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 36311);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41632);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 41633);
							match s {
								[] => true,
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
			super::push(out, 36312);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41634);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41635);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 41636);
							match s {
								[] => true,
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
			super::push(out, 36313);
			match s {
				[] => true,
				['д', 'е', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41637);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 41638);
							match s {
								[] => true,
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

/// Words ending -оообразить.
pub(crate) fn n52046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30815);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 36314);
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

/// Words ending -сообразить.
pub(crate) fn n52047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30816);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тобразить.
pub(crate) fn n35877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23167);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зразить.
pub(crate) fn n9445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7786);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15159);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23168);
					match s {
						[] => true,
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
			super::push(out, 15160);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23169);
					match s {
						[] => true,
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

/// Words ending -оразить.
pub(crate) fn n9446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7787);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15161);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сразить.
pub(crate) fn n9447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7788);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тразить.
pub(crate) fn n9448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7789);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 15162);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыразить.
pub(crate) fn n9449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7790);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word проелозить.
pub(crate) fn n35918(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ъелозить.
pub(crate) fn n20828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15203);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 23186);
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

/// Words ending -юлозить.
pub(crate) fn n9490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7823);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мозить.
pub(crate) fn n3027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2837);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7824);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23187);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23188);
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
			super::push(out, 7825);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23189);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23190);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 23191);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30821);
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
						['и', s @ ..] => {
							super::push(out, 23192);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30822);
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
							super::push(out, 23193);
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
						['т', s @ ..] => {
							super::push(out, 23194);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30823);
									match s {
										[] => true,
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
			super::push(out, 7826);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15205);
					match s {
						[] => true,
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

/// Words ending -нозить.
pub(crate) fn n3028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2838);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7827);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15206);
					match s {
						[] => true,
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

/// Words ending -розить.
pub(crate) fn n3029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2839);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15207);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15208);
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
				['и', s @ ..] => {
					super::push(out, 15209);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15210);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 7829);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23195);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23196);
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
							super::push(out, 23197);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36315);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39768);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83317(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
						['з', s @ ..] => {
							super::push(out, 23198);
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
							super::push(out, 23199);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30824);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30825);
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
							super::push(out, 23200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23201);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 23202);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30826);
									match s {
										[] => true,
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

/// Words ending -фозить.
pub(crate) fn n3030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2840);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7830);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15212);
					match s {
						[] => true,
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

/// Words ending -хозить.
pub(crate) fn n3031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2841);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7831);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15213);
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
			super::push(out, 7832);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15214);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30827);
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -тузить.
pub(crate) fn n3040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2850);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7845);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7846);
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
		['и', s @ ..] => {
			super::push(out, 7847);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15240);
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
			super::push(out, 7848);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15241);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30832);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36316);
											match s {
												[] => true,
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
					super::push(out, 15242);
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
			super::push(out, 7849);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15243);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 7850);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15244);
					match s {
						[] => true,
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
			super::push(out, 7851);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15245);
					match s {
						[] => true,
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
			super::push(out, 7852);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23213);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23214);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30833);
									match s {
										[] => true,
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
							super::push(out, 23215);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30834);
									match s {
										[] => true,
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

/// Words ending -фузить.
pub(crate) fn n3041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2851);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 7853);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23216);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23217);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30835);
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
							super::push(out, 23218);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23219);
							match s {
								[] => true,
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

/// Words ending -цузить.
pub(crate) fn n3042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2852);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 7854);
			match s {
				[] => true,
				['а', 'р', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23220);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23221);
							match s {
								[] => true,
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

/// The word расканнибалить.
pub(crate) fn n76789(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -развалить.
pub(crate) fn n20906(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
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

/// Words ending -ивалить.
pub(crate) fn n9540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7870);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15260);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15261);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23231);
					match s {
						[] => true,
						['е', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -квалить.
pub(crate) fn n9541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7871);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 15262);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23232);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овалить.
pub(crate) fn n9542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7872);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15263);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 15264);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23234);
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
		['п', s @ ..] => {
			super::push(out, 15265);
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

/// The word свалить.
pub(crate) fn n20916(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -твалить.
pub(crate) fn n9544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7874);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word увалить.
pub(crate) fn n9545(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -хвалить.
pub(crate) fn n9546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7875);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15266);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15267);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 23235);
					s.is_empty()
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
			super::push(out, 15268);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15269);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23236);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23237);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 30841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 15270);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23238);
					match s {
						[] => true,
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

/// Words ending -ывалить.
pub(crate) fn n9547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7876);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15271);
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

/// Words ending -галить.
pub(crate) fn n3051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2861);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7877);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15272);
					match s {
						[] => true,
						['п', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30842);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 36318);
											match s {
												[] => true,
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
					super::push(out, 15273);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30843);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36319);
											match s {
												[] => true,
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

/// Words ending -далить.
pub(crate) fn n3052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2862);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 7878);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 23240);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36320);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36321);
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
											super::push(out, 36322);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39769);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 39770);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83318(s, out),
														_ => false,
													}
												}
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
							super::push(out, 23241);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30845);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30846);
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
			super::push(out, 7879);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7880);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жалить.
pub(crate) fn n3053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2863);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7881);
			s.is_empty()
		}
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23242);
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

/// Words ending -залить.
pub(crate) fn n3054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2864);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7882);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7883);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15275);
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
		_ => false,
	}
}

/// Words ending -калить.
pub(crate) fn n3055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2865);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7884);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7885);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23243);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30847);
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
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 15278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23244);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 23245);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30848);
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
							super::push(out, 23246);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30849);
									match s {
										[] => true,
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
		['б', s @ ..] => {
			super::push(out, 7886);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15279);
					match s {
						[] => true,
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
			super::push(out, 7887);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15280);
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
			super::push(out, 7888);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15281);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 15282);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23247);
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
		['о', s @ ..] => {
			super::push(out, 7889);
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
		['р', s @ ..] => {
			super::push(out, 7890);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15283);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30850);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30851);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 30852);
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
		['с', s @ ..] => {
			super::push(out, 7891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15284);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15285);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23248);
							match s {
								[] => true,
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
					super::push(out, 15286);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 23249);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30853);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 15287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23250);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 23251);
							match s {
								[] => true,
								['у', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36323);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36324);
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
						['п', s @ ..] => {
							super::push(out, 23252);
							match s {
								[] => true,
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
					super::push(out, 15288);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23253);
							match s {
								[] => true,
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
			super::push(out, 7892);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15289);
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
		['ы', s @ ..] => {
			super::push(out, 7893);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15291);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23254);
							match s {
								[] => true,
								['м', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -малить.
pub(crate) fn n3056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2866);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7894);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15292);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 23255);
							match s {
								[] => true,
								['с', 'к', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7896);
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
		['х', s @ ..] => {
			super::push(out, 7897);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15294);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30854);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30855);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
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
								['о', s @ ..] => {
									super::push(out, 30856);
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
								['ы', s @ ..] => {
									super::push(out, 30857);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36325);
											match s {
												[] => true,
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
		['ш', s @ ..] => {
			super::push(out, 7898);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15295);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -налить.
pub(crate) fn n3057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2867);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7899);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15296);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15297);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23256);
							match s {
								[] => true,
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
			super::push(out, 7901);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15298);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30858);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30859);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30860);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36326);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 36327);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39771);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83319(s, out),
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
									super::push(out, 30861);
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
		['и', s @ ..] => {
			super::push(out, 7902);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 15299);
					match s {
						[] => true,
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

/// Words ending -палить.
pub(crate) fn n3058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2868);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7903);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7904);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 15300);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15301);
					match s {
						[] => true,
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
			super::push(out, 7905);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7906);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7907);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15303);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23258);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'с', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7909);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7910);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7911);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15304);
					match s {
						[] => true,
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

/// Words ending -ралить.
pub(crate) fn n3059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2869);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23259);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23260);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30863);
									match s {
										[] => true,
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
			super::push(out, 7913);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15306);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23261);
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
				['н', s @ ..] => {
					super::push(out, 15307);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23262);
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
		['о', s @ ..] => {
			super::push(out, 7914);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15308);
					match s {
						[] => true,
						['з', 'и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7915);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15309);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15310);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23263);
							match s {
								[] => true,
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
					super::push(out, 15311);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23264);
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
					super::push(out, 15312);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23265);
							match s {
								[] => true,
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

/// Words ending -салить.
pub(crate) fn n3060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2870);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7916);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7917);
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
		['е', s @ ..] => {
			super::push(out, 7918);
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
			super::push(out, 7919);
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
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15313);
					match s {
						[] => true,
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
			super::push(out, 7921);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15314);
					match s {
						[] => true,
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

/// Words ending -талить.
pub(crate) fn n3061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2871);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7922);
			match s {
				[] => true,
				['п', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23266);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 23267);
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
				['р', s @ ..] => {
					super::push(out, 15315);
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
			super::push(out, 7923);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15316);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23268);
							match s {
								[] => true,
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
					super::push(out, 15317);
					match s {
						[] => true,
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

/// Words ending -чалить.
pub(crate) fn n3062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2872);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7924);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7925);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15319);
					match s {
						[] => true,
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
			super::push(out, 7926);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7927);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23269);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23270);
							match s {
								[] => true,
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
					super::push(out, 15321);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23271);
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
		['и', s @ ..] => {
			super::push(out, 7928);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15322);
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
			super::push(out, 7929);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23272);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23273);
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
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7930);
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
		['т', s @ ..] => {
			super::push(out, 7931);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15323);
					match s {
						[] => true,
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
			super::push(out, 7932);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7933);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15325);
					match s {
						[] => true,
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

/// Words ending -шалить.
pub(crate) fn n3063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2873);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7934);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7935);
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
			super::push(out, 7936);
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
			super::push(out, 7937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пределить.
pub(crate) fn n36147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23297);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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

/// Words ending -разделить.
pub(crate) fn n21060(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 23298);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30870);
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

/// Words ending -оделить.
pub(crate) fn n9654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7973);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15354);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15356);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тделить.
pub(crate) fn n9655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7974);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			super::push(out, 15357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уделить.
pub(crate) fn n9656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7975);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15358);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 15359);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23299);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыделить.
pub(crate) fn n9657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7976);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15360);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -желить.
pub(crate) fn n3079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2889);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7977);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15361);
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
		['я', s @ ..] => {
			super::push(out, 7978);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23300);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
		_ => false,
	}
}

/// Words ending -лелить.
pub(crate) fn n3080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2890);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 7979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15362);
					match s {
						[] => true,
						['р', 'а', 'п', s @ ..] => {
							match s {
								[] => true,
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мелить.
pub(crate) fn n3081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2891);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7980);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7981);
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
			super::push(out, 7982);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15363);
					match s {
						[] => true,
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
			super::push(out, 7983);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7984);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15364);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15365);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23301);
							match s {
								[] => true,
								['е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7985);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7986);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15368);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23302);
							match s {
								[] => true,
								['у', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7987);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15369);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7988);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15371);
					match s {
						[] => true,
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

/// Words ending -пелить.
pub(crate) fn n3082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2892);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7989);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23303);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23304);
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

/// Words ending -релить.
pub(crate) fn n3083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2893);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 7990);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7991);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15374);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['о', s @ ..] => {
							super::push(out, 23306);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23307);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30871);
									match s {
										[] => true,
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

/// Words ending -селить.
pub(crate) fn n3084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2894);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7993);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7994);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23308);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
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
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23309);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7995);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15376);
					match s {
						[] => true,
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
			super::push(out, 7996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7997);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15377);
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
			super::push(out, 7998);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23310);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23311);
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
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15379);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23312);
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
		['и', s @ ..] => {
			super::push(out, 7999);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15380);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23313);
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
				['р', s @ ..] => {
					super::push(out, 15381);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 23314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8000);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15382);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30872);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30873);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 36328);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8001);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15383);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30874);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30875);
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
		['о', s @ ..] => {
			super::push(out, 8002);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8003);
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
		['т', s @ ..] => {
			super::push(out, 8004);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15384);
					match s {
						[] => true,
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
			super::push(out, 8005);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15385);
					match s {
						[] => true,
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

/// Words ending -телить.
pub(crate) fn n3085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2895);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8006);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8007);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23316);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23317);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23318);
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
		['и', s @ ..] => {
			super::push(out, 8008);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15387);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30876);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30877);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8010);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15388);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15389);
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15390);
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
					super::push(out, 15391);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15392);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23319);
							match s {
								[] => true,
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

/// Words ending -целить.
pub(crate) fn n3086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2896);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8011);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8012);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15393);
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
		['и', s @ ..] => {
			super::push(out, 8013);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15394);
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
			super::push(out, 8014);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15395);
					match s {
						[] => true,
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
			super::push(out, 8015);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15396);
					match s {
						[] => true,
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

/// Words ending -челить.
pub(crate) fn n3087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2897);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8016);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15397);
					match s {
						[] => true,
						['л', 'о', 'и', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шелить.
pub(crate) fn n3088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2898);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8017);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15398);
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
		_ => false,
	}
}

/// Words ending -щелить.
pub(crate) fn n3089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2899);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 8018);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15399);
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

/// The word остропилить.
pub(crate) fn n76802(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -обессилить.
pub(crate) fn n52231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30880);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усилить.
pub(crate) fn n9734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8051);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тилить.
pub(crate) fn n3102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2912);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 8052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хилить.
pub(crate) fn n3103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2913);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чилить.
pub(crate) fn n3104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2914);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -соблаговолить.
pub(crate) fn n76803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39772);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word помирволить.
pub(crate) fn n76804(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -голить.
pub(crate) fn n3117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2927);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8082);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8083);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15451);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 23344);
							match s {
								[] => true,
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
			super::push(out, 8084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -долить.
pub(crate) fn n3118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2928);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8085);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8086);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15453);
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
			super::push(out, 8087);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8088);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15455);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30885);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30886);
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
		['у', s @ ..] => {
			super::push(out, 8089);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15456);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 23345);
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

/// Words ending -золить.
pub(crate) fn n3119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2929);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8090);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8091);
			match s {
				[] => true,
				['о', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8092);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15457);
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
			super::push(out, 8093);
			match s {
				[] => true,
				['е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15458);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23346);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
		['т', s @ ..] => {
			super::push(out, 8095);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15460);
					match s {
						[] => true,
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
			super::push(out, 8096);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15461);
					match s {
						[] => true,
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

/// Words ending -колить.
pub(crate) fn n3120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2930);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8097);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15462);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23347);
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
				['т', s @ ..] => {
					super::push(out, 15463);
					match s {
						[] => true,
						['о', 'р', 'п', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 8098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15464);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15465);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15466);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 15467);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23348);
							match s {
								[] => true,
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

/// Words ending -молить.
pub(crate) fn n3121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2931);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8099);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8100);
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
			super::push(out, 8101);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15468);
					match s {
						[] => true,
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
			super::push(out, 8102);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15469);
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
			super::push(out, 8103);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15470);
					match s {
						[] => true,
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
			super::push(out, 8104);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15471);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 23349);
							match s {
								[] => true,
								['о', 'к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15472);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15473);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15474);
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
					super::push(out, 15475);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23350);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30887);
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
				['ы', s @ ..] => {
					super::push(out, 15476);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23351);
							match s {
								[] => true,
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
			super::push(out, 8106);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15477);
					match s {
						[] => true,
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
			super::push(out, 8107);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8108);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15478);
					match s {
						[] => true,
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

/// Words ending -полить.
pub(crate) fn n3122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2932);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ролить.
pub(crate) fn n3123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2933);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8109);
			match s {
				[] => true,
				['п', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8110);
			match s {
				[] => true,
				['д', 'н', 'а', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8111);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -солить.
pub(crate) fn n3124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2934);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8112);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8113);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15480);
					match s {
						[] => true,
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
			super::push(out, 8114);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15481);
					match s {
						[] => true,
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
			super::push(out, 8115);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15482);
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
			super::push(out, 8116);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15483);
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
		['и', s @ ..] => {
			super::push(out, 8117);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15484);
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
			super::push(out, 8118);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15485);
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
					super::push(out, 15486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15487);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23353);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30888);
									s.is_empty()
								}
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
			super::push(out, 8119);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15488);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23354);
							match s {
								[] => true,
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
					super::push(out, 15489);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23355);
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
					super::push(out, 15490);
					match s {
						[] => true,
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
			super::push(out, 8120);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15491);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15492);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23356);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23357);
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
						['з', s @ ..] => {
							super::push(out, 23358);
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
							super::push(out, 23359);
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
						['т', s @ ..] => {
							super::push(out, 23360);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30889);
									match s {
										[] => true,
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
					super::push(out, 15493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23361);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23362);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 23363);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23364);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30890);
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
		['ы', s @ ..] => {
			super::push(out, 8121);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15494);
					match s {
						[] => true,
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

/// Words ending -толить.
pub(crate) fn n3125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2935);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фолить.
pub(crate) fn n3126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2936);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8123);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8124);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15495);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30891);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30892);
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
			super::push(out, 8125);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -холить.
pub(crate) fn n3127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2937);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8126);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8127);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'а', 'б', '\u{1}', s @ ..] => {
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
		['с', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8128);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15496);
					match s {
						[] => true,
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

/// The word просверлить.
pub(crate) fn n76805(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word сопричислить.
pub(crate) fn n76806(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -услить.
pub(crate) fn n3139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2949);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15523);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15524);
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15525);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23377);
							match s {
								[] => true,
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
			super::push(out, 8142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15526);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15527);
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыслить.
pub(crate) fn n3140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2950);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15528);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15529);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23378);
							match s {
								[] => true,
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
					super::push(out, 15530);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23379);
							match s {
								[] => true,
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
					super::push(out, 15531);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23380);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30896);
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
						['и', s @ ..] => {
							super::push(out, 23381);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15532);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23382);
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
					super::push(out, 15533);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23383);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23384);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30898);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36335);
											match s {
												[] => true,
												['а', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 30899);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 36336);
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
						['п', s @ ..] => {
							super::push(out, 23385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23386);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30900);
									match s {
										[] => true,
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
							super::push(out, 23387);
							match s {
								[] => true,
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
					super::push(out, 15534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23388);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23389);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30901);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30902);
									match s {
										[] => true,
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
					super::push(out, 15535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23390);
							match s {
								[] => true,
								['л', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 15536);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23391);
							match s {
								[] => true,
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

/// Words ending -нтулить.
pub(crate) fn n9867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8174);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15555);
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

/// Words ending -утулить.
pub(crate) fn n9868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8175);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 15556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23402);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23403);
					match s {
						[] => true,
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

/// Words ending -ытулить.
pub(crate) fn n9869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8176);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хулить.
pub(crate) fn n3158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2968);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8177);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8178);
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
		['с', s @ ..] => {
			super::push(out, 8179);
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

/// Words ending -шулить.
pub(crate) fn n3159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2969);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8180);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15558);
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

/// The word познакомить.
pub(crate) fn n76811(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поломить.
pub(crate) fn n36482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23440);
	s.is_empty()
}

/// Words ending -роломить.
pub(crate) fn n21478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15610);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 23441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23442);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30918);
					match s {
						[] => true,
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

/// Words ending -сломить.
pub(crate) fn n9973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8247);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тломить.
pub(crate) fn n9974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8248);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word уломить.
pub(crate) fn n9975(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыломить.
pub(crate) fn n9976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8249);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -номить.
pub(crate) fn n3202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3012);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8250);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15612);
					match s {
						[] => true,
						['э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30919);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30920);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30921);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36340);
											match s {
												[] => true,
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
									super::push(out, 30922);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 30923);
									match s {
										[] => true,
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
				['р', s @ ..] => {
					super::push(out, 15613);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 23443);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ромить.
pub(crate) fn n3203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3013);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 8251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15614);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15615);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 23444);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30924);
									s.is_empty()
								}
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
			super::push(out, 8252);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15617);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23445);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30925);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30926);
									match s {
										[] => true,
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
					super::push(out, 15618);
					match s {
						[] => true,
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

/// Words ending -томить.
pub(crate) fn n3204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3014);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8253);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8254);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23446);
							match s {
								[] => true,
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8255);
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
			super::push(out, 8256);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15620);
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
		['с', s @ ..] => {
			super::push(out, 8257);
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
		['у', s @ ..] => {
			super::push(out, 8258);
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

/// The word забарабанить.
pub(crate) fn n76813(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word набарабанить.
pub(crate) fn n76814(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word добарабанить.
pub(crate) fn n76815(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word побарабанить.
pub(crate) fn n76816(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word пробарабанить.
pub(crate) fn n76817(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word потарабанить.
pub(crate) fn n76818(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рбанить.
pub(crate) fn n10019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8285);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15655);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30942);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 30943);
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
		['ы', 'д', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ванить.
pub(crate) fn n3223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3033);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8286);
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 8287);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15656);
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
			super::push(out, 8288);
			match s {
				[] => true,
				['л', 'о', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15657);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23474);
							match s {
								[] => true,
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

/// Words ending -ганить.
pub(crate) fn n3224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3034);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8289);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15658);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30944);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36347);
											match s {
												[] => true,
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
					super::push(out, 15659);
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
		['е', s @ ..] => {
			super::push(out, 8290);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15660);
					match s {
						[] => true,
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
			super::push(out, 8291);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15661);
					match s {
						[] => true,
						['у', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30945);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30946);
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
		['о', s @ ..] => {
			super::push(out, 8292);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23475);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
			super::push(out, 8293);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15663);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30947);
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30948);
									match s {
										[] => true,
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
		['ы', s @ ..] => {
			super::push(out, 8294);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15664);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23476);
							match s {
								[] => true,
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 15665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23477);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23478);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30949);
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
							super::push(out, 23479);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30950);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 30951);
									match s {
										[] => true,
										['н', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 23480);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30952);
									match s {
										[] => true,
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

/// Words ending -йданить.
pub(crate) fn n10030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8295);
	match s {
		[] => true,
		['а', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -занить.
pub(crate) fn n3226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3036);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8296);
			match s {
				[] => true,
				['т', 'р', s @ ..] => {
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30953);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 36348);
											match s {
												[] => true,
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
						['у', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8297);
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

/// Words ending -ианить.
pub(crate) fn n3227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3037);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 8298);
			match s {
				[] => true,
				['а', 'т', 'е', 'г', 'е', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -канить.
pub(crate) fn n3228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3038);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8299);
			s.is_empty()
		}
		['а', 'т', 'с', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8300);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23481);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23482);
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
							super::push(out, 23483);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23484);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30954);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36349);
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
							super::push(out, 23485);
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
						['т', s @ ..] => {
							super::push(out, 23486);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 23487);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30955);
									match s {
										[] => true,
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
			super::push(out, 8301);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15666);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23488);
							match s {
								[] => true,
								['л', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8302);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23489);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 8303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15667);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15668);
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
		['ш', s @ ..] => {
			super::push(out, 8304);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15669);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23490);
							match s {
								[] => true,
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

/// Words ending -ланить.
pub(crate) fn n3229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3039);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8305);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8306);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23491);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23492);
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
		['з', s @ ..] => {
			super::push(out, 8307);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15671);
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
		['к', s @ ..] => {
			super::push(out, 8308);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15672);
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
		['р', s @ ..] => {
			super::push(out, 8309);
			match s {
				[] => true,
				['о', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23493);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23494);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30956);
									match s {
										[] => true,
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
							super::push(out, 23495);
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
		_ => false,
	}
}

/// Words ending -манить.
pub(crate) fn n3230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3040);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8310);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8311);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15675);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23496);
							match s {
								[] => true,
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
					super::push(out, 15676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23497);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 23498);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23499);
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
		['д', s @ ..] => {
			super::push(out, 8312);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15677);
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
			super::push(out, 8313);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15678);
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
			super::push(out, 8314);
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
		['и', s @ ..] => {
			super::push(out, 8315);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15679);
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
			super::push(out, 8316);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15680);
					match s {
						[] => true,
						['и', 'с', 'к', 'о', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15681);
					match s {
						[] => true,
						['т', 'и', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15682);
					match s {
						[] => true,
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
			super::push(out, 8317);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15683);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 15684);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30957);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30958);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'а', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8319);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15685);
					match s {
						[] => true,
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
			super::push(out, 8320);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15686);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23500);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23501);
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
							super::push(out, 23502);
							match s {
								[] => true,
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
			super::push(out, 8321);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15688);
					match s {
						[] => true,
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

/// Words ending -панить.
pub(crate) fn n3231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3041);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8322);
			match s {
				[] => true,
				['л', 'р', 'о', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23503);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23504);
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

/// Words ending -ранить.
pub(crate) fn n3232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3042);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8323);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8324);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15689);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23505);
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
				['м', s @ ..] => {
					super::push(out, 15690);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23506);
							match s {
								[] => true,
								['т', 'а', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23507);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23508);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23509);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30959);
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
			super::push(out, 8325);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15692);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15693);
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
					super::push(out, 15694);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 15695);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15696);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23510);
							match s {
								[] => true,
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
			super::push(out, 8326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15697);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15698);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15699);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23511);
							match s {
								[] => true,
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
			super::push(out, 8327);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15700);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23512);
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
				['о', s @ ..] => {
					super::push(out, 15701);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23513);
							match s {
								[] => true,
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
			super::push(out, 8328);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15702);
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
			super::push(out, 8329);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15703);
					match s {
						[] => true,
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
			super::push(out, 8330);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23514);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23515);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23516);
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
			super::push(out, 8331);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15705);
					match s {
						[] => true,
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
			super::push(out, 8332);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15706);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8333);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 8334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15708);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
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

/// Words ending -санить.
pub(crate) fn n3233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3043);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8335);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танить.
pub(crate) fn n3234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3044);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8336);
			match s {
				[] => true,
				['л', 'р', 'а', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23517);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23518);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23519);
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
		['и', s @ ..] => {
			super::push(out, 8337);
			match s {
				[] => true,
				['к', 'р', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8338);
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

/// Words ending -фанить.
pub(crate) fn n3235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3045);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8339);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8340);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15710);
					match s {
						[] => true,
						['а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щанить.
pub(crate) fn n3236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3046);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8341);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
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

/// Words ending -ленить.
pub(crate) fn n3264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3073);
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8380);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15745);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23541);
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
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23542);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23543);
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
							super::push(out, 23544);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 23545);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 23546);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30961);
									match s {
										[] => true,
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
							super::push(out, 23547);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30962);
									match s {
										[] => true,
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
		['п', s @ ..] => {
			super::push(out, 8381);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 8382);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15746);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15747);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23548);
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
					super::push(out, 15748);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15749);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15750);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 15751);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23549);
							match s {
								[] => true,
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

/// Words ending -менить.
pub(crate) fn n3265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3074);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8383);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15752);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15753);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15754);
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8384);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8385);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8386);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8387);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15757);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23550);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30963);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30964);
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
						['в', s @ ..] => {
							super::push(out, 23551);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30965);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30966);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36350);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 36351);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 39773);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83321(s, out),
														_ => false,
													}
												}
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
							super::push(out, 23552);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23553);
							match s {
								[] => true,
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
					super::push(out, 15758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23554);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23555);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 23556);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30968);
									match s {
										[] => true,
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
							super::push(out, 23557);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23558);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30969);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30970);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30971);
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
		['з', s @ ..] => {
			super::push(out, 8388);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23559);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30972);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 36352);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 30973);
									match s {
										[] => true,
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
			super::push(out, 8389);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15760);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23560);
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
					super::push(out, 15761);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23561);
							match s {
								[] => true,
								['о', 'п', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8391);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8392);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15763);
					match s {
						[] => true,
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

/// Words ending -пенить.
pub(crate) fn n3266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3075);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8393);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8394);
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
			super::push(out, 8395);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15764);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23562);
							match s {
								[] => true,
								['ъ', 'з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15765);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 15766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23563);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23564);
							match s {
								[] => true,
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
			super::push(out, 8396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8397);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15767);
					match s {
						[] => true,
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

/// Words ending -ренить.
pub(crate) fn n3267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3076);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 8398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15768);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15769);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23565);
							match s {
								[] => true,
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
					super::push(out, 15770);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23566);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8399);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15771);
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
		['т', s @ ..] => {
			super::push(out, 8400);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15772);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15773);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23567);
							match s {
								[] => true,
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

/// Words ending -сенить.
pub(crate) fn n3268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3077);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8401);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8402);
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

/// Words ending -тенить.
pub(crate) fn n3269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3078);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8403);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8404);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15774);
					match s {
						[] => true,
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
			super::push(out, 8405);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15775);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23568);
							match s {
								[] => true,
								['ю', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36353);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
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
				['т', s @ ..] => {
					super::push(out, 15776);
					match s {
						[] => true,
						['р', 'е', 'в', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'б', 'з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8406);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8408);
			match s {
				[] => true,
				['о', 'к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8409);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15777);
					match s {
						[] => true,
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

/// The word фенить.
pub(crate) fn n10157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8410);
	s.is_empty()
}

/// Words ending -ценить.
pub(crate) fn n3271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3080);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8411);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8412);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15779);
					match s {
						[] => true,
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
			super::push(out, 8413);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15780);
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
		['и', s @ ..] => {
			super::push(out, 8414);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8415);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15781);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15782);
					match s {
						[] => true,
						['д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15783);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23569);
							match s {
								[] => true,
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
					super::push(out, 15784);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23570);
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
		['у', s @ ..] => {
			super::push(out, 8417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченить.
pub(crate) fn n3272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3081);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8418);
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

/// Words ending -щенить.
pub(crate) fn n3273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3082);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8419);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word продразнить.
pub(crate) fn n76823(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -линить.
pub(crate) fn n3285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3094);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8438);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15803);
					match s {
						[] => true,
						['ф', 'а', 'н', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8439);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8440);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15805);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15806);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23586);
							match s {
								[] => true,
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
					super::push(out, 15807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15808);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23588);
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
					super::push(out, 15809);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23589);
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
				['с', s @ ..] => {
					super::push(out, 15810);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23590);
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
					super::push(out, 15811);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23591);
							match s {
								[] => true,
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
			super::push(out, 8441);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15812);
					match s {
						[] => true,
						['р', 'б', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оинить.
pub(crate) fn n3286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3095);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 8442);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15813);
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

/// Words ending -ринить.
pub(crate) fn n3287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3096);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 8443);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23592);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23593);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23594);
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

/// Words ending -синить.
pub(crate) fn n3288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3097);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8444);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8445);
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8446);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15814);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23595);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30987);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23596);
							match s {
								[] => true,
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
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8447);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15815);
					match s {
						[] => true,
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

/// Words ending -тинить.
pub(crate) fn n3289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3098);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8448);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 15816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23597);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23598);
							match s {
								[] => true,
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
			super::push(out, 8449);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15817);
					match s {
						[] => true,
						['р', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30988);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30989);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36356);
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
			super::push(out, 8450);
			match s {
				[] => true,
				['к', 'с', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уинить.
pub(crate) fn n3290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3099);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 8451);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -финить.
pub(crate) fn n3291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3100);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8452);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15818);
					match s {
						[] => true,
						['а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чинить.
pub(crate) fn n3292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3101);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8453);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8454);
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
			super::push(out, 8455);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15819);
					match s {
						[] => true,
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
			super::push(out, 8456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8457);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['е', s @ ..] => {
			super::push(out, 8458);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15820);
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
		['и', s @ ..] => {
			super::push(out, 8459);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15821);
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
			super::push(out, 8460);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15822);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15825);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23599);
							match s {
								[] => true,
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
					super::push(out, 15826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8461);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15827);
					match s {
						[] => true,
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
			super::push(out, 8462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15828);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15829);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23600);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30990);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30991);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36357);
											match s {
												[] => true,
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
									super::push(out, 30992);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36358);
											match s {
												[] => true,
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
		['ы', s @ ..] => {
			super::push(out, 8463);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15830);
					match s {
						[] => true,
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

/// Words ending -шинить.
pub(crate) fn n3293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3102);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8465);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23601);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23602);
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

/// Words ending -щинить.
pub(crate) fn n3294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3103);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 8466);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23603);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23604);
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

/// The word перенаполнить.
pub(crate) fn n76827(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перезвонить.
pub(crate) fn n76828(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word затрезвонить.
pub(crate) fn n76829(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word натрезвонить.
pub(crate) fn n76830(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -астрезвонить.
pub(crate) fn n76831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39774);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оттрезвонить.
pub(crate) fn n76832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39775);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -здонить.
pub(crate) fn n10234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8480);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15862);
			match s {
				[] => true,
				['п', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдонить.
pub(crate) fn n10235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8481);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 15863);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30998);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 30999);
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

/// Words ending -удонить.
pub(crate) fn n10236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8482);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15864);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23627);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23628);
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

/// Words ending -жонить.
pub(crate) fn n3307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3116);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8483);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15865);
					match s {
						[] => true,
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

/// Words ending -зонить.
pub(crate) fn n3308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3117);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8484);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23629);
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23630);
							match s {
								[] => true,
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
					super::push(out, 15867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ионить.
pub(crate) fn n3309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3118);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 8485);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23632);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23633);
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
							super::push(out, 23634);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -конить.
pub(crate) fn n3310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3119);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8486);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15868);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'д', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8487);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15869);
					match s {
						[] => true,
						['р', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8488);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 15870);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лонить.
pub(crate) fn n3311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3120);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8489);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 15871);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23635);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31000);
									match s {
										[] => true,
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
		['к', s @ ..] => {
			super::push(out, 8490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15872);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15873);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23636);
							match s {
								[] => true,
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
					super::push(out, 15874);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23637);
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
					super::push(out, 15875);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23638);
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
				['и', s @ ..] => {
					super::push(out, 15876);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 23639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15877);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23640);
							match s {
								[] => true,
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
					super::push(out, 15878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15879);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23641);
							match s {
								[] => true,
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
					super::push(out, 15880);
					match s {
						[] => true,
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
			super::push(out, 8491);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15881);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23642);
							match s {
								[] => true,
								['д', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36365);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36366);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 39776);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83324(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23643);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23644);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23645);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23646);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15884);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23647);
							match s {
								[] => true,
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
			super::push(out, 8492);
			match s {
				[] => true,
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -монить.
pub(crate) fn n3312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3121);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8493);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15885);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23648);
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 8494);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15886);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23649);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23650);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31001);
									match s {
										[] => true,
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
							super::push(out, 23651);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31002);
									match s {
										[] => true,
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
							super::push(out, 23652);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31003);
									match s {
										[] => true,
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
							super::push(out, 23653);
							match s {
								[] => true,
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

/// Words ending -нонить.
pub(crate) fn n3313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3122);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8495);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15887);
					match s {
						[] => true,
						['д', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -понить.
pub(crate) fn n3314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3123);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8496);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23654);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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

/// Words ending -ронить.
pub(crate) fn n3315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3124);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8497);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8498);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 15889);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23655);
							match s {
								[] => true,
								['а', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36367);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36368);
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
		['б', s @ ..] => {
			super::push(out, 8499);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15890);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23656);
							match s {
								[] => true,
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
					super::push(out, 15891);
					match s {
						[] => true,
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
			super::push(out, 8500);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15892);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 23657);
							match s {
								[] => true,
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
			super::push(out, 8501);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15893);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23658);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23659);
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
							super::push(out, 23660);
							match s {
								[] => true,
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
						['о', s @ ..] => {
							super::push(out, 23661);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23662);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31004);
									match s {
										[] => true,
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
					super::push(out, 15894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23663);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23664);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31005);
									match s {
										[] => true,
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
							super::push(out, 23665);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31006);
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
				['р', s @ ..] => {
					super::push(out, 15895);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23666);
							match s {
								[] => true,
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
					super::push(out, 15896);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23667);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31007);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31008);
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
				['х', s @ ..] => {
					super::push(out, 15897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23668);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23669);
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
							super::push(out, 23670);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 8502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8503);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15898);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23672);
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
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8504);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 8505);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15899);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23673);
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
		['ы', s @ ..] => super::t20::n10260(s, out),
		_ => false,
	}
}

/// Words ending -сонить.
pub(crate) fn n3316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3125);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8507);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 15901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23674);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23675);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31009);
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
							super::push(out, 23676);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31010);
									match s {
										[] => true,
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
		['к', s @ ..] => {
			super::push(out, 8508);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15902);
					match s {
						[] => true,
						['л', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонить.
pub(crate) fn n3317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3126);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8509);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'а', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8510);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15903);
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
		['и', s @ ..] => {
			super::push(out, 8511);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15904);
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
			super::push(out, 8512);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15905);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23677);
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
				['ь', s @ ..] => {
					super::push(out, 15906);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23678);
							match s {
								[] => true,
								['р', 'а', 'ч', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8513);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фонить.
pub(crate) fn n3318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3127);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8514);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8515);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15907);
					match s {
						[] => true,
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
			super::push(out, 8516);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15908);
					match s {
						[] => true,
						['е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8517);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23679);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31011);
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
		['ф', s @ ..] => {
			super::push(out, 8518);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15910);
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

/// Words ending -ьонить.
pub(crate) fn n3319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3128);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8519);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15911);
					match s {
						[] => true,
						['т', 'ч', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word повытеснить.
pub(crate) fn n76835(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -еобезьянить.
pub(crate) fn n52673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31019);
	match s {
		[] => true,
		['р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -собезьянить.
pub(crate) fn n52674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31020);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -льянить.
pub(crate) fn n10324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8560);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15958);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23712);
					match s {
						[] => true,
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

/// Words ending -пьянить.
pub(crate) fn n10325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8561);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15959);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тьянить.
pub(crate) fn n10326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8562);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 15961);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23713);
					match s {
						[] => true,
						['р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15962);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23714);
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

/// Words ending -ищепить.
pub(crate) fn n10452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8639);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 16018);
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

/// Words ending -сщепить.
pub(crate) fn n10453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8640);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16019);
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

/// Words ending -тщепить.
pub(crate) fn n10454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8641);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыщепить.
pub(crate) fn n10455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8642);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16021);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сопить.
pub(crate) fn n3422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3230);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8672);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23733);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23734);
							match s {
								[] => true,
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

/// Words ending -топить.
pub(crate) fn n3423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3231);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8673);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8674);
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
		['в', s @ ..] => {
			super::push(out, 8675);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8676);
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
			super::push(out, 8677);
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
			super::push(out, 8678);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
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
		['у', s @ ..] => {
			super::push(out, 8679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8680);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16044);
					match s {
						[] => true,
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

/// Words ending -шопить.
pub(crate) fn n3424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3232);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8681);
			match s {
				[] => true,
				['т', 'о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23735);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23736);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23737);
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

/// Words ending -рупить.
pub(crate) fn n3436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3244);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8706);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 16060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23744);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23745);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -супить.
pub(crate) fn n3437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3245);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8707);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8708);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16061);
					match s {
						[] => true,
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

/// Words ending -тупить.
pub(crate) fn n3438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3246);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8709);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8710);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8711);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16063);
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
			super::push(out, 8712);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16064);
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
		['и', s @ ..] => {
			super::push(out, 8713);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16065);
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
			super::push(out, 8714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16066);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16068);
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
			super::push(out, 8715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16069);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23749);
							match s {
								[] => true,
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
					super::push(out, 16070);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23750);
							match s {
								[] => true,
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
					super::push(out, 16071);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 16072);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23751);
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
					super::push(out, 16073);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23752);
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
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23753);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23754);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31029);
									match s {
										[] => true,
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
					super::push(out, 16074);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 23755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23756);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23758);
							match s {
								[] => true,
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
					super::push(out, 16076);
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
				['ы', s @ ..] => {
					super::push(out, 16077);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23759);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -афигарить.
pub(crate) fn n22268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16099);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

