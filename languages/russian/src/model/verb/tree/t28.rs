use flexion::Path;

/// Words ending -уетиться.
pub(crate) fn n27341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19499);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 27732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33955);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33956);
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
				['о', s @ ..] => {
					super::push(out, 33957);
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

/// Words ending -фетиться.
pub(crate) fn n27342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27733);
			match s {
				[] => true,
				['р', 'а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38034);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38035);
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
		_ => false,
	}
}

/// Words ending -шетиться.
pub(crate) fn n27343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19501);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27734);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38036);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 38037);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40607);
									match s {
										[] => true,
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
							super::push(out, 38038);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 40608);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олотиться.
pub(crate) fn n44434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27757);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 33979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38057);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40610);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 33980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38058);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38059);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 38060);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38061);
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
				['ы', s @ ..] => {
					super::push(out, 38062);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42124);
									s.is_empty()
								}
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
			super::push(out, 33981);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38063);
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
					super::push(out, 38064);
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
					super::push(out, 38065);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 38066);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 33982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38067);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38068);
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
					super::push(out, 38069);
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
					super::push(out, 38070);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 40613);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42125);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 38071);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ротиться.
pub(crate) fn n27361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19519);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27760);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33986);
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
		['к', s @ ..] => {
			super::push(out, 27761);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33987);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38073);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42126);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42127);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33988);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27762);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 33989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38075);
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
							super::push(out, 38076);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 40615);
									match s {
										[] => true,
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
							super::push(out, 38077);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40616);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42128);
											s.is_empty()
										}
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
					super::push(out, 33990);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38078);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 38079);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 38080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33991);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38081);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 40617);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42129);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42130);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word охотиться.
pub(crate) fn n60154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33992);
	s.is_empty()
}

/// The word наохотиться.
pub(crate) fn n60155(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word разохотиться.
pub(crate) fn n60156(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word приохотиться.
pub(crate) fn n60157(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оохотиться.
pub(crate) fn n60158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33993);
	match s {
		[] => true,
		['в', 'о', 'л', 'с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
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

/// The word отохотиться.
pub(crate) fn n60159(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ухотиться.
pub(crate) fn n44441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27764);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 33994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word заненаститься.
pub(crate) fn n87951(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -еститься.
pub(crate) fn n27367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19525);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27774);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34018);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 34019);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34020);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 38093);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40620);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40621);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42133);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34021);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 34022);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38094);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40622);
									match s {
										[] => true,
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
							super::push(out, 38095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40623);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40624);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 40625);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42134);
											match s {
												[] => true,
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
									super::push(out, 40626);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27775);
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 27776);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34023);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 34024);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 34025);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
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
						['о', 'в', '\u{1}', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 34026);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38096);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27777);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34027);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 38097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 40627);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 42135);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38098);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38099);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 38100);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40628);
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
							super::push(out, 38101);
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
							super::push(out, 38102);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38103);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40629);
									match s {
										[] => true,
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
							super::push(out, 38104);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40630);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t31::n60219(s, out),
				['с', s @ ..] => super::t31::n60220(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чиститься.
pub(crate) fn n44458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27779);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34031);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 34032);
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
				['о', 'п', '\u{1}', s @ ..] => {
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
			super::push(out, 34033);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', 'а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 34034);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оститься.
pub(crate) fn n27372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19530);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27784);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 34035);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38107);
							match s {
								[] => true,
								['ч', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27785);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34036);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34037);
					match s {
						[] => true,
						['з', s @ ..] => {
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
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 34038);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 34039);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38108);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34040);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38109);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40631);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42138);
											match s {
												[] => true,
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
									super::push(out, 40632);
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
				['с', s @ ..] => {
					super::push(out, 34041);
					match s {
						[] => true,
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
			super::push(out, 27787);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34042);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40633);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 40634);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42139);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27788);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34043);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34044);
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
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 34045);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 34046);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38110);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27789);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34047);
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
				['е', s @ ..] => {
					super::push(out, 34048);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', s @ ..] => {
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
				['с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27790);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рститься.
pub(crate) fn n27373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19531);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27791);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38112);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 38113);
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
		_ => false,
	}
}

/// Words ending -уститься.
pub(crate) fn n27374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19532);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 27792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34051);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34052);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34053);
					match s {
						[] => true,
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
			super::push(out, 27793);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34054);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34055);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38114);
							match s {
								[] => true,
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
					super::push(out, 34056);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38115);
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
					super::push(out, 34057);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27794);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34058);
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
						['к', s @ ..] => {
							super::push(out, 38117);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 40635);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34060);
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
								['н', '\u{1}', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 34061);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38118);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27795);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38119);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38120);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыститься.
pub(crate) fn n27375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19533);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 27796);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38121);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38122);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40636);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьститься.
pub(crate) fn n27376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19534);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 27797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34063);
					s.is_empty()
				}
				['е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34064);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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

/// The word шутиться.
pub(crate) fn n44493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27812);
	s.is_empty()
}

/// Words ending -ашутиться.
pub(crate) fn n44494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27813);
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

/// The word дошутиться.
pub(crate) fn n44495(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word расшутиться.
pub(crate) fn n44496(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тшутиться.
pub(crate) fn n44497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27814);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ышутиться.
pub(crate) fn n44498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27815);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 34082);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щутиться.
pub(crate) fn n27384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19542);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еволочиться.
pub(crate) fn n72980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38158);
	match s {
		[] => true,
		['р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зволочиться.
pub(crate) fn n72981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38159);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 40644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иволочиться.
pub(crate) fn n72982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38160);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 40646);
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

/// Words ending -оволочиться.
pub(crate) fn n72983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38161);
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

