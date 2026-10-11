use flexion::Path;

/// Words ending -зведываться.
pub(crate) fn n69032(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -оведываться.
pub(crate) fn n69033(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'и', '\u{1}', s @ ..] => {
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

/// The word выведываться.
pub(crate) fn n69035(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -здываться.
pub(crate) fn n40813(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'в', 'г', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'г', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
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

/// Words ending -идываться.
pub(crate) fn n40814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25416);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36995);
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
				['ы', 'в', s @ ..] => {
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

/// Words ending -ядываться.
pub(crate) fn n40815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25417);
	match s {
		[] => true,
		['л', 'г', s @ ..] => {
			match s {
				[] => true,
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
				['е', s @ ..] => {
					super::push(out, 36996);
					match s {
						[] => true,
						['р', 'е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42054);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42989);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'п', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word вырезываться.
pub(crate) fn n69084(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -изываться.
pub(crate) fn n40821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25423);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 32042);
			match s {
				[] => true,
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 32043);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32044);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -палзываться.
pub(crate) fn n40822(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -озываться.
pub(crate) fn n40823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25424);
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
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

/// Words ending -ерзываться.
pub(crate) fn n40824(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ъ', 'з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сзываться.
pub(crate) fn n40825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25425);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тзываться.
pub(crate) fn n40826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25426);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ызываться.
pub(crate) fn n40827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25427);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32046);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 32047);
			match s {
				[] => true,
				['г', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оскальзываться.
pub(crate) fn n40828(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -язываться.
pub(crate) fn n40829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25428);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 32048);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 32049);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
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
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word поприкалываться.
pub(crate) fn n89263(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -делываться.
pub(crate) fn n55666(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
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
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word напридумываться.
pub(crate) fn n89264(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ымываться.
pub(crate) fn n40846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25442);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32065);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word дооткапываться.
pub(crate) fn n89265(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -склепываться.
pub(crate) fn n79343(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -шлепываться.
pub(crate) fn n69201(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трепываться.
pub(crate) fn n55699(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
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
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щипываться.
pub(crate) fn n40852(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
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
				['д', '\u{1}', s @ ..] => {
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
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -опываться.
pub(crate) fn n40853(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
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
		_ => false,
	}
}

/// Words ending -рпываться.
pub(crate) fn n40854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25443);
	match s {
		[] => true,
		['а', 'ш', s @ ..] => {
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
		['е', 'ч', s @ ..] => {
			match s {
				[] => true,
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
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
				['с', s @ ..] => {
					super::push(out, 37008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40303);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42059);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упываться.
pub(crate) fn n40855(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япываться.
pub(crate) fn n40856(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
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

/// Words ending -крываться.
pub(crate) fn n40865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25452);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32078);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40305);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
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
			super::push(out, 32079);
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
						['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
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
		['т', s @ ..] => {
			super::push(out, 32080);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40306);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42061);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отрываться.
pub(crate) fn n55744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32082);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 37011);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 40307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word урываться.
pub(crate) fn n55745(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ырываться.
pub(crate) fn n40870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25457);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32083);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word повыбрасываться.
pub(crate) fn n89266(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word зачесываться.
pub(crate) fn n79441(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word начесываться.
pub(crate) fn n79442(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word обчесываться.
pub(crate) fn n69326(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word подчесываться.
pub(crate) fn n69327(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перечесываться.
pub(crate) fn n69328(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -очесываться.
pub(crate) fn n69329(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -счесываться.
pub(crate) fn n69330(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word отчесываться.
pub(crate) fn n69331(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word учесываться.
pub(crate) fn n69332(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word вычесываться.
pub(crate) fn n69333(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -писываться.
pub(crate) fn n40873(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', s @ ..] => {
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
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'п', '\u{1}', s @ ..] => {
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кусываться.
pub(crate) fn n40874(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
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
		['д', s @ ..] => {
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ясываться.
pub(crate) fn n40875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25459);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 32084);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40308);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 42063);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32085);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['о', '\u{1}', s @ ..] => {
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
		['р', 'т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word окатываться.
pub(crate) fn n79478(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word докатываться.
pub(crate) fn n79479(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word покатываться.
pub(crate) fn n79480(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word прокатываться.
pub(crate) fn n79481(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -скатываться.
pub(crate) fn n69371(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word откатываться.
pub(crate) fn n69372(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word укатываться.
pub(crate) fn n69373(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выкатываться.
pub(crate) fn n69374(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -латываться.
pub(crate) fn n55777(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -матываться.
pub(crate) fn n55778(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хлопатываться.
pub(crate) fn n55779(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хохатываться.
pub(crate) fn n55780(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -печатываться.
pub(crate) fn n55781(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
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
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шатываться.
pub(crate) fn n55782(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етываться.
pub(crate) fn n40877(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
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
		['м', s @ ..] => {
			match s {
				[] => true,
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
				['с', '\u{1}', s @ ..] => {
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

/// Words ending -итываться.
pub(crate) fn n40878(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', 'к', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'о', 'в', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
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
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -балтываться.
pub(crate) fn n40879(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -работываться.
pub(crate) fn n40880(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птываться.
pub(crate) fn n40881(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'т', s @ ..] => {
			match s {
				[] => true,
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'ш', s @ ..] => {
			match s {
				[] => true,
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ертываться.
pub(crate) fn n40882(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стываться.
pub(crate) fn n40883(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', 'а', 'р', 'г', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'л', 'х', s @ ..] => {
			match s {
				[] => true,
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['р', 'е', 'в', s @ ..] => {
			match s {
				[] => true,
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'л', 'х', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утываться.
pub(crate) fn n40884(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пытываться.
pub(crate) fn n40885(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', 'и', s @ ..] => {
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -прятываться.
pub(crate) fn n40886(s: &[char], _out: &mut Path<'_>) -> bool {
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -занездоровиться.
pub(crate) fn n89269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43466);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word облагородиться.
pub(crate) fn n89271(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -слородиться.
pub(crate) fn n71167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37482);
	match s {
		[] => true,
		['и', 'к', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -породиться.
pub(crate) fn n58265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33015);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тродиться.
pub(crate) fn n42984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26607);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33016);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уродиться.
pub(crate) fn n42985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26608);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыродиться.
pub(crate) fn n42986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26609);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 33017);
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

/// The word проканителиться.
pub(crate) fn n89273(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -разъерепениться.
pub(crate) fn n89274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43467);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word взъерепениться.
pub(crate) fn n89275(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word честиться.
pub(crate) fn n60219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34029);
	s.is_empty()
}

/// Words ending -счеститься.
pub(crate) fn n60220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34030);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 38105);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42136);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word занеистовствовать.
pub(crate) fn n89278(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word посочувствовать.
pub(crate) fn n89279(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поруководствовать.
pub(crate) fn n89280(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рдствовать.
pub(crate) fn n47630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28847);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34678);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', 'л', 'и', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42178);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42179);
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
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42180);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 42181);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											super::push(out, 43013);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 42182);
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

/// Words ending -облудствовать.
pub(crate) fn n62781(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38459);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40776);
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
		['к', 'ы', 'з', 'я', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word злыдствовать.
pub(crate) fn n62783(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word бесстыдствовать.
pub(crate) fn n62784(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ядствовать.
pub(crate) fn n47633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28850);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34679);
			match s {
				[] => true,
				['н', 'у', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34680);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word заторжествовать.
pub(crate) fn n89281(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word восторжествовать.
pub(crate) fn n89282(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word снохачествовать.
pub(crate) fn n85650(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ечествовать.
pub(crate) fn n62794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34688);
	match s {
		[] => true,
		['д', 'о', 'л', 'о', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'у', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ичествовать.
pub(crate) fn n62795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34689);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38473);
			match s {
				[] => true,
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 38474);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40783);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 40784);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42188);
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
				['и', s @ ..] => {
					super::push(out, 40785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 38475);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40786);
					match s {
						[] => true,
						['е', 'п', 'о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 40787);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42189);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40788);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42190);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 43016);
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
							super::push(out, 42191);
							match s {
								[] => true,
								['о', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 42192);
							match s {
								[] => true,
								['е', 'м', 'в', 'о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчествовать.
pub(crate) fn n62796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34690);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 38476);
			match s {
				[] => true,
				['д', 'а', 'л', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очествовать.
pub(crate) fn n62797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34691);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 38477);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42193);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 42194);
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
		['п', s @ ..] => {
			super::push(out, 38478);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38479);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40791);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43017);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43018);
									match s {
										[] => true,
										['ж', 'л', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 38480);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 40792);
					match s {
						[] => true,
						['б', 'з', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 43468);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43706);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90221(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 43469);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычествовать.
pub(crate) fn n62798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34692);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 38481);
			match s {
				[] => true,
				['а', 'л', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шествовать.
pub(crate) fn n47637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28854);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 34693);
			match s {
				[] => true,
				['г', 'р', 'о', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 38482);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40793);
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
				['п', 'о', 'б', 'е', 'л', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 34694);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38483);
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
			super::push(out, 34695);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38484);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40794);
							match s {
								[] => true,
								['о', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43019);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 43470);
													match s {
														[] => true,
														['а', 'л', 'б', '\u{1}', s @ ..] => super::t32::n89936(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 43020);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38485);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40795);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43021);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43022);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43471);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89937(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 43023);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 38486);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40796);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34696);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38487);
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
		['о', s @ ..] => {
			super::push(out, 34697);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38488);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40797);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43024);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43472);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89938(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38489);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 40798);
							match s {
								[] => true,
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
					super::push(out, 38490);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40799);
							match s {
								[] => true,
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
					super::push(out, 38491);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40800);
							match s {
								[] => true,
								['м', 'о', 'к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 40801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34698);
			match s {
				[] => true,
				['а', 'и', 'р', 'т', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34699);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38492);
					match s {
						[] => true,
						['м', 'у', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42196);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42197);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43025);
											match s {
												[] => true,
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
									super::push(out, 42198);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43026);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t32::n62807(s, out),
		_ => false,
	}
}

/// Words ending -ществовать.
pub(crate) fn n47638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28855);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34701);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38496);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40804);
							match s {
								[] => true,
								['в', 'о', 'т', 'о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34702);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38497);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40805);
							match s {
								[] => true,
								['п', 'с', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34703);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38498);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40806);
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
				['с', s @ ..] => {
					super::push(out, 38499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42204);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43028);
											match s {
												[] => true,
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
									super::push(out, 42205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word походатайствовать.
pub(crate) fn n89303(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -исходатайствовать.
pub(crate) fn n89304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43478);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одействовать.
pub(crate) fn n74268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38507);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 40812);
			match s {
				[] => true,
				['ю', 'л', 'е', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 43031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 40813);
			match s {
				[] => true,
				['и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 40814);
			match s {
				[] => true,
				['у', 'ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 40815);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 40816);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42208);
					match s {
						[] => true,
						['а', 'з', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43479);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43710);
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
		['н', s @ ..] => {
			super::push(out, 40817);
			match s {
				[] => true,
				['н', 'е', 'щ', 'я', 'в', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 40818);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40819);
			match s {
				[] => true,
				['а', 'ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 40820);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42209);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 42210);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 43032);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 40821);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42211);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43033);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ойствовать.
pub(crate) fn n47645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28862);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34712);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38510);
					match s {
						[] => true,
						['т', 'у', 'н', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 34713);
			match s {
				[] => true,
				['е', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40824);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уйствовать.
pub(crate) fn n47646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28863);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38511);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40825);
							match s {
								[] => true,
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
					super::push(out, 38512);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40826);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 40827);
							match s {
								[] => true,
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
					super::push(out, 38513);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34715);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38514);
					match s {
						[] => true,
						['л', 'п', 'о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34716);
			match s {
				[] => true,
				['о', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юйствовать.
pub(crate) fn n47647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28864);
	match s {
		[] => true,
		['л', 'п', 'о', 'т', 'с', 'и', 'ч', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яйствовать.
pub(crate) fn n47648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28865);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 34717);
			match s {
				[] => true,
				['з', 'и', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'е', 'п', 'у', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'о', 'х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38515);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40829);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word зашаманствовать.
pub(crate) fn n89310(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word застранствовать.
pub(crate) fn n89311(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word пространствовать.
pub(crate) fn n89312(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -денствовать.
pub(crate) fn n62857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34733);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38533);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40839);
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
		['о', s @ ..] => {
			super::push(out, 38534);
			match s {
				[] => true,
				['г', 'л', 'о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женствовать.
pub(crate) fn n62858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34734);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38535);
			match s {
				[] => true,
				['л', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42221);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43037);
									match s {
										[] => true,
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
							super::push(out, 42222);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43038);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 38536);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ленствовать.
pub(crate) fn n62859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34735);
	match s {
		[] => true,
		['ч', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -менствовать.
pub(crate) fn n62860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34736);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 38537);
			match s {
				[] => true,
				['е', 'н', 'з', 'и', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 38538);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40841);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенствовать.
pub(crate) fn n62861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34737);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 38539);
			match s {
				[] => true,
				['ю', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенствовать.
pub(crate) fn n62862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34738);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 38540);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40842);
					match s {
						[] => true,
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
			super::push(out, 38541);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40843);
					match s {
						[] => true,
						['в', 'о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 43039);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенствовать.
pub(crate) fn n62863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34739);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 38542);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 40844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 38543);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40845);
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

/// Words ending -знствовать.
pub(crate) fn n47656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28873);
	match s {
		[] => true,
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инствовать.
pub(crate) fn n47657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28874);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 34740);
			match s {
				[] => true,
				['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 34741);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 38544);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40846);
							match s {
								[] => true,
								['р', 'а', 'т', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'а', 'т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онствовать.
pub(crate) fn n47658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28875);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 34742);
			match s {
				[] => true,
				['и', 'п', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 34743);
			match s {
				[] => true,
				['а', 'л', 'е', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', 'г', 'а', 'п', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34744);
			match s {
				[] => true,
				['л', 'е', 'м', 'а', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 34745);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 40847);
							match s {
								[] => true,
								['е', 'ч', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 40848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34746);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38545);
					match s {
						[] => true,
						['и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 38546);
					match s {
						[] => true,
						['ь', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42223);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42224);
									match s {
										[] => true,
										['п', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34747);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40849);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									match s {
										[] => true,
										['к', 'з', 'и', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['л', 'о', 'д', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 40850);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42225);
									match s {
										[] => true,
										['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34748);
			match s {
				[] => true,
				['а', 'ф', 'н', 'а', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 34749);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38547);
					match s {
						[] => true,
						['к', 'и', 'м', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унствовать.
pub(crate) fn n47659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28876);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 34750);
			match s {
				[] => true,
				['е', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34751);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38548);
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
		['щ', s @ ..] => {
			super::push(out, 34752);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40851);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42226);
									match s {
										[] => true,
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
							super::push(out, 40852);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42227);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янствовать.
pub(crate) fn n47660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28877);
	match s {
		[] => true,
		['ь', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38549);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40853);
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
				['л', s @ ..] => {
					super::push(out, 38550);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40854);
							match s {
								[] => true,
								['г', 'е', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40855);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40856);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 40857);
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
				['т', s @ ..] => {
					super::push(out, 38552);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40858);
							match s {
								[] => true,
								['е', 'р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word намытарствовать.
pub(crate) fn n89316(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -дрствовать.
pub(crate) fn n47666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28882);
	match s {
		[] => true,
		['о', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38573);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38574);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40870);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 40871);
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
		['у', s @ ..] => {
			super::push(out, 34764);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40872);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40873);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42230);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 42231);
									match s {
										[] => true,
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
							super::push(out, 40874);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 42232);
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
							super::push(out, 40875);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 40876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерствовать.
pub(crate) fn n47667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28883);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34765);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38575);
					match s {
						[] => true,
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
			super::push(out, 34766);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38576);
					match s {
						[] => true,
						['н', 'а', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 34767);
			match s {
				[] => true,
				['е', 'у', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 34768);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38577);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38578);
					match s {
						[] => true,
						['о', 'р', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38579);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40878);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 42233);
									match s {
										[] => true,
										['ж', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 40879);
							match s {
								[] => true,
								['а', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34769);
			match s {
				[] => true,
				['я', 'о', 'в', 'и', 'м', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 34770);
			match s {
				[] => true,
				['а', 'р', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 34771);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38580);
					match s {
						[] => true,
						['н', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38581);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 40880);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34772);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38582);
					match s {
						[] => true,
						['а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 34773);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38583);
					match s {
						[] => true,
						['ц', 'и', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38584);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40881);
							match s {
								[] => true,
								['ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34774);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38585);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40882);
							match s {
								[] => true,
								['ф', '\u{1}', s @ ..] => {
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
						['ж', s @ ..] => {
							super::push(out, 40883);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 42234);
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
						['р', s @ ..] => {
							super::push(out, 40884);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42235);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 42236);
									match s {
										[] => true,
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
							super::push(out, 40885);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 42237);
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
				['и', s @ ..] => {
					super::push(out, 38586);
					match s {
						[] => true,
						['р', 'т', 'к', 'о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'у', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 38587);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40886);
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
				['о', s @ ..] => {
					super::push(out, 38588);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40887);
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42238);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 43041);
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
								['с', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 43042);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 43480);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t32::n89945(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 43043);
											match s {
												[] => true,
												['и', 'м', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89946(s, out),
														['о', 'к', '\u{1}', s @ ..] => super::t32::n89947(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34775);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38589);
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
			super::push(out, 34776);
			match s {
				[] => true,
				['с', 'и', 'ж', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t32::n62907(s, out),
		['ф', s @ ..] => {
			super::push(out, 34778);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38596);
					match s {
						[] => true,
						['ш', s @ ..] => super::t32::n81619(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t32::n62909(s, out),
		['ш', s @ ..] => super::t32::n62910(s, out),
		['ь', s @ ..] => super::t32::n62911(s, out),
		_ => false,
	}
}

/// Words ending -жирствовать.
pub(crate) fn n62913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34783);
	match s {
		[] => true,
		['н', 'а', 'р', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пирствовать.
pub(crate) fn n62914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34784);
	match s {
		[] => true,
		['м', 'а', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тирствовать.
pub(crate) fn n62915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34785);
	match s {
		[] => true,
		['е', 'к', 'э', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'е', 'з', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ширствовать.
pub(crate) fn n62916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34786);
	match s {
		[] => true,
		['о', 'б', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орствовать.
pub(crate) fn n47669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28885);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34787);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38600);
					match s {
						[] => true,
						['н', 'и', 'д', 'е', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['х', 'о', 'р', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38601);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40895);
							match s {
								[] => true,
								['и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34788);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38602);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', 'ч', 'ю', 'р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 42241);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 42242);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 43045);
											match s {
												[] => true,
												['т', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34789);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38603);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 34790);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38604);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40897);
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
				['о', s @ ..] => {
					super::push(out, 38605);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 38606);
					match s {
						[] => true,
						['л', 'е', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 34791);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38607);
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
		['п', s @ ..] => {
			super::push(out, 34792);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 38608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40898);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42243);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34793);
			match s {
				[] => true,
				['н', 'о', 'п', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'е', 'ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34794);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38609);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40899);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42244);
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
						['з', s @ ..] => {
							super::push(out, 40900);
							match s {
								[] => true,
								['и', 'л', 'а', 'р', 'о', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 40901);
							match s {
								[] => true,
								['р', 'е', 'б', 'у', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 42245);
									match s {
										[] => true,
										['п', 'м', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', 'и', 'л', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 42246);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43046);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43481);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89948(s, out),
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
									super::push(out, 42247);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 43047);
											match s {
												[] => true,
												['и', s @ ..] => {
													match s {
														[] => true,
														['г', 'е', 'р', '\u{1}', s @ ..] => super::t32::n89949(s, out),
														['н', 'и', 'м', 'д', 'а', '\u{1}', s @ ..] => super::t32::n89950(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t32::n81640(s, out),
						_ => false,
					}
				}
				['в', 'а', s @ ..] => super::t32::n74418(s, out),
				['и', s @ ..] => super::t32::n74419(s, out),
				['к', s @ ..] => super::t32::n74420(s, out),
				['н', s @ ..] => super::t32::n74421(s, out),
				['я', s @ ..] => super::t32::n74422(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урствовать.
pub(crate) fn n47670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28886);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 34795);
			match s {
				[] => true,
				['а', 'л', 'а', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 34796);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34797);
			match s {
				[] => true,
				['а', 'р', 'е', 'т', 'и', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырствовать.
pub(crate) fn n47671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28887);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 34798);
			match s {
				[] => true,
				['с', 'а', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40908);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 42254);
									match s {
										[] => true,
										['л', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 40909);
							match s {
								[] => true,
								['х', 'р', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эрствовать.
pub(crate) fn n47672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28888);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 34799);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярствовать.
pub(crate) fn n47673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28889);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38614);
					match s {
						[] => true,
						['и', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38615);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40910);
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
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 34800);
			match s {
				[] => true,
				['л', 'и', 'м', 'а', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word поприветствовать.
pub(crate) fn n89325(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отприветствовать.
pub(crate) fn n89326(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word посибаритствовать.
pub(crate) fn n89327(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -антствовать.
pub(crate) fn n62950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34815);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 38626);
			match s {
				[] => true,
				['е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'к', 'е', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 38627);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40915);
					match s {
						[] => true,
						['е', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'ы', 'з', 'у', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 38628);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 40916);
					match s {
						[] => true,
						['ъ', 'д', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ентствовать.
pub(crate) fn n62951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34816);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 38629);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40917);
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
		['д', s @ ..] => {
			super::push(out, 38630);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40918);
					match s {
						[] => true,
						['к', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 40919);
					match s {
						[] => true,
						['з', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'с', 'и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 40920);
					match s {
						[] => true,
						['т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лянтствовать.
pub(crate) fn n74455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38631);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 40921);
			match s {
				[] => true,
				['в', 'к', 'с', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отствовать.
pub(crate) fn n47679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28895);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 34818);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38632);
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
		['и', s @ ..] => {
			super::push(out, 34819);
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38633);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 38634);
					match s {
						[] => true,
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
			super::push(out, 34820);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38635);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40922);
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
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38636);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 38637);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40923);
							match s {
								[] => true,
								['а', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38638);
					match s {
						[] => true,
						['к', 'н', 'о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'б', 'о', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 40924);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 42257);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утствовать.
pub(crate) fn n47680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28896);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 34821);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38639);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38640);
					match s {
						[] => true,
						['л', 'а', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 38641);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 34822);
			match s {
				[] => true,
				['и', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
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
		['т', s @ ..] => {
			super::push(out, 34823);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38642);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40926);
							match s {
								[] => true,
								['с', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытствовать.
pub(crate) fn n47681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28897);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 34824);
			match s {
				[] => true,
				['о', 'б', 'ю', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40927);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40928);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42258);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтствовать.
pub(crate) fn n47682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28898);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34825);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 38643);
					match s {
						[] => true,
						['с', 'н', 'о', 'к', 'с', 'и', 'р', 'ю', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятствовать.
pub(crate) fn n47683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28899);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 34826);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40929);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 40930);
							match s {
								[] => true,
								['ц', 'и', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'г', 'а', 'л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42259);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43053);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 34827);
			match s {
				[] => true,
				['е', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40931);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40932);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42260);
									match s {
										[] => true,
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
							super::push(out, 40933);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42261);
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

/// Words ending -ральствовать.
pub(crate) fn n74481(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40940);
			match s {
				[] => true,
				['н', 'е', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40941);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42263);
					match s {
						[] => true,
						['д', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хальствовать.
pub(crate) fn n74482(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40942);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 42264);
					match s {
						[] => true,
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
			super::push(out, 40943);
			match s {
				[] => true,
				['а', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чальствовать.
pub(crate) fn n74483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38650);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40944);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 43054);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43482);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ельствовать.
pub(crate) fn n62970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34831);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 38651);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40945);
					match s {
						[] => true,
						['б', 'е', 'л', 'х', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', 'а', 'д', 'о', 'п', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 42265);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 43055);
									match s {
										[] => true,
										['м', 'о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 42266);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 43711);
													match s {
														[] => true,
														['з', s @ ..] => super::t32::n90226(s, out),
														_ => false,
													}
												}
												['д', s @ ..] => {
													super::push(out, 43712);
													match s {
														[] => true,
														['е', 'р', 'п', s @ ..] => super::t32::n90227(s, out),
														_ => false,
													}
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
								['о', 'н', 'о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', 'д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 43056);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 43483);
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
						['к', s @ ..] => {
							super::push(out, 42267);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 43057);
									match s {
										[] => true,
										['и', 'о', 'д', 'а', 'л', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', 'е', 'ж', 'о', s @ ..] => super::t32::n85867(s, out),
						['м', 'и', 'н', 'и', 'р', 'п', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => super::t32::n85868(s, out),
						['р', 'а', 'т', 'с', '\u{1}', s @ ..] => super::t32::n85869(s, out),
						['с', s @ ..] => super::t32::n85870(s, out),
						['т', s @ ..] => super::t32::n85871(s, out),
						['х', 'ы', 'п', 'о', 'л', 'з', '\u{1}', s @ ..] => super::t32::n85872(s, out),
						['ш', 'а', 'л', 'г', 'о', 'с', '\u{1}', s @ ..] => super::t32::n85873(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t32::n81712(s, out),
				['и', s @ ..] => super::t32::n81713(s, out),
				['я', s @ ..] => super::t32::n81714(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ильствовать.
pub(crate) fn n62971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34832);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 38652);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40949);
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
		['ф', s @ ..] => {
			super::push(out, 38653);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42274);
							match s {
								[] => true,
								['ю', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 42275);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 43075);
									match s {
										[] => true,
										['н', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 42276);
							match s {
								[] => true,
								['я', 'в', 'а', 'л', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 42277);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 43076);
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
		_ => false,
	}
}

/// Words ending -ольствовать.
pub(crate) fn n62972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34833);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38654);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43077);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 43078);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43496);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 42278);
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
		['г', s @ ..] => {
			super::push(out, 38655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 38656);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40951);
					match s {
						[] => true,
						['о', 'п', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульствовать.
pub(crate) fn n62973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34834);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 38657);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 40952);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['х', s @ ..] => {
			super::push(out, 38658);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40953);
					match s {
						[] => true,
						['г', 'о', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word денатурализовать.
pub(crate) fn n89972(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ренатурализовать.
pub(crate) fn n89973(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word законсервировать.
pub(crate) fn n89376(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word расконсервировать.
pub(crate) fn n89377(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поностальгировать.
pub(crate) fn n89378(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word распропагандировать.
pub(crate) fn n89379(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -обмундировать.
pub(crate) fn n81966(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -фундировать.
pub(crate) fn n63459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35116);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38849);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 38850);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41047);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word импровизировать.
pub(crate) fn n89384(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -аимпровизировать.
pub(crate) fn n89385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43499);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 43723);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -симпровизировать.
pub(crate) fn n89386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43500);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сымпровизировать.
pub(crate) fn n89387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43501);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реяровизировать.
pub(crate) fn n88205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43096);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ояровизировать.
pub(crate) fn n86046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42335);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 43097);
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

/// Words ending -свизировать.
pub(crate) fn n63537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35169);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гизировать.
pub(crate) fn n48287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29283);
	match s {
		[] => true,
		['о', 'л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38887);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 41073);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 41074);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38888);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41075);
							match s {
								[] => true,
								['д', 'и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43098);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43502);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89977(s, out),
														_ => false,
													}
												}
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
							super::push(out, 41076);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ц', 'о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 41077);
							match s {
								[] => true,
								['э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'и', 'т', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42337);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 42338);
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
						['н', s @ ..] => {
							super::push(out, 41078);
							match s {
								[] => true,
								['и', 'м', 'р', 'е', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41079);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 42339);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 41080);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 42340);
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
						['ф', s @ ..] => {
							super::push(out, 41081);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 42341);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 43099);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 42342);
									match s {
										[] => true,
										['о', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 41082);
							match s {
								[] => true,
								['и', 'с', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дизировать.
pub(crate) fn n48288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29284);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38889);
					match s {
						[] => true,
						['а', 'в', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38890);
					match s {
						[] => true,
						['б', 'и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 35170);
			match s {
				[] => true,
				['о', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35171);
			match s {
				[] => true,
				['м', 'р', 'е', 'б', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еизировать.
pub(crate) fn n48289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29285);
	match s {
		[] => true,
		['п', 'о', 'р', 'в', 'е', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кизировать.
pub(crate) fn n48290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29286);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 35172);
			match s {
				[] => true,
				['ю', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лизировать.
pub(crate) fn n48291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29287);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 35173);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38891);
					match s {
						[] => true,
						['о', 'л', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 38892);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41083);
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
				['г', s @ ..] => {
					super::push(out, 38893);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41084);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 38894);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41085);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 41086);
							match s {
								[] => true,
								['а', 'к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38895);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41087);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 41088);
							match s {
								[] => true,
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
					super::push(out, 38896);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38897);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41089);
							match s {
								[] => true,
								['а', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'е', 'в', 'д', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 41090);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 42343);
									match s {
										[] => true,
										['с', 'у', 'д', 'н', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => super::t32::n82046(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t32::n74962(s, out),
				['м', s @ ..] => super::t32::n74963(s, out),
				['н', s @ ..] => super::t32::n74964(s, out),
				['п', s @ ..] => super::t32::n74965(s, out),
				['р', s @ ..] => super::t32::n74966(s, out),
				['с', s @ ..] => super::t32::n74967(s, out),
				['т', s @ ..] => super::t32::n74968(s, out),
				['у', s @ ..] => super::t32::n74969(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 35174);
			s.is_empty()
		}
		['е', s @ ..] => super::t32::n63547(s, out),
		['и', s @ ..] => super::t32::n63548(s, out),
		['л', s @ ..] => super::t32::n63549(s, out),
		['о', s @ ..] => super::t32::n63550(s, out),
		_ => false,
	}
}

/// Words ending -мизировать.
pub(crate) fn n48292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29288);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35179);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38917);
					match s {
						[] => true,
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38918);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41117);
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
		['е', s @ ..] => {
			super::push(out, 35180);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38919);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41118);
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
				['ф', s @ ..] => {
					super::push(out, 38920);
					match s {
						[] => true,
						['в', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35181);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38921);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38922);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41120);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41121);
							match s {
								[] => true,
								['н', 'а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43119);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43120);
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
				['с', s @ ..] => {
					super::push(out, 38923);
					match s {
						[] => true,
						['к', 'а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38924);
					match s {
						[] => true,
						['и', 'г', 'е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 38925);
					match s {
						[] => true,
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
			super::push(out, 35182);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38926);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 41122);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41123);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 42376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38927);
					match s {
						[] => true,
						['о', 'к', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38928);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 35183);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38929);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'г', 'л', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -низировать.
pub(crate) fn n48293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29289);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35184);
			match s {
				[] => true,
				['б', 'р', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 38930);
					match s {
						[] => true,
						['и', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ь', 'л', 'а', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'т', 'с', 'и', 'р', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 38931);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41124);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 43121);
											match s {
												[] => true,
												['м', 'а', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['ф', s @ ..] => {
											super::push(out, 43122);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 43512);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89991(s, out),
														_ => false,
													}
												}
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
							super::push(out, 41125);
							match s {
								[] => true,
								['у', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 43123);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43513);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89992(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38932);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41126);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'е', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 41127);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43124);
											s.is_empty()
										}
										['е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 38933);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 41128);
							match s {
								[] => true,
								['а', 'ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38934);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41129);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 42377);
									match s {
										[] => true,
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
							super::push(out, 41130);
							match s {
								[] => true,
								['э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38935);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41131);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41132);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 42378);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 38936);
					match s {
						[] => true,
						['е', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 42379);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 43125);
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
			super::push(out, 35185);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38937);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', 'о', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'н', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'о', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38938);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41133);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', 'м', 'ю', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41134);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 42380);
									match s {
										[] => true,
										['у', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'с', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35186);
			match s {
				[] => true,
				['а', 'р', 'к', 'у', s @ ..] => super::t32::n75015(s, out),
				['е', 'т', 'ю', 'л', '\u{1}', s @ ..] => super::t32::n75016(s, out),
				['л', s @ ..] => super::t32::n75017(s, out),
				['м', 'е', 'ф', '\u{1}', s @ ..] => super::t32::n75018(s, out),
				['т', s @ ..] => super::t32::n75019(s, out),
				['у', 'р', '\u{1}', s @ ..] => super::t32::n75020(s, out),
				['ш', 'а', 'м', '\u{1}', s @ ..] => super::t32::n75021(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t32::n63559(s, out),
		['р', s @ ..] => super::t32::n63560(s, out),
		['у', s @ ..] => super::t32::n63561(s, out),
		['х', s @ ..] => super::t32::n63562(s, out),
		['я', s @ ..] => super::t32::n63563(s, out),
		_ => false,
	}
}

/// Words ending -оизировать.
pub(crate) fn n48294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29290);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 35192);
			match s {
				[] => true,
				['е', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41149);
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

/// Words ending -пизировать.
pub(crate) fn n48295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29291);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 35193);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'е', 'р', 'е', 'т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ризировать.
pub(crate) fn n48296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29292);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35194);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38952);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41150);
							match s {
								[] => true,
								['е', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 38953);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 38954);
					match s {
						[] => true,
						['н', 'е', 'г', 'е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38955);
					match s {
						[] => true,
						['л', 'к', 'у', 'н', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38956);
					match s {
						[] => true,
						['л', 'о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38957);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41151);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 42387);
									match s {
										[] => true,
										['о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 42388);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 43131);
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
						['и', s @ ..] => {
							super::push(out, 41152);
							match s {
								[] => true,
								['л', 'и', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43132);
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
						['н', s @ ..] => {
							super::push(out, 41153);
							match s {
								[] => true,
								['е', 'в', 'н', 'и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43133);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43134);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 43135);
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35195);
			match s {
				[] => true,
				['б', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', 'ь', 'л', 'у', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['й', 'е', 'в', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 38958);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41154);
							match s {
								[] => true,
								['е', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38959);
					match s {
						[] => true,
						['е', 'т', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ю', 'ь', 'п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', 'у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', 'у', 'а', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35196);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38960);
					match s {
						[] => true,
						['а', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 38961);
					match s {
						[] => true,
						['е', 'т', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 38962);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38963);
					match s {
						[] => true,
						['а', 'в', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['й', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38964);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41155);
							match s {
								[] => true,
								['е', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43136);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43517);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n89996(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38965);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41156);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 41157);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42389);
									match s {
										[] => true,
										['р', 'т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41158);
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
				['ф', s @ ..] => {
					super::push(out, 38966);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41159);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 42390);
									match s {
										[] => true,
										['е', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35197);
			match s {
				[] => true,
				['е', s @ ..] => super::t32::n75059(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35198);
			match s {
				[] => true,
				['к', 'р', 'е', 'м', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'к', 'у', 'р', 'т', 'с', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t32::n63571(s, out),
		['я', s @ ..] => super::t32::n63572(s, out),
		_ => false,
	}
}

/// Words ending -сизировать.
pub(crate) fn n48297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29293);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 35201);
			match s {
				[] => true,
				['у', 'а', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тизировать.
pub(crate) fn n48298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29294);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35202);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38968);
					match s {
						[] => true,
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 42392);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43137);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43518);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t32::n89997(s, out),
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
									super::push(out, 42393);
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
						['х', s @ ..] => {
							super::push(out, 41162);
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
				['к', s @ ..] => {
					super::push(out, 38969);
					match s {
						[] => true,
						['и', 'л', 'и', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 38970);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41163);
							match s {
								[] => true,
								['р', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 41164);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 42394);
									match s {
										[] => true,
										['л', 'ф', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 42395);
									match s {
										[] => true,
										['т', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 42396);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43138);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
								['л', s @ ..] => {
									super::push(out, 42397);
									match s {
										[] => true,
										['б', 'о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43139);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43140);
											match s {
												[] => true,
												['м', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 43141);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 43519);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 42398);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 43142);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'л', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t32::n86187(s, out),
								['а', s @ ..] => super::t32::n86188(s, out),
								['к', 'а', s @ ..] => super::t32::n86189(s, out),
								_ => false,
							}
						}
						['м', s @ ..] => super::t32::n82184(s, out),
						['о', s @ ..] => super::t32::n82185(s, out),
						['р', s @ ..] => super::t32::n82186(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t32::n75069(s, out),
				['п', s @ ..] => super::t32::n75070(s, out),
				['р', s @ ..] => super::t32::n75071(s, out),
				['т', s @ ..] => super::t32::n75072(s, out),
				['ф', s @ ..] => super::t32::n75073(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t32::n63575(s, out),
		['и', s @ ..] => super::t32::n63576(s, out),
		['н', s @ ..] => super::t32::n63577(s, out),
		['о', s @ ..] => super::t32::n63578(s, out),
		['п', s @ ..] => super::t32::n63579(s, out),
		['р', s @ ..] => super::t32::n63580(s, out),
		['э', s @ ..] => super::t32::n63581(s, out),
		['ю', s @ ..] => super::t32::n63582(s, out),
		_ => false,
	}
}

/// Words ending -уизировать.
pub(crate) fn n48299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29295);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35210);
			match s {
				[] => true,
				['а', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -физировать.
pub(crate) fn n48300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29296);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 35211);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39001);
					match s {
						[] => true,
						['т', 'с', 'а', 'т', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 35212);
			match s {
				[] => true,
				['о', 'м', 'а', 'т', 'е', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хизировать.
pub(crate) fn n48301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29297);
	match s {
		[] => true,
		['р', 'а', 'р', 'е', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цизировать.
pub(crate) fn n48302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29298);
	match s {
		[] => true,
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39002);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 39003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шизировать.
pub(crate) fn n48303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29299);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35213);
			match s {
				[] => true,
				['ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35214);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 39004);
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

/// The word заштабелировать.
pub(crate) fn n89457(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -роэкстраполировать.
pub(crate) fn n89458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43528);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word проинтерполировать.
pub(crate) fn n89459(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word проконтролировать.
pub(crate) fn n89460(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перерегулировать.
pub(crate) fn n89461(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -зажестикулировать.
pub(crate) fn n89462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43529);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word рециркулировать.
pub(crate) fn n89463(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ькулировать.
pub(crate) fn n63728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35318);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 39098);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42445);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42446);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мулировать.
pub(crate) fn n48392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29384);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 35320);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 41241);
							match s {
								[] => true,
								['и', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39101);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 42447);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43161);
											match s {
												[] => true,
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
									super::push(out, 42448);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43162);
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
		['р', s @ ..] => {
			super::push(out, 35321);
			match s {
				[] => true,
				['о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41242);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42449);
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
							super::push(out, 41243);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41244);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 35322);
			match s {
				[] => true,
				['к', 'к', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41245);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 35323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39102);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нулировать.
pub(crate) fn n48393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35324);
			match s {
				[] => true,
				['р', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 35325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пулировать.
pub(crate) fn n48394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29386);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 35326);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 35327);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35328);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39104);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рулировать.
pub(crate) fn n48395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29387);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 35329);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сулировать.
pub(crate) fn n48396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29388);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 35330);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 41247);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 42450);
									match s {
										[] => true,
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
							super::push(out, 41248);
							match s {
								[] => true,
								['р', 'к', 'и', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тулировать.
pub(crate) fn n48397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29389);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 35331);
			match s {
				[] => true,
				['п', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41249);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42451);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35332);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39105);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фулировать.
pub(crate) fn n48398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29390);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35333);
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

/// The word закамуфлировать.
pub(crate) fn n89466(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word декамуфлировать.
pub(crate) fn n89467(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word переформировать.
pub(crate) fn n89470(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word дореформировать.
pub(crate) fn n89471(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word недореформировать.
pub(crate) fn n89472(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word дезинформировать.
pub(crate) fn n89473(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word проинформировать.
pub(crate) fn n89474(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word мизансценировать.
pub(crate) fn n89477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43532);
	s.is_empty()
}

/// Words ending -омизансценировать.
pub(crate) fn n89478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43533);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рминировать.
pub(crate) fn n63837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35409);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41289);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 41290);
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

/// Words ending -уминировать.
pub(crate) fn n63838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35410);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 39178);
			match s {
				[] => true,
				['и', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьминировать.
pub(crate) fn n63839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35411);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 39179);
			match s {
				[] => true,
				['у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юминировать.
pub(crate) fn n63840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35412);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 39180);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41291);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -синировать.
pub(crate) fn n48461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29450);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 35413);
			match s {
				[] => true,
				['р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинировать.
pub(crate) fn n48462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29451);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35414);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41292);
							match s {
								[] => true,
								['ж', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41293);
							match s {
								[] => true,
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
					super::push(out, 39181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39183);
					match s {
						[] => true,
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
			super::push(out, 35415);
			match s {
				[] => true,
				['ь', 'л', 'и', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 35416);
			match s {
				[] => true,
				['а', 'р', 'к', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 35417);
			match s {
				[] => true,
				['л', 'г', 'г', 'а', 'м', 'е', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уинировать.
pub(crate) fn n48463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29452);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 35418);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -финировать.
pub(crate) fn n48464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29453);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 35419);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41295);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42480);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35420);
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

/// Words ending -цинировать.
pub(crate) fn n48465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29454);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 35421);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41296);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43174);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43175);
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
							super::push(out, 41297);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42481);
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
		['ь', s @ ..] => {
			super::push(out, 35422);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39184);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 35423);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39185);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41298);
							match s {
								[] => true,
								['а', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шинировать.
pub(crate) fn n48466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29455);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -репозиционировать.
pub(crate) fn n89480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43534);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word фракционировать.
pub(crate) fn n89481(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -екционировать.
pub(crate) fn n82486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41306);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43178);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43535);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 43734);
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
				['л', s @ ..] => {
					super::push(out, 43179);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 42485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нкционировать.
pub(crate) fn n82487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41307);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42486);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 43180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42487);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 43181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 43537);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43735);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юционировать.
pub(crate) fn n75430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39193);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41309);
			match s {
				[] => true,
				['о', 'в', 'э', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 43182);
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
		_ => false,
	}
}