/// Words ending -сволочиться.
pub(crate) fn n72984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38162);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тволочиться.
pub(crate) fn n72985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38163);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уволочиться.
pub(crate) fn n72986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38164);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыволочиться.
pub(crate) fn n72987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38165);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 40647);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -толочиться.
pub(crate) fn n60434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34150);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 38166);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мочиться.
pub(crate) fn n27433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19589);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27893);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27894);
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
			super::push(out, 27895);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27896);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27897);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34151);
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
			super::push(out, 27898);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34152);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 34153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34154);
					match s {
						[] => true,
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
			super::push(out, 27899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34155);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38167);
							match s {
								[] => true,
								['о', 'п', 'у', '\u{1}', s @ ..] => {
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
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27901);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27902);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27903);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рочиться.
pub(crate) fn n27434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19590);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 27904);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34157);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 34158);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38168);
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
					super::push(out, 34159);
					match s {
						[] => true,
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
			super::push(out, 27905);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38169);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38170);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
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
				['п', s @ ..] => {
					super::push(out, 34161);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38171);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38172);
							match s {
								[] => true,
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
					super::push(out, 34162);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38173);
							match s {
								[] => true,
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
					super::push(out, 34163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38174);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 38175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 38176);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40649);
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
							super::push(out, 38177);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40650);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27906);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34164);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 34165);
					match s {
						[] => true,
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
			super::push(out, 27907);
			match s {
				[] => true,
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27908);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 34166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38178);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38179);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40651);
									match s {
										[] => true,
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
							super::push(out, 38180);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40652);
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
							super::push(out, 38181);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 40653);
									match s {
										[] => true,
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
							super::push(out, 38182);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40654);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27909);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сочиться.
pub(crate) fn n27435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19591);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27910);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27911);
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
			super::push(out, 27912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27913);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34167);
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
			super::push(out, 27914);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27915);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34168);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точиться.
pub(crate) fn n27436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19592);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27916);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27917);
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
			super::push(out, 27918);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27919);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34169);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 34170);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38183);
							match s {
								[] => true,
								['р', 'с', s @ ..] => {
									match s {
										[] => true,
										['о', 'с', s @ ..] => {
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
				['р', s @ ..] => {
					super::push(out, 34171);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38184);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27921);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34172);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34173);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38185);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40655);
									match s {
										[] => true,
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
							super::push(out, 38186);
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
					super::push(out, 34174);
					match s {
						[] => true,
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
			super::push(out, 27922);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27924);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34175);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40656);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 40657);
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
				['в', s @ ..] => {
					super::push(out, 34176);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ручиться.
pub(crate) fn n27454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27956);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 27957);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 27958);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27959);
			match s {
				[] => true,
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27960);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27961);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34188);
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
		['т', s @ ..] => {
			super::push(out, 27962);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 34189);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 38193);
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
		['ы', s @ ..] => {
			super::push(out, 27963);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сучиться.
pub(crate) fn n27455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19611);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27964);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27965);
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
			super::push(out, 27966);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27967);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27968);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27969);
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
			super::push(out, 27970);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27971);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тучиться.
pub(crate) fn n27456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19612);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27972);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27973);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыучиться.
pub(crate) fn n27457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19613);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27974);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34193);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыкрошиться.
pub(crate) fn n60626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34228);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38216);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орошиться.
pub(crate) fn n44757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28017);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 34229);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38217);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 38218);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40659);
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
					super::push(out, 38219);
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 34230);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38220);
					match s {
						[] => true,
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
			super::push(out, 34231);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38221);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38222);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 34232);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38223);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 38224);
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

/// Words ending -прошиться.
pub(crate) fn n44758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28018);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 34233);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трошиться.
pub(crate) fn n44759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28019);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34234);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40660);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 40661);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 42141);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тошиться.
pub(crate) fn n27501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19657);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28020);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34235);
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
		['о', s @ ..] => {
			super::push(out, 28021);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38225);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38226);
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
		['с', s @ ..] => {
			super::push(out, 28022);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 34237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сушиться.
pub(crate) fn n27515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19670);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28048);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 28049);
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
			super::push(out, 28050);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
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
			super::push(out, 28051);
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
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 28052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 28053);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34252);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушиться.
pub(crate) fn n27516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28054);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 28055);
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
			super::push(out, 28056);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38236);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 38237);
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
				['р', s @ ..] => {
					super::push(out, 34254);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38238);
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
			super::push(out, 28057);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28058);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34257);
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
			super::push(out, 28059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 28060);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word сподвигнуться.
pub(crate) fn n87961(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ергнуться.
pub(crate) fn n45002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28143);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 38264);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 40677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 38265);
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
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38266);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42146);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 43008);
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
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38267);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 40678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оргнуться.
pub(crate) fn n45003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28144);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 34294);
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34295);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38269);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40680);
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
					super::push(out, 38270);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 40681);
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							super::push(out, 40682);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40683);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 42147);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 38271);
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

/// Words ending -угнуться.
pub(crate) fn n27615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19726);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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

/// The word выгнуться.
pub(crate) fn n60834(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рыгнуться.
pub(crate) fn n45009(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 34296);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'и', '\u{1}', s @ ..] => {
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

/// The word матюгнуться.
pub(crate) fn n27617(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ягнуться.
pub(crate) fn n27618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19728);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28146);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 28147);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -столкнуться.
pub(crate) fn n73299(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word оттолкнуться.
pub(crate) fn n73300(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ытолкнуться.
pub(crate) fn n73301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38284);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 40688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мкнуться.
pub(crate) fn n27639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19747);
	match s {
		[] => true,
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -окнуться.
pub(crate) fn n27640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19748);
	match s {
		[] => true,
		['к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 28171);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34312);
					match s {
						[] => true,
						['в', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 28172);
			match s {
				[] => true,
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'ч', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ркнуться.
pub(crate) fn n27641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19749);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28173);
			match s {
				[] => true,
				['в', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 34313);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 38285);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 38286);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40689);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 34314);
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
		['е', s @ ..] => {
			super::push(out, 28174);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34315);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38287);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40690);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40691);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 34316);
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 38288);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40692);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 28175);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 34317);
					match s {
						[] => true,
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
			super::push(out, 28176);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 34318);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 40693);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 40694);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42148);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34319);
					match s {
						[] => true,
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
			super::push(out, 28177);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 34320);
					match s {
						[] => true,
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
			super::push(out, 28178);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34321);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38289);
							match s {
								[] => true,
								['к', s @ ..] => {
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
				['т', s @ ..] => {
					super::push(out, 34322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 34323);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 38290);
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

/// Words ending -ткнуться.
pub(crate) fn n27642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19750);
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
		['е', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 28179);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
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
				['п', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 28180);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укнуться.
pub(crate) fn n27643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19751);
	match s {
		[] => true,
		['а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ж', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'у', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 28181);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -чкнуться.
pub(crate) fn n27644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19752);
	match s {
		[] => true,
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкнуться.
pub(crate) fn n27645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19753);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 28182);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34325);
					match s {
						[] => true,
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
			super::push(out, 28183);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34326);
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
								['о', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 28184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34327);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
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
			super::push(out, 28185);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 28186);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 28187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34329);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34330);
					match s {
						[] => true,
						['п', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькнуться.
pub(crate) fn n27646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19754);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28188);
			match s {
				[] => true,
				['у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкнуться.
pub(crate) fn n27647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19755);
	match s {
		[] => true,
		['а', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34331);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38291);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -якнуться.
pub(crate) fn n27648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19756);
	match s {
		[] => true,
		['б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'з', 'и', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ж', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', 'х', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вырнуться.
pub(crate) fn n45136(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34344);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 40697);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 42149);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34345);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 40698);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 42150);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 34346);
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
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 38299);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прыснуться.
pub(crate) fn n61025(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 38302);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юснуться.
pub(crate) fn n27674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19781);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28220);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдохнуться.
pub(crate) fn n61103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34368);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38314);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40704);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лохнуться.
pub(crate) fn n45186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28244);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34369);
			match s {
				[] => true,
				['ш', s @ ..] => {
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
		['о', 'п', 'с', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -грохнуться.
pub(crate) fn n61106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34370);
	s.is_empty()
}

/// Words ending -орохнуться.
pub(crate) fn n61107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34371);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38315);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сохнуться.
pub(crate) fn n45188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28246);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 34372);
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
		_ => false,
	}
}

/// The word пхнуться.
pub(crate) fn n27689(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рхнуться.
pub(crate) fn n27690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19795);
	match s {
		[] => true,
		['о', 'п', 'с', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ухнуться.
pub(crate) fn n27691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19796);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 28247);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 28248);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34373);
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
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыхнуться.
pub(crate) fn n27692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19797);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28249);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'о', 'р', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 28250);
			match s {
				[] => true,
				['е', 'р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 28251);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34374);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38316);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 38317);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40705);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 28252);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 38318);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 40706);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42152);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юхнуться.
pub(crate) fn n27693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19798);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28253);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яхнуться.
pub(crate) fn n27694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19799);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 38319);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40707);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word откашлянуться.
pub(crate) fn n87968(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выкашлянуться.
pub(crate) fn n87969(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -правляться.
pub(crate) fn n61234(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -травляться.
pub(crate) fn n61235(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
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

/// Words ending -ставляться.
pub(crate) fn n45348(s: &[char], out: &mut Path<'_>) -> bool {
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', 'и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'н', '\u{1}', s @ ..] => {
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
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 34395);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40714);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42155);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -евляться.
pub(crate) fn n27823(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', 'г', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['е', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'д', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -звляться.
pub(crate) fn n27824(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'т', s @ ..] => {
			match s {
				[] => true,
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			match s {
				[] => true,
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ъ', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивляться.
pub(crate) fn n27825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19859);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 28296);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38333);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40715);
									match s {
										[] => true,
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
							super::push(out, 38334);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40716);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 28297);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 28298);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 38335);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40717);
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
							super::push(out, 38336);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40718);
									match s {
										[] => true,
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
							super::push(out, 38337);
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
			super::push(out, 28299);
			match s {
				[] => true,
				['о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'о', 'л', 'и', 'м', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овляться.
pub(crate) fn n27827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19860);
	match s {
		[] => true,
		['г', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'р', 'е', 'ц', 'о', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['с', 'о', 'г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
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
						['з', 'о', 'в', '\u{1}', s @ ..] => {
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'с', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'о', s @ ..] => {
			match s {
				[] => true,
				['д', 'з', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['о', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['ы', 'б', s @ ..] => {
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

/// Words ending -твляться.
pub(crate) fn n27828(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28300);
			match s {
				[] => true,
				['в', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', 'м', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 28301);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', 'з', 'ы', s @ ..] => {
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
						['ч', 'а', 'к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['щ', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', s @ ..] => {
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', 'в', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', 'с', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
						['а', 'д', 'у', 'с', 'о', 'г', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
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

/// Words ending -являться.
pub(crate) fn n27830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19861);
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
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', s @ ..] => {
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
		['ъ', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
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

/// The word поупражняться.
pub(crate) fn n87971(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word помолебствовать.
pub(crate) fn n87976(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word позлобствовать.
pub(crate) fn n87977(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поспособствовать.
pub(crate) fn n87978(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -фобствовать.
pub(crate) fn n62754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34655);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', 'ю', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'г', 'н', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', 'с', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['я', 'в', 'а', 'л', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'у', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вствовать.
pub(crate) fn n30172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20579);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28836);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 34656);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38441);
							match s {
								[] => true,
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34657);
					match s {
						[] => true,
						['у', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34658);
					match s {
						[] => true,
						['д', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40759);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40760);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42166);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 38442);
							match s {
								[] => true,
								['у', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 28837);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34659);
					match s {
						[] => true,
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
			super::push(out, 28838);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28839);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34661);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34662);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38444);
							match s {
								[] => true,
								['т', 'с', 'е', 'л', 'х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34663);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38445);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 38446);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40761);
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
				['п', s @ ..] => {
					super::push(out, 34664);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38447);
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
					super::push(out, 34665);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38448);
							match s {
								[] => true,
								['и', 'е', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42167);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43009);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89278(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 28840);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38449);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 38450);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 40762);
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
							super::push(out, 38451);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 38452);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 40763);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 40764);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 40765);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42168);
											match s {
												[] => true,
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
									super::push(out, 40766);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42169);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42170);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43010);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89279(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38453);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 28841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дствовать.
pub(crate) fn n30174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20581);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28843);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34667);
					match s {
						[] => true,
						['к', 'и', 'р', 'п', 'о', 'к', 'у', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40767);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42171);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34668);
					match s {
						[] => true,
						['о', 'л', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40768);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42172);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 28844);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38455);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 40770);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42173);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34670);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34671);
					match s {
						[] => true,
						['д', 'ю', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', 'а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34672);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38456);
							match s {
								[] => true,
								['л', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40771);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40772);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42174);
											s.is_empty()
										}
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
					super::push(out, 34673);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 40773);
									match s {
										[] => true,
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42175);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42176);
											match s {
												[] => true,
												['р', 'б', 'о', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 28845);
			match s {
				[] => true,
				['х', 'е', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28846);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34674);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38457);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 38458);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 40774);
									match s {
										[] => true,
										['у', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43011);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 43012);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89280(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34675);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34676);
					match s {
						[] => true,
						['б', 'с', 'а', 'м', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ю', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40775);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42177);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34677);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t31::n47630(s, out),
		['у', s @ ..] => {
			super::push(out, 28848);
			match s {
				[] => true,
				['л', 'б', 'о', s @ ..] => super::t31::n62781(s, out),
				['н', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 28849);
			match s {
				[] => true,
				['л', 'з', '\u{1}', s @ ..] => super::t31::n62783(s, out),
				['т', 'с', 'с', 'е', 'б', '\u{1}', s @ ..] => super::t31::n62784(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t31::n47633(s, out),
		_ => false,
	}
}

/// Words ending -ествовать.
pub(crate) fn n30175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20582);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 28851);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34681);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 38461);
							match s {
								[] => true,
								['а', 'л', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 38462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 28852);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34682);
					match s {
						[] => true,
						['а', 'х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34683);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38463);
							match s {
								[] => true,
								['е', 'ж', 'у', 'м', '\u{1}', s @ ..] => {
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
				['р', s @ ..] => {
					super::push(out, 34684);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38464);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42183);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42184);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43014);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89281(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42185);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43015);
													match s {
														[] => true,
														['в', '\u{1}', s @ ..] => super::t31::n89282(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34685);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38465);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40777);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42186);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 38466);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40778);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 28853);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34686);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34687);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 38467);
							match s {
								[] => true,
								['о', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 38468);
							match s {
								[] => true,
								['у', 'ч', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 38469);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40779);
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
						['к', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40780);
									match s {
										[] => true,
										['о', 'л', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ю', s @ ..] => {
									super::push(out, 40781);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42187);
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
						['л', s @ ..] => {
							super::push(out, 38470);
							match s {
								[] => true,
								['а', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 38471);
							match s {
								[] => true,
								['с', 'а', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 38472);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40782);
									match s {
										[] => true,
										['н', 'с', '\u{1}', s @ ..] => super::t31::n85650(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t31::n62794(s, out),
				['и', s @ ..] => super::t31::n62795(s, out),
				['н', s @ ..] => super::t31::n62796(s, out),
				['о', s @ ..] => super::t31::n62797(s, out),
				['ы', s @ ..] => super::t31::n62798(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t31::n47637(s, out),
		['щ', s @ ..] => super::t31::n47638(s, out),
		_ => false,
	}
}

/// Words ending -иствовать.
pub(crate) fn n30176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20583);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 28856);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34704);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38500);
							match s {
								[] => true,
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
					super::push(out, 34705);
					match s {
						[] => true,
						['ш', 'ь', 'н', 'е', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 28857);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34706);
					match s {
						[] => true,
						['а', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 28858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34707);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йствовать.
pub(crate) fn n30177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20584);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28859);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34708);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38501);
							match s {
								[] => true,
								['н', 'с', 'а', 'р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34709);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38502);
							match s {
								[] => true,
								['б', 'е', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34710);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38503);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 40807);
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
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34711);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38504);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['о', 'х', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 43029);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89303(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 43030);
													match s {
														[] => true,
														['и', s @ ..] => super::t31::n89304(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['я', s @ ..] => {
											super::push(out, 42206);
											match s {
												[] => true,
												['л', 'г', 'о', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 40808);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 42207);
											match s {
												[] => true,
												['л', 'г', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 28860);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38505);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'ц', 'и', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 38506);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 40810);
									match s {
										[] => true,
										['б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 40811);
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
						['н', 'и', 'х', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => super::t31::n74268(s, out),
						['с', s @ ..] => {
							super::push(out, 38508);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 38509);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 40823);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', 'д', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'о', 'т', 'о', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'а', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'е', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'и', 'р', 'а', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28861);
			match s {
				[] => true,
				['т', 'и', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t31::n47645(s, out),
		['у', s @ ..] => super::t31::n47646(s, out),
		['ю', s @ ..] => super::t31::n47647(s, out),
		['я', s @ ..] => super::t31::n47648(s, out),
		_ => false,
	}
}

/// Words ending -мствовать.
pub(crate) fn n30178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20585);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28866);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 34718);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38516);
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28867);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34719);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38517);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40830);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42213);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34720);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38518);
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
				['о', s @ ..] => {
					super::push(out, 34721);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 38519);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40831);
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
						['х', s @ ..] => {
							super::push(out, 38520);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 40832);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28868);
			match s {
				[] => true,
				['д', 'и', 'п', 'о', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'о', 'р', 'е', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 28869);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34722);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38521);
							match s {
								[] => true,
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
					super::push(out, 34723);
					match s {
						[] => true,
						['о', 'н', 'ь', 'л', 'о', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40833);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42214);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34724);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40834);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42215);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34725);
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
		['я', s @ ..] => {
			super::push(out, 28870);
			match s {
				[] => true,
				['р', 'п', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нствовать.
pub(crate) fn n30179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20586);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28871);
			match s {
				[] => true,
				['б', 'а', 'ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', 'з', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 34726);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							super::push(out, 38522);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 40835);
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
						['ы', 'ц', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 34727);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38523);
							match s {
								[] => true,
								['т', 'р', 'а', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34728);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38524);
							match s {
								[] => true,
								['а', 'т', 'е', 'г', 'е', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38525);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 40836);
									match s {
										[] => true,
										['и', 'р', 'х', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 34729);
					match s {
						[] => true,
						['и', 'т', 'и', s @ ..] => {
							match s {
								[] => true,
								['л', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34730);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38526);
							match s {
								[] => true,
								['т', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42216);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43034);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89310(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38527);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 40837);
									match s {
										[] => true,
										['г', 'н', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34731);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38528);
							match s {
								[] => true,
								['л', 'о', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 38529);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38530);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42217);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42218);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43035);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89311(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 42219);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 42220);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 43036);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89312(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'и', 'п', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 28872);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34732);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38531);
							s.is_empty()
						}
						['о', 'х', 'р', 'е', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 38532);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 40838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t31::n62857(s, out),
				['ж', s @ ..] => super::t31::n62858(s, out),
				['л', s @ ..] => super::t31::n62859(s, out),
				['м', s @ ..] => super::t31::n62860(s, out),
				['п', s @ ..] => super::t31::n62861(s, out),
				['ш', s @ ..] => super::t31::n62862(s, out),
				['щ', s @ ..] => super::t31::n62863(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t31::n47656(s, out),
		['и', s @ ..] => super::t31::n47657(s, out),
		['о', s @ ..] => super::t31::n47658(s, out),
		['у', s @ ..] => super::t31::n47659(s, out),
		['я', s @ ..] => super::t31::n47660(s, out),
		_ => false,
	}
}