/// Words ending -йонировать.
pub(crate) fn n48475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29464);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35430);
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

/// Words ending -конировать.
pub(crate) fn n48476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29465);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 35431);
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

/// Words ending -лонировать.
pub(crate) fn n48477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29466);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35432);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 39194);
					match s {
						[] => true,
						['э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35433);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 39195);
					match s {
						[] => true,
						['э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 35434);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39196);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41310);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -монировать.
pub(crate) fn n48478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29467);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 35435);
			match s {
				[] => true,
				['а', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -понировать.
pub(crate) fn n48479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29468);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35436);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39197);
					match s {
						[] => true,
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
			super::push(out, 35437);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39198);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42488);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42489);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43183);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39199);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39200);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42490);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42491);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35438);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41311);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42492);
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
		['с', s @ ..] => {
			super::push(out, 35439);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39202);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 39203);
					match s {
						[] => true,
						['э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42493);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43184);
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
				['н', s @ ..] => {
					super::push(out, 39204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35440);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39205);
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
		['ш', s @ ..] => {
			super::push(out, 35441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронировать.
pub(crate) fn n48480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29469);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39206);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 39207);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 41312);
							match s {
								[] => true,
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
					super::push(out, 39208);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 39209);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41313);
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
		['д', s @ ..] => {
			super::push(out, 35443);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39210);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42494);
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
			super::push(out, 35444);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39211);
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
		['х', s @ ..] => {
			super::push(out, 35445);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39212);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сонировать.