/// Words ending -пствовать.
pub(crate) fn n30180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20587);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28878);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34753);
					match s {
						[] => true,
						['т', 'а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 28879);
			match s {
				[] => true,
				['л', 'о', 'б', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28880);
			match s {
				[] => true,
				['з', 'е', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 34754);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38553);
							match s {
								[] => true,
								['и', 'п', 'е', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'о', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', 'т', 'о', 'в', 'о', 'л', 'о', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рствовать.
pub(crate) fn n30181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20588);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28881);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38554);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38555);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40859);
									match s {
										[] => true,
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
							super::push(out, 38556);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 34756);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38557);
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
				['д', s @ ..] => {
					super::push(out, 34757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38558);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38559);
							match s {
								[] => true,
								['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 38560);
							match s {
								[] => true,
								['с', 'о', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34758);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38561);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40861);
									match s {
										[] => true,
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
							super::push(out, 38562);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 40862);
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
				['м', s @ ..] => {
					super::push(out, 34759);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38563);
							match s {
								[] => true,
								['н', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34760);
					match s {
						[] => true,
						['с', 'и', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34761);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38564);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40863);
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
						['е', s @ ..] => {
							super::push(out, 38565);
							match s {
								[] => true,
								['р', 'к', 'е', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 38566);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42228);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 43040);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89316(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34762);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38567);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 34763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38568);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38569);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40864);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 40865);
									match s {
										[] => true,
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
							super::push(out, 38570);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40866);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 40867);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42229);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 38571);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40868);
									match s {
										[] => true,
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
							super::push(out, 38572);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40869);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t31::n47666(s, out),
		['е', s @ ..] => super::t31::n47667(s, out),
		['и', s @ ..] => {
			super::push(out, 28884);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34782);
					match s {
						[] => true,
						['а', 'г', 'и', 'р', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'а', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t31::n62913(s, out),
				['п', s @ ..] => super::t31::n62914(s, out),
				['т', s @ ..] => super::t31::n62915(s, out),
				['ш', s @ ..] => super::t31::n62916(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t31::n47669(s, out),
		['у', s @ ..] => super::t31::n47670(s, out),
		['ы', s @ ..] => super::t31::n47671(s, out),
		['э', s @ ..] => super::t31::n47672(s, out),
		['я', s @ ..] => super::t31::n47673(s, out),
		_ => false,
	}
}

/// Words ending -тствовать.
pub(crate) fn n30182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20589);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28890);
			match s {
				[] => true,
				['б', 'о', 'р', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'з', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'о', 'в', 'д', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 34801);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38616);
							match s {
								[] => true,
								['ф', 'а', 'н', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 34802);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38617);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 40911);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'ц', 'е', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34803);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['и', 'н', 'а', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', 'т', 'я', 'в', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 28891);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34804);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38618);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40912);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43049);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 43050);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89325(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 43051);
													match s {
														[] => true,
														['о', '\u{1}', s @ ..] => super::t31::n89326(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38619);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40913);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42255);
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
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34805);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38620);
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
				['л', s @ ..] => {
					super::push(out, 34806);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38621);
							match s {
								[] => true,
								['о', 'х', 'и', 'т', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34807);
					match s {
						[] => true,
						['о', 'х', 'а', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34808);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28892);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34809);
					match s {
						[] => true,
						['н', 'а', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 34810);
					match s {
						[] => true,
						['а', 'р', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34811);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38623);
							match s {
								[] => true,
								['с', 'и', 'т', 'н', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34812);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38624);
							match s {
								[] => true,
								['б', 'и', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42256);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43052);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89327(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38625);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40914);
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
				['у', s @ ..] => {
					super::push(out, 34813);
					match s {
						[] => true,
						['з', 'е', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 28893);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34814);
					match s {
						[] => true,
						['ю', 'ъ', 'д', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28894);
			match s {
				[] => true,
				['а', s @ ..] => super::t31::n62950(s, out),
				['е', s @ ..] => super::t31::n62951(s, out),
				['я', s @ ..] => {
					super::push(out, 34817);
					match s {
						[] => true,
						['л', s @ ..] => super::t31::n74455(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t31::n47679(s, out),
		['у', s @ ..] => super::t31::n47680(s, out),
		['ы', s @ ..] => super::t31::n47681(s, out),
		['ь', s @ ..] => super::t31::n47682(s, out),
		['я', s @ ..] => super::t31::n47683(s, out),
		_ => false,
	}
}

/// Words ending -фствовать.
pub(crate) fn n30183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20590);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28900);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 34828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38644);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40934);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 28901);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34829);
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
		['о', s @ ..] => {
			super::push(out, 28902);
			match s {
				[] => true,
				['с', 'о', 'л', 'и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38645);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38646);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40935);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 40936);
									match s {
										[] => true,
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
							super::push(out, 38647);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40937);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьствовать.
pub(crate) fn n30184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20591);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34830);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38648);
							match s {
								[] => true,
								['х', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 38649);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 40938);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 42262);
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
								['с', s @ ..] => {
									super::push(out, 40939);
									match s {
										[] => true,
										['и', 'ф', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'б', 'у', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t31::n74481(s, out),
						['х', s @ ..] => super::t31::n74482(s, out),
						['ч', s @ ..] => super::t31::n74483(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t31::n62970(s, out),
				['и', s @ ..] => super::t31::n62971(s, out),
				['о', s @ ..] => super::t31::n62972(s, out),
				['у', s @ ..] => super::t31::n62973(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дематериализовать.
pub(crate) fn n88148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43080);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -икализовать.
pub(crate) fn n63087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34903);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 38696);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 38697);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40965);
					match s {
						[] => true,
						['е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 38698);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40966);
					match s {
						[] => true,
						['м', 'м', 'а', 'р', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окализовать.
pub(crate) fn n63088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34904);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
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

/// Words ending -мализовать.
pub(crate) fn n47818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28981);
	match s {
		[] => true,
		['р', 'о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 40967);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 42287);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40968);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42288);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нализовать.
pub(crate) fn n47819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28982);
	match s {
		[] => true,
		['а', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', 'и', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 34905);
			match s {
				[] => true,
				['г', 'р', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34906);
			match s {
				[] => true,
				['и', 'ц', 'а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40969);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 40970);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рализовать.
pub(crate) fn n47820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28983);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34907);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 34908);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38699);
					match s {
						[] => true,
						['и', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38700);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40971);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 42289);
									match s {
										[] => true,
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
							super::push(out, 40972);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43081);
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
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 34909);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38701);
					match s {
						[] => true,
						['с', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 34910);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34911);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38703);
					match s {
						[] => true,
						['е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 38704);
					s.is_empty()
				}
				['н', 'е', 'ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40973);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42290);
									match s {
										[] => true,
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
							super::push(out, 40974);
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
		['у', s @ ..] => {
			super::push(out, 34912);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38705);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40975);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43497);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89972(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 43498);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89973(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40976);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 42291);
									match s {
										[] => true,
										['р', 'т', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тализовать.
pub(crate) fn n47821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28984);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34913);
			match s {
				[] => true,
				['л', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34914);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38706);
					match s {
						[] => true,
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
			super::push(out, 34915);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38707);
					match s {
						[] => true,
						['м', 'м', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уализовать.
pub(crate) fn n47822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28985);
	match s {
		[] => true,
		['з', 'и', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38708);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42292);
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
				['п', 'е', 'ц', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'и', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'к', 'е', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елизовать.
pub(crate) fn n30278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20662);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 28986);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34916);
					match s {
						[] => true,
						['р', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илизовать.
pub(crate) fn n30279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20663);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 28987);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34917);
					match s {
						[] => true,
						['т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34918);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38709);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 40977);
									match s {
										[] => true,
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
							super::push(out, 38710);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 38711);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38712);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40978);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 28988);
			s.is_empty()
		}
		['р', 'е', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34919);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38713);
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
		['т', s @ ..] => {
			super::push(out, 28989);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 34920);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 34921);
					match s {
						[] => true,
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
			super::push(out, 28990);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34922);
					match s {
						[] => true,
						['р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клизовать.
pub(crate) fn n30280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20664);
	match s {
		[] => true,
		['и', 'ц', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллизовать.
pub(crate) fn n30281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20665);
	match s {
		[] => true,
		['а', 'т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34923);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38714);
							match s {
								[] => true,
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
					super::push(out, 34924);
					match s {
						[] => true,
						['и', 'р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40979);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40980);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 40981);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 40982);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 40983);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42293);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олизовать.
pub(crate) fn n30282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20666);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 28991);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34925);
					match s {
						[] => true,
						['к', 'л', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 28992);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34926);
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

/// The word характеризовать.
pub(crate) fn n88160(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -охарактеризовать.
pub(crate) fn n88161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43083);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оризовать.
pub(crate) fn n30293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20676);
	match s {
		[] => true,
		['в', 'а', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', 'е', 'т', 'а', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 29021);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34950);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38728);
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29022);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34951);
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
		['т', s @ ..] => {
			super::push(out, 29023);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34952);
					match s {
						[] => true,
						['а', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 34953);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38729);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40992);
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
						['е', s @ ..] => {
							super::push(out, 38730);
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
				['о', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', 'а', 'т', 'е', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тризовать.
pub(crate) fn n30294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20677);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29024);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34954);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38731);
							match s {
								[] => true,
								['е', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34955);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38732);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40993);
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
						['о', s @ ..] => {
							super::push(out, 38733);
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
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29025);
			match s {
				[] => true,
				['е', 'л', 'э', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38734);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 40994);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уризовать.
pub(crate) fn n30295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20678);
	match s {
		[] => true,
		['т', 'к', 'у', 'р', 'т', 'с', 'е', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юризовать.
pub(crate) fn n30296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20679);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 29026);
			match s {
				[] => true,
				['а', 'и', 'н', 'и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яризовать.
pub(crate) fn n30297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20680);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34956);
					match s {
						[] => true,
						['п', s @ ..] => {
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
				['у', s @ ..] => {
					super::push(out, 34957);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38735);
							match s {
								[] => true,
								['е', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 38736);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', 'д', 'а', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тивировать.
pub(crate) fn n48196(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 35056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 42297);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38795);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 41018);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 42298);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['у', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 42299);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43084);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35057);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38796);
					match s {
						[] => true,
						['т', 'с', 'б', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35058);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41019);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42300);
									match s {
										[] => true,
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
							super::push(out, 41020);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 42301);
									match s {
										[] => true,
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
							super::push(out, 41021);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 35059);
			match s {
				[] => true,
				['л', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41022);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41023);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 41024);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43085);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43086);
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
							super::push(out, 41025);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -архивировать.
pub(crate) fn n63384(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38797);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 38798);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 41026);
					match s {
						[] => true,
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
			super::push(out, 38799);
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

/// Words ending -овировать.
pub(crate) fn n30576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20871);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 29199);
			match s {
				[] => true,
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29200);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 38800);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 41027);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвировать.
pub(crate) fn n30577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20872);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 35060);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41028);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42302);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 38801);
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
				['с', s @ ..] => {
					super::push(out, 35062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38802);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 38803);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 41029);
									match s {
										[] => true,
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
							super::push(out, 38804);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41030);
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
							super::push(out, 38805);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42303);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43087);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89376(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42304);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 43088);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t31::n89377(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьвировать.
pub(crate) fn n30578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20873);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29201);
			match s {
				[] => true,
				['а', 'в', 'е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 38806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 38807);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word десегрегировать.
pub(crate) fn n88175(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -апеленгировать.
pub(crate) fn n81927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41035);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онгировать.
pub(crate) fn n48213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29213);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 35075);
			match s {
				[] => true,
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огировать.
pub(crate) fn n30583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20878);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29214);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'н', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38818);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 41036);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29215);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35076);
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