pub(crate) fn n48481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29470);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35446);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 39213);
					match s {
						[] => true,
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
			super::push(out, 35447);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39215);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41314);
							match s {
								[] => true,
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
					super::push(out, 39216);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонировать.
pub(crate) fn n48482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29471);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 35448);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 35449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39217);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 39218);
					match s {
						[] => true,
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
			super::push(out, 35450);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39219);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41316);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41317);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 39220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41319);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 41320);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 35451);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39221);
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
		['н', s @ ..] => {
			super::push(out, 35452);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41321);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41322);
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
			super::push(out, 35453);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39223);
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

/// Words ending -фонировать.
pub(crate) fn n48483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29472);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35454);
			match s {
				[] => true,
				['л', 'е', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41323);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42495);
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

/// Words ending -шонировать.
pub(crate) fn n48484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29473);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 35455);
			match s {
				[] => true,
				['о', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нферировать.
pub(crate) fn n63971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35509);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 39272);
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

/// Words ending -рферировать.
pub(crate) fn n63972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35510);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39273);
			match s {
				[] => true,
				['т', 'н', 'и', '\u{1}', s @ ..] => {
					super::push(out, 41356);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -церировать.
pub(crate) fn n48540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29526);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35511);
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

/// The word зарегистрировать.
pub(crate) fn n89489(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word переадминистрировать.
pub(crate) fn n89490(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -демонстрировать.
pub(crate) fn n75592(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 41365);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42523);
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

/// Words ending -устрировать.
pub(crate) fn n64003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35528);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 39293);
			match s {
				[] => true,
				['ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юстрировать.
pub(crate) fn n64004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35529);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 41366);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43188);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43189);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43538);
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
						['ы', s @ ..] => {
							super::push(out, 42524);
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
				['р', s @ ..] => {
					super::push(out, 41367);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42525);
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

/// Words ending -утрировать.
pub(crate) fn n48561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29545);
	s.is_empty()
}