/// Words ending -ргировать.
pub(crate) fn n30584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20879);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35077);
					match s {
						[] => true,
						['и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 35078);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38819);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угировать.
pub(crate) fn n30585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20880);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 29216);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'т', 'н', 'е', 'ц', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьгировать.
pub(crate) fn n30586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20881);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29217);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35079);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 38820);
							match s {
								[] => true,
								['с', 'о', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42306);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43089);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89378(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35080);
					match s {
						[] => true,
						['ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 35081);
					match s {
						[] => true,
						['д', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		_ => false,
	}
}

/// Words ending -югировать.
pub(crate) fn n30587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20882);
	match s {
		[] => true,
		['л', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ъ', 'н', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word забаррикадировать.
pub(crate) fn n88177(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word заскладировать.
pub(crate) fn n88178(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -алидировать.
pub(crate) fn n63435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35094);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллидировать.
pub(crate) fn n63436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35095);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 38832);
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

/// Words ending -олидировать.
pub(crate) fn n63437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35096);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 38833);
			match s {
				[] => true,
				['н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42310);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мидировать.
pub(crate) fn n48229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29226);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оидировать.
pub(crate) fn n48230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29227);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 35098);
			match s {
				[] => true,
				['а', 'г', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сидировать.
pub(crate) fn n48231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29228);
	match s {
		[] => true,
		['б', 'у', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38834);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41041);
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
		['к', s @ ..] => {
			super::push(out, 35099);
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

/// Words ending -фидировать.
pub(crate) fn n48232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29229);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 35100);
			match s {
				[] => true,
				['л', 'у', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йдировать.
pub(crate) fn n30591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20886);
	match s {
		[] => true,
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35101);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 35102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38835);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38836);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38837);
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