/// Words ending -ьтрировать.
pub(crate) fn n48562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29546);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 35530);
			match s {
				[] => true,
				['и', 'ф', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word реконфигурировать.
pub(crate) fn n89492(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -токонфигурировать.
pub(crate) fn n89493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43539);
	match s {
		[] => true,
		['в', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word сконфигурировать.
pub(crate) fn n89494(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word забалансировать.
pub(crate) fn n89496(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ребалансировать.
pub(crate) fn n89497(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word разбалансировать.
pub(crate) fn n89498(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -пробалансировать.
pub(crate) fn n89499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43541);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дисбалансировать.
pub(crate) fn n89500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43542);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word отбалансировать.
pub(crate) fn n89501(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -недофинансировать.
pub(crate) fn n89502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43543);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рофинансировать.
pub(crate) fn n86492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42538);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 43202);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -софинансировать.
pub(crate) fn n86493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42539);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юансировать.
pub(crate) fn n64043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35558);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 39316);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енсировать.
pub(crate) fn n48585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29568);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 35559);
			match s {
				[] => true,
				['н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 41380);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'м', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онсировать.
pub(crate) fn n48586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29569);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 35560);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41381);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42540);
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
				['е', s @ ..] => {
					super::push(out, 39318);
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
		['п', s @ ..] => {
			super::push(out, 35561);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39319);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41382);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42541);
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

/// Words ending -ежиссировать.
pub(crate) fn n75665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39337);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42548);
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

/// Words ending -лиссировать.
pub(crate) fn n64061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35573);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 39338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 39339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оссировать.
pub(crate) fn n48594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29577);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 35574);
			match s {
				[] => true,
				['н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 35575);
			match s {
				[] => true,
				['г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 35576);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39340);
					match s {
						[] => true,
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
			super::push(out, 35577);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уссировать.
pub(crate) fn n48595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29578);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 35578);
			match s {
				[] => true,
				['с', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 35579);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -епетировать.
pub(crate) fn n64115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35619);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41422);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 41423);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41424);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42554);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42555);
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
					super::push(out, 41425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41426);
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

/// Words ending -ретировать.
pub(crate) fn n48622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29605);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 35620);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39375);
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
				['н', s @ ..] => {
					super::push(out, 39376);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41427);
							match s {
								[] => true,
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
					super::push(out, 39377);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41428);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39378);
					match s {
						[] => true,
						['к', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 35621);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39379);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41429);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', 'т', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41430);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42556);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43204);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43205);
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
							super::push(out, 41431);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42557);
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
		['р', s @ ..] => {
			super::push(out, 35622);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39380);
					match s {
						[] => true,
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
			super::push(out, 35623);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ь', 'л', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word фетировать.
pub(crate) fn n64120(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -цетировать.
pub(crate) fn n48624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29607);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35624);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 39381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шетировать.
pub(crate) fn n48625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29608);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 35625);
			match s {
				[] => true,
				['к', 'и', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41432);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 41433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оцитировать.
pub(crate) fn n64147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35646);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 39400);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41439);
					match s {
						[] => true,
						['ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 39401);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ксцитировать.
pub(crate) fn n75771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39402);
	match s {
		[] => true,
		['э', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -епроектировать.
pub(crate) fn n82801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41451);
	match s {
		[] => true,
		['р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -спроектировать.
pub(crate) fn n82802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41452);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пектировать.
pub(crate) fn n64160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35657);
	match s {
		[] => true,
		['с', 'н', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42562);
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
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42563);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42564);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42565);
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