/// Words ending -лдировать.
pub(crate) fn n30592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20887);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29230);
			match s {
				[] => true,
				['в', 'у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндировать.
pub(crate) fn n30593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20888);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29231);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35103);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38838);
							match s {
								[] => true,
								['п', 'о', 'р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42311);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 43090);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t31::n89379(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35104);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38839);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41042);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42312);
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
				['м', s @ ..] => {
					super::push(out, 35105);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38840);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42313);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 42314);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42315);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 42316);
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29232);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35106);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41043);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41044);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42317);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35107);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38841);
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
				['р', s @ ..] => {
					super::push(out, 35108);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38842);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41045);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42318);
											match s {
												[] => true,
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
									super::push(out, 41046);
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
		['о', s @ ..] => {
			super::push(out, 29233);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 35109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38843);
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
				['п', s @ ..] => {
					super::push(out, 35110);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38844);
							match s {
								[] => true,
								['е', 'р', 'р', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35111);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 38845);
							match s {
								[] => true,
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
					super::push(out, 35112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29234);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35113);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38846);
							match s {
								[] => true,
								['м', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35114);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38847);
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
				['м', s @ ..] => {
					super::push(out, 35115);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38848);
							match s {
								[] => true,
								['о', s @ ..] => super::t31::n81966(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t31::n63459(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одировать.
pub(crate) fn n30594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20889);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29235);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 29236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35117);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38851);
							match s {
								[] => true,
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
					super::push(out, 35118);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38852);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41048);
									match s {
										[] => true,
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
							super::push(out, 38853);
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 29238);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35119);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41049);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41050);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 41051);
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
		['м', s @ ..] => {
			super::push(out, 29239);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35120);
					match s {
						[] => true,
						['к', 'к', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29240);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 35122);
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
		['р', s @ ..] => {
			super::push(out, 29241);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35123);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38854);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41052);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41053);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 35124);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38855);
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
				['э', s @ ..] => {
					super::push(out, 35125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдировать.
pub(crate) fn n30595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20890);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29242);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35126);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 35127);
					match s {
						[] => true,
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
			super::push(out, 29243);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35128);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
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

/// Words ending -удировать.
pub(crate) fn n30596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 29245);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29246);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35129);
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
		['т', s @ ..] => {
			super::push(out, 29247);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 35130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38857);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38858);
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

/// Words ending -ьдировать.
pub(crate) fn n30597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20892);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29248);
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

/// Words ending -юдировать.
pub(crate) fn n30598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20893);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29249);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35131);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35132);
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
		_ => false,
	}
}

/// The word заангажировать.
pub(crate) fn n88187(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word самомассажировать.
pub(crate) fn n88188(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word помассажировать.
pub(crate) fn n88189(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word пошантажировать.
pub(crate) fn n88190(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -еджировать.
pub(crate) fn n48261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29257);
	match s {
		[] => true,
		['н', 'е', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'р', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ф', 'ь', 'л', 'о', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 35143);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38867);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 41062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тежировать.
pub(crate) fn n48262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29258);
	match s {
		[] => true,
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ижировать.
pub(crate) fn n30603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20898);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29259);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35144);
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
		['л', s @ ..] => {
			super::push(out, 29260);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35145);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38868);
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
		['р', s @ ..] => {
			super::push(out, 29261);
			match s {
				[] => true,
				['и', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38869);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41063);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 41064);
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
							super::push(out, 38870);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 41065);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29262);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 35146);
					match s {
						[] => true,
						['л', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нжировать.
pub(crate) fn n30604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20899);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29263);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41066);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42323);
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
		_ => false,
	}
}

/// Words ending -ожировать.
pub(crate) fn n30605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20900);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 29264);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29265);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржировать.
pub(crate) fn n30606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20901);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29266);
			match s {
				[] => true,
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29267);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35148);
					match s {
						[] => true,
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

/// Words ending -тжировать.
pub(crate) fn n30607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20902);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29268);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужировать.
pub(crate) fn n30608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20903);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29269);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжировать.
pub(crate) fn n30609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20904);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29270);
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

/// Words ending -зафантазировать.
pub(crate) fn n88192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43091);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нафантазировать.
pub(crate) fn n88193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43092);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дофантазировать.
pub(crate) fn n88194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43093);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пофантазировать.
pub(crate) fn n88195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43094);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word сфантазировать.
pub(crate) fn n88196(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word гипостазировать.
pub(crate) fn n88197(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тезировать.
pub(crate) fn n48283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29279);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 35158);
			match s {
				[] => true,
				['и', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41067);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42329);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'е', 'н', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -изировать.