/// Words ending -ректировать.
pub(crate) fn n64161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35658);
	match s {
		[] => true,
		['р', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41453);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 41454);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41455);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -сектировать.
pub(crate) fn n64162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35659);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 39414);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41457);
					match s {
						[] => true,
						['з', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тектировать.
pub(crate) fn n64163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35660);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39415);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фектировать.
pub(crate) fn n64164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35661);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 39416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 41460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъектировать.
pub(crate) fn n64165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35662);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39417);
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

/// Words ending -нктировать.
pub(crate) fn n48641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29623);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35663);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35664);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41461);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42566);
									match s {
										[] => true,
										['т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уктировать.
pub(crate) fn n48642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29624);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 35665);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39418);
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
		['р', s @ ..] => {
			super::push(out, 35666);
			match s {
				[] => true,
				['т', 'с', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41462);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42567);
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

/// Words ending -ментировать.
pub(crate) fn n64180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35677);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39426);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42570);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 42571);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43544);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43736);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90250(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41465);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42572);
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
		['г', s @ ..] => {
			super::push(out, 39427);
			match s {
				[] => true,
				['а', 'р', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 39428);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41466);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42573);
							match s {
								[] => true,
								['м', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41467);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42574);
							match s {
								[] => true,
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 41468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42575);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
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
		['и', s @ ..] => {
			super::push(out, 39429);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41469);
					match s {
						[] => true,
						['е', 'п', 'с', 'к', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43207);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43208);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 43545);
											match s {
												[] => true,
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
									super::push(out, 43209);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43546);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 43547);
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
		['м', s @ ..] => {
			super::push(out, 39430);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41470);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43210);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43211);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 39431);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41471);
					match s {
						[] => true,
						['ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 39432);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41472);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 41473);
					match s {
						[] => true,
						['о', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43212);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43213);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43548);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41474);
					match s {
						[] => true,
						['т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онтировать.
pub(crate) fn n48645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29627);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 35680);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39434);
					match s {
						[] => true,
						['и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 35681);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39435);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 39436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 39437);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39438);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41476);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 42577);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 42578);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 42579);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 43214);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39439);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39440);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39441);
					match s {
						[] => true,
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
			super::push(out, 35682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39442);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41477);
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
					super::push(out, 39443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 39444);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41478);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 35683);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 39445);
					match s {
						[] => true,
						['н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реконвертировать.
pub(crate) fn n89513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43549);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word проконвертировать.
pub(crate) fn n89514(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word сконвертировать.
pub(crate) fn n89515(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отконвертировать.
pub(crate) fn n89516(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -импортировать.
pub(crate) fn n75859(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 41495);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спортировать.
pub(crate) fn n75860(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', 'э', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 42588);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43219);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 41496);
			match s {
				[] => true,
				['а', 'р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сортировать.
pub(crate) fn n64224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35709);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39459);
			s.is_empty()
		}
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 39460);
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
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 39461);
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

/// The word проинвестировать.
pub(crate) fn n89518(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word проассистировать.
pub(crate) fn n89519(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -остировать.
pub(crate) fn n48669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29651);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 35721);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 35722);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 35723);
			match s {
				[] => true,
				['м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41505);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 41506);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41507);
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

/// Words ending -устировать.
pub(crate) fn n48670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29652);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 35724);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39475);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42597);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43222);
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
		['м', s @ ..] => {
			super::push(out, 35725);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 39476);
					match s {
						[] => true,
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
			super::push(out, 35726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юстировать.
pub(crate) fn n48671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29653);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 35727);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 35728);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 35729);
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

/// The word заасфальтировать.
pub(crate) fn n89521(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word проконсультировать.
pub(crate) fn n89522(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -фицировать.
pub(crate) fn n48715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29691);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35770);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39518);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 39519);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 41539);
							match s {
								[] => true,
								['у', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39520);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41540);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 35771);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39521);
					match s {
						[] => true,
						['о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39522);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 39523);
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 39524);
					match s {
						[] => true,
						['и', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39525);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41541);
							match s {
								[] => true,
								['в', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43232);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43561);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90037(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 43562);
													match s {
														[] => true,
														['е', s @ ..] => super::t32::n90038(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', 'и', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', 'у', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 41542);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 39526);
					match s {
						[] => true,
						['у', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39527);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', 'м', 'а', '\u{1}', s @ ..] => super::t32::n86702(s, out),
								['с', 'р', 'е', 'п', s @ ..] => super::t32::n86703(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t32::n75974(s, out),
				['с', s @ ..] => super::t32::n75975(s, out),
				['т', s @ ..] => super::t32::n75976(s, out),
				['ц', s @ ..] => super::t32::n75977(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t32::n64305(s, out),
		['о', s @ ..] => super::t32::n64306(s, out),
		['ф', s @ ..] => super::t32::n64307(s, out),
		_ => false,
	}
}

/// Words ending -тискиваться.
pub(crate) fn n67460(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 39841);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42651);
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыскиваться.
pub(crate) fn n54909(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
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
				['в', s @ ..] => {
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
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'п', s @ ..] => {
			match s {
				[] => true,
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
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -равливаться.
pub(crate) fn n67569(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', 'з', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тавливаться.
pub(crate) fn n67570(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', 'г', s @ ..] => {
			match s {
				[] => true,
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
				['д', 'о', 'п', s @ ..] => {
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
				['у', s @ ..] => {
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
		['с', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -молвливаться.
pub(crate) fn n54946(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овливаться.
pub(crate) fn n54947(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', 'с', 'у', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'к', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'й', 'а', 'н', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word огосударствливаться.
pub(crate) fn n54948(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -явливаться.
pub(crate) fn n54949(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', 'ю', 'л', 'с', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'ы', 'д', s @ ..] => {
			match s {
				[] => true,
				['з', 'и', '\u{1}', s @ ..] => {
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

/// Words ending -енствоваться.
pub(crate) fn n78129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39882);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 41683);
			match s {
				[] => true,
				['о', 'х', 'р', 'е', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', 'р', 'е', 'в', 'о', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 42659);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 43249);
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
				['у', s @ ..] => {
					super::push(out, 42660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьянствоваться.
pub(crate) fn n83922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41684);
	match s {
		[] => true,
		['п', s @ ..] => {
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

/// The word израболепствоваться.
pub(crate) fn n78131(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рствоваться.
pub(crate) fn n68357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36676);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['т', 'ы', 'м', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'у', 'м', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тствоваться.
pub(crate) fn n68358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36677);
	match s {
		[] => true,
		['е', 'в', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42661);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43250);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				_ => false,
			}
		}
		['у', 'п', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фствоваться.
pub(crate) fn n68359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36678);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 39884);
			match s {
				[] => true,
				['с', 'о', 'л', 'и', 'ф', s @ ..] => {
					match s {
						[] => true,
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
						['о', 'д', '\u{1}', s @ ..] => {
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

/// Words ending -ьствоваться.
pub(crate) fn n68360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36679);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41685);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42662);
							match s {
								[] => true,
								['е', 'д', 'и', 'в', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43570);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43740);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90253(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 43571);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43741);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43742);
													match s {
														[] => true,
														['р', s @ ..] => super::t32::n90254(s, out),
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
									super::push(out, 43251);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 43572);
											match s {
												[] => true,
												['о', 'р', 'к', 'о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41686);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42663);
							match s {
								[] => true,
								['о', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43573);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43574);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 43575);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ализоваться.
pub(crate) fn n68413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36704);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 39918);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41706);
					match s {
						[] => true,
						['л', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41707);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42668);
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
		['г', s @ ..] => {
			super::push(out, 39919);
			match s {
				[] => true,
				['е', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 39920);
			match s {
				[] => true,
				['н', 'а', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 39921);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 39922);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 39923);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42669);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42670);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43253);
									match s {
										[] => true,
										['в', 'д', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						['е', s @ ..] => {
							super::push(out, 42671);
							match s {
								[] => true,
								['т', 'а', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43577);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43578);
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
						['т', s @ ..] => {
							super::push(out, 42672);
							match s {
								[] => true,
								['с', 'у', 'д', 'н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 41708);
					match s {
						[] => true,
						['р', 'е', 'м', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 39924);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41709);
					match s {
						[] => true,
						['д', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'к', 'е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'а', 'м', 'м', 'а', 'р', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41710);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43254);
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
		['м', 'р', 'о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 41712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42674);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43255);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
					super::push(out, 41713);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42675);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43256);
									match s {
										[] => true,
										['а', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 42676);
							match s {
								[] => true,
								['и', 'р', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43579);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43743);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90255(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41714);
					match s {
						[] => true,
						['и', 'ц', 'а', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43257);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 43258);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['а', s @ ..] => {
					super::push(out, 41715);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t32::n83995(s, out),
				['и', s @ ..] => super::t32::n83996(s, out),
				['к', s @ ..] => super::t32::n83997(s, out),
				['о', s @ ..] => super::t32::n83998(s, out),
				['т', s @ ..] => super::t32::n83999(s, out),
				['у', s @ ..] => super::t32::n84000(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t32::n78219(s, out),
		['у', s @ ..] => super::t32::n78220(s, out),
		_ => false,
	}
}

/// The word отмобилизоваться.
pub(crate) fn n89596(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ллизоваться.
pub(crate) fn n68416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36706);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39929);
			match s {
				[] => true,
				['т', 'с', 'и', 'р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42688);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 42689);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 43265);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олизоваться.
pub(crate) fn n68417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36707);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 39930);
			match s {
				[] => true,
				['о', 'к', 'л', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 39931);
			match s {
				[] => true,
				['р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 39932);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41728);
					match s {
						[] => true,
						['и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мизоваться.
pub(crate) fn n55357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31883);
	match s {
		[] => true,
		['и', 'т', 'и', 'г', 'е', 'л', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'о', 'ф', 'и', 'н', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'и', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39933);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41729);
							match s {
								[] => true,
								['л', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -низоваться.
pub(crate) fn n55358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31884);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', 'ь', 'л', 'а', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 39934);
					match s {
						[] => true,
						['р', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42690);
									s.is_empty()
								}
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43266);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43581);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90058(s, out),
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
									super::push(out, 42691);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 43267);
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
								['о', s @ ..] => {
									super::push(out, 42692);
									match s {
										[] => true,
										['м', 'а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 42693);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'т', 'с', 'и', 'р', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'л', 'у', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'о', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39935);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41730);
							match s {
								[] => true,
								['э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 36708);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41731);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 42694);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'р', 'а', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', 'д', 'о', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