pub(crate) fn n30612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20906);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29280);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 35159);
					match s {
						[] => true,
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35160);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 35161);
					match s {
						[] => true,
						['р', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 29281);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35162);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 35163);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 38881);
							match s {
								[] => true,
								['о', 'р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29282);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 38882);
							match s {
								[] => true,
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
					super::push(out, 35165);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 38884);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 41068);
									match s {
										[] => true,
										['л', 'о', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 35166);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41069);
									match s {
										[] => true,
										['м', 'и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 42330);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42331);
											match s {
												[] => true,
												['л', 'л', 'о', 'к', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['ъ', 'б', 'о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 41070);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 42332);
											match s {
												[] => true,
												['е', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35167);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38885);
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
					super::push(out, 35168);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41071);
									match s {
										[] => true,
										['м', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89384(s, out),
														['а', s @ ..] => super::t31::n89385(s, out),
														['с', s @ ..] => super::t31::n89386(s, out),
														_ => false,
													}
												}
												['ы', s @ ..] => {
													super::push(out, 43095);
													match s {
														[] => true,
														['с', s @ ..] => super::t31::n89387(s, out),
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
									super::push(out, 41072);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 42333);
											match s {
												[] => true,
												['ц', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42334);
											match s {
												[] => true,
												['р', s @ ..] => super::t31::n88205(s, out),
												_ => false,
											}
										}
										['о', s @ ..] => super::t31::n86046(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t31::n63537(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t31::n48287(s, out),
		['д', s @ ..] => super::t31::n48288(s, out),
		['е', s @ ..] => super::t31::n48289(s, out),
		['к', s @ ..] => super::t31::n48290(s, out),
		['л', s @ ..] => super::t31::n48291(s, out),
		['м', s @ ..] => super::t31::n48292(s, out),
		['н', s @ ..] => super::t31::n48293(s, out),
		['о', s @ ..] => super::t31::n48294(s, out),
		['п', s @ ..] => super::t31::n48295(s, out),
		['р', s @ ..] => super::t31::n48296(s, out),
		['с', s @ ..] => super::t31::n48297(s, out),
		['т', s @ ..] => super::t31::n48298(s, out),
		['у', s @ ..] => super::t31::n48299(s, out),
		['ф', s @ ..] => super::t31::n48300(s, out),
		['х', s @ ..] => super::t31::n48301(s, out),
		['ц', s @ ..] => super::t31::n48302(s, out),
		['ш', s @ ..] => super::t31::n48303(s, out),
		_ => false,
	}
}

/// Words ending -нзировать.
pub(crate) fn n30613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20907);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29300);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35215);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39005);
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
				['ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39006);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39007);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42419);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42420);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 42421);
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
							super::push(out, 39008);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 41189);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42422);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'р', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29301);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35216);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41190);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41191);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 42423);
											match s {
												[] => true,
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
									super::push(out, 41192);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42424);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озировать.
pub(crate) fn n30614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20908);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 35217);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35218);
					match s {
						[] => true,
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
			super::push(out, 29303);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35219);
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
		['м', s @ ..] => {
			super::push(out, 29304);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35220);
					match s {
						[] => true,
						['т', 'с', 'а', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29305);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35221);
					match s {
						[] => true,
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41193);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29306);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 35222);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 39009);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29307);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35223);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39010);
							match s {
								[] => true,
								['к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35224);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39011);
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

/// The word каузировать.
pub(crate) fn n48312(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word аркебузировать.
pub(crate) fn n48313(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -юзировать.
pub(crate) fn n30616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20910);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 29308);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35225);
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

/// Words ending -аркировать.
pub(crate) fn n48334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29327);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39029);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 39030);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 41199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 41200);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 42425);
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
				['е', s @ ..] => {
					super::push(out, 39031);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41201);
							match s {
								[] => true,
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
					super::push(out, 39032);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41202);
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

/// Words ending -скировать.
pub(crate) fn n30629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20923);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29328);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39033);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39034);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41203);
									match s {
										[] => true,
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
							super::push(out, 39035);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 39036);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41204);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькировать.
pub(crate) fn n30630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20924);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29329);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39037);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39038);
							match s {
								[] => true,
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
					super::push(out, 35245);
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

/// Words ending -дезавуалировать.
pub(crate) fn n88352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43153);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -глировать.
pub(crate) fn n30633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20927);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 29344);
			match s {
				[] => true,
				['о', 'ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39051);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39052);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41209);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word йодлировать.
pub(crate) fn n63656(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -елировать.
pub(crate) fn n30635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20929);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29346);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35259);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41210);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 41211);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42427);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43154);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89457(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35260);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29347);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35261);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41212);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41213);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29348);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39054);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41214);
									match s {
										[] => true,
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
							super::push(out, 39055);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29349);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29350);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35262);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 41215);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 42428);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29351);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 29352);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39057);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39058);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39059);
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
				['у', s @ ..] => {
					super::push(out, 35263);
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
		['р', s @ ..] => {
			super::push(out, 29353);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 29354);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35265);
					match s {
						[] => true,
						['о', 'р', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 29355);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35266);
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

/// Words ending -илировать.
pub(crate) fn n30636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20930);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29356);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35267);
					match s {
						[] => true,
						['н', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29357);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35268);
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
		['м', s @ ..] => {
			super::push(out, 29358);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 29359);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35270);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41216);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41217);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41218);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 41219);
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
				['э', s @ ..] => {
					super::push(out, 35271);
					match s {
						[] => true,
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
			super::push(out, 29360);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35273);
					match s {
						[] => true,
						['ф', 'с', 'о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41220);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41221);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 42429);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29361);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35274);
					match s {
						[] => true,
						['о', 'б', 'р', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 29362);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35275);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 39060);
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
				['к', s @ ..] => {
					super::push(out, 35276);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39061);
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
					super::push(out, 35277);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39062);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42430);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42431);
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
				['у', s @ ..] => {
					super::push(out, 35278);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 39063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 29363);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35279);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 35280);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39064);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41222);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41223);
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
					super::push(out, 35281);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39065);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42432);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42433);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42434);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35282);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39066);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29364);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 35284);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39067);
							match s {
								[] => true,
								['о', 'д', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клировать.
pub(crate) fn n30637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20931);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29365);
			match s {
				[] => true,
				['ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'у', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

