use flexion::Path;

/// Words ending -дарить.
pub(crate) fn n3445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3253);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8734);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8735);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16101);
					match s {
						[] => true,
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
			super::push(out, 8736);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16102);
					match s {
						[] => true,
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
			super::push(out, 8737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8738);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16103);
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
			super::push(out, 8739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16104);
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
			super::push(out, 8740);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16105);
					match s {
						[] => true,
						['и', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31033);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31034);
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
			super::push(out, 8741);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16106);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 16107);
					match s {
						[] => true,
						['а', 'л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31035);
									s.is_empty()
								}
								['з', 'о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31036);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31037);
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
					super::push(out, 16108);
					match s {
						[] => true,
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
			super::push(out, 8743);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16109);
					match s {
						[] => true,
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
			super::push(out, 8744);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16110);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16111);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23770);
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
		_ => false,
	}
}

/// Words ending -жарить.
pub(crate) fn n3446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3254);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8745);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8746);
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
			super::push(out, 8747);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16112);
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
			super::push(out, 8748);
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
			super::push(out, 8749);
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
			super::push(out, 8750);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зарить.
pub(crate) fn n3447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3255);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8751);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8752);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23771);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23772);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31038);
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
							super::push(out, 23773);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31039);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8753);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иарить.
pub(crate) fn n3448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3256);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 8754);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16115);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
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

/// Words ending -карить.
pub(crate) fn n3449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3257);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ж', 'о', 'л', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', 'и', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', 'т', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -марить.
pub(crate) fn n3450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3258);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8755);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8756);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23774);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23775);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 23776);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23777);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31040);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31041);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36371);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8757);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16116);
					match s {
						[] => true,
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
			super::push(out, 8758);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 8759);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16118);
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
		['ш', s @ ..] => {
			super::push(out, 8760);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16119);
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

/// Words ending -нарить.
pub(crate) fn n3451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3259);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8761);
			match s {
				[] => true,
				['л', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23779);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23780);
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
		['с', s @ ..] => {
			super::push(out, 8762);
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

/// Words ending -парить.
pub(crate) fn n3452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3260);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8763);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8764);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16120);
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
		['в', s @ ..] => {
			super::push(out, 8765);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8766);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8767);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8768);
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
				['п', s @ ..] => {
					super::push(out, 16121);
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
		['с', s @ ..] => {
			super::push(out, 8769);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16122);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16123);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16124);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23781);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8770);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8771);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 8772);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16125);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16126);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 16128);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23782);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8773);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сарить.
pub(crate) fn n3453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3261);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8774);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16131);
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
		['с', s @ ..] => {
			super::push(out, 8775);
			match s {
				[] => true,
				['и', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23783);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23784);
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
			super::push(out, 8776);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23785);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23786);
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

/// Words ending -тарить.
pub(crate) fn n3454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3262);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8777);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8778);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23787);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 23788);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31042);
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
		['и', s @ ..] => {
			super::push(out, 8779);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16132);
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
		['о', s @ ..] => {
			super::push(out, 8780);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16133);
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
		['с', s @ ..] => {
			super::push(out, 8781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16134);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16135);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16136);
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
		['у', s @ ..] => {
			super::push(out, 8782);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23789);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23790);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23791);
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
		['ч', s @ ..] => {
			super::push(out, 8783);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16138);
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
			super::push(out, 8784);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23792);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23793);
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
							super::push(out, 23794);
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
							super::push(out, 23795);
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
							super::push(out, 23796);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31043);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уарить.
pub(crate) fn n3455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3263);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 8785);
			match s {
				[] => true,
				['а', 'с', 'к', 'и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23797);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23798);
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

/// Words ending -фарить.
pub(crate) fn n3456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3264);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16139);
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

/// Words ending -харить.
pub(crate) fn n3457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3265);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8787);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8788);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16140);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23799);
							match s {
								[] => true,
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
					super::push(out, 16141);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23800);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23801);
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
							super::push(out, 23802);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31044);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31045);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31046);
									s.is_empty()
								}
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
			super::push(out, 8789);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16142);
					match s {
						[] => true,
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
			super::push(out, 8790);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 16145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -царить.
pub(crate) fn n3458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3266);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8791);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8792);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чарить.
pub(crate) fn n3459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3267);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8793);
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

/// Words ending -шарить.
pub(crate) fn n3460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3268);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8794);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8795);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16148);
					match s {
						[] => true,
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
			super::push(out, 8796);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8797);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8798);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16149);
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 16150);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16151);
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
			super::push(out, 8799);
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
			super::push(out, 8800);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бмизерить.
pub(crate) fn n37327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23833);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31058);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тизерить.
pub(crate) fn n22463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16222);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юзерить.
pub(crate) fn n10697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8849);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 16223);
			match s {
				[] => true,
				['б', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -керить.
pub(crate) fn n3488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3296);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8850);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16224);
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
		['ш', s @ ..] => {
			super::push(out, 8851);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лерить.
pub(crate) fn n3489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3297);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 8852);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16226);
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31059);
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
		['к', s @ ..] => {
			super::push(out, 8853);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16227);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31060);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31061);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мерить.
pub(crate) fn n3490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3298);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8854);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8855);
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
			super::push(out, 8856);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8857);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16228);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23834);
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
					super::push(out, 16229);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23835);
							match s {
								[] => true,
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
					super::push(out, 16230);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23836);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36373);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39777);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83325(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16231);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 23837);
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
		['з', s @ ..] => {
			super::push(out, 8858);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
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
				['и', s @ ..] => {
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
		['и', s @ ..] => {
			super::push(out, 8859);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 8860);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16232);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31062);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31063);
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
			super::push(out, 8861);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16233);
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
				['л', s @ ..] => {
					super::push(out, 16234);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23839);
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
		['т', s @ ..] => {
			super::push(out, 8862);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8863);
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
			super::push(out, 8864);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 8865);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16236);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23840);
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

/// Words ending -нерить.
pub(crate) fn n3491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3299);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8866);
			match s {
				[] => true,
				['ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8867);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16237);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23841);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23842);
							match s {
								[] => true,
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
					super::push(out, 16238);
					match s {
						[] => true,
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16239);
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
		['й', s @ ..] => {
			super::push(out, 8868);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16240);
					match s {
						[] => true,
						['б', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8869);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23843);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31064);
									match s {
										[] => true,
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
							super::push(out, 23844);
							match s {
								[] => true,
								['н', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8870);
			match s {
				[] => true,
				['р', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перить.
pub(crate) fn n3492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3300);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8871);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 8872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8873);
			match s {
				[] => true,
				['ш', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['щ', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 8874);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16241);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23845);
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
		['о', s @ ..] => {
			super::push(out, 8875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8876);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16242);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 23846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -серить.
pub(crate) fn n3493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3301);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8877);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8878);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16243);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16244);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16245);
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
		['с', s @ ..] => {
			super::push(out, 8879);
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

/// Words ending -терить.
pub(crate) fn n3494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3302);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8880);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23847);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 23848);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31065);
									match s {
										[] => true,
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
							super::push(out, 23849);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 23850);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31066);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8881);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16247);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 23851);
							match s {
								[] => true,
								['х', 'у', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8882);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16248);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23852);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31067);
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
							super::push(out, 23853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8883);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16249);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23854);
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
		['с', s @ ..] => {
			super::push(out, 8884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16250);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31068);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16251);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31069);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31070);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16252);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23855);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23856);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31071);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16253);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31072);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 31073);
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
		['т', s @ ..] => {
			super::push(out, 8885);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16254);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23857);
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
		['х', s @ ..] => {
			super::push(out, 8886);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16255);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 23858);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8887);
			match s {
				[] => true,
				['в', 'е', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ферить.
pub(crate) fn n3495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3303);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8888);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 23859);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31074);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херить.
pub(crate) fn n3496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3304);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8889);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8890);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16256);
					match s {
						[] => true,
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
			super::push(out, 8891);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16257);
					match s {
						[] => true,
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
			super::push(out, 8892);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 16258);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23860);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23861);
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

/// Words ending -черить.
pub(crate) fn n3497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3305);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8893);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16259);
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
		['о', s @ ..] => {
			super::push(out, 8894);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8895);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шерить.
pub(crate) fn n3498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3306);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8896);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8897);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 16262);
					match s {
						[] => true,
						['л', 'е', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щерить.
pub(crate) fn n3499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3307);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8898);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8900);
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

/// Words ending -ьерить.
pub(crate) fn n3500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3308);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 8901);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16263);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 23862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8902);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16264);
					match s {
						[] => true,
						['к', 'а', 'р', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31075);
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

/// Words ending -говорить.
pub(crate) fn n22558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16283);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23871);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23872);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31080);
					match s {
						[] => true,
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
			super::push(out, 23873);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 23874);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23875);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 23876);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31081);
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31083);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 23877);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23878);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 31084);
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
					super::push(out, 31085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31086);
					match s {
						[] => true,
						['п', s @ ..] => {
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23879);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23880);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23881);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сворить.
pub(crate) fn n10778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8925);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16285);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16286);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16287);
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

/// Words ending -творить.
pub(crate) fn n10779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8926);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16288);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16289);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23884);
					match s {
						[] => true,
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
		['е', s @ ..] => {
			super::push(out, 16290);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', 'и', 'л', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16291);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16292);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23885);
					match s {
						[] => true,
						['и', 'ж', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36377);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36378);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 23886);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31091);
							match s {
								[] => true,
								['л', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39778);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39779);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 41639);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 39780);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 31092);
							match s {
								[] => true,
								['б', 'о', s @ ..] => {
									super::push(out, 36379);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 23887);
					match s {
						[] => true,
						['о', 'л', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36380);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36381);
									match s {
										[] => true,
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
							super::push(out, 31093);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 36382);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				['р', s @ ..] => {
					super::push(out, 23888);
					match s {
						[] => true,
						['и', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36383);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 36384);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 23889);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 23890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16293);
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
						['о', s @ ..] => {
							super::push(out, 31094);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 36385);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16294);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -горить.
pub(crate) fn n3514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3322);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8927);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дорить.
pub(crate) fn n3515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3323);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8928);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16295);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23892);
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
				['з', s @ ..] => {
					super::push(out, 16296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
		['з', s @ ..] => {
			super::push(out, 8929);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23893);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23894);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31095);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8930);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16298);
					match s {
						[] => true,
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
			super::push(out, 8931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16299);
					match s {
						[] => true,
						['п', 'ш', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -жорить.
pub(crate) fn n3516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3324);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8932);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16300);
					match s {
						[] => true,
						['й', 'а', 'х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зорить.
pub(crate) fn n3517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3325);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8933);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8934);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16301);
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
		['о', s @ ..] => {
			super::push(out, 8935);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23895);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8936);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16304);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 16305);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23897);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31096);
									match s {
										[] => true,
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
							super::push(out, 23898);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -корить.
pub(crate) fn n3518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3326);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8937);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8938);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16306);
					match s {
						[] => true,
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
			super::push(out, 8940);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16307);
					match s {
						[] => true,
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
			super::push(out, 8941);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 8942);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16308);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23899);
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
		['я', s @ ..] => {
			super::push(out, 8943);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16309);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16310);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23900);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -морить.
pub(crate) fn n3519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3327);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8944);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8945);
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
			super::push(out, 8946);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8947);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8948);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16311);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23901);
							match s {
								[] => true,
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
					super::push(out, 16312);
					match s {
						[] => true,
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
			super::push(out, 8949);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8950);
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
			super::push(out, 8951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8952);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 8953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8954);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23902);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23903);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31097);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8955);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16314);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 16315);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23904);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -норить.
pub(crate) fn n3520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3328);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 8956);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23905);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 31098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -порить.
pub(crate) fn n3521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3329);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8957);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16316);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31099);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31100);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 36386);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31101);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											super::push(out, 36387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 23907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31102);
									s.is_empty()
								}
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
			super::push(out, 8958);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16317);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16318);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23908);
							match s {
								[] => true,
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
					super::push(out, 16319);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23909);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16320);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23910);
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
					super::push(out, 16321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23911);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 23912);
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
							super::push(out, 23913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23914);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23915);
							match s {
								[] => true,
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
					super::push(out, 16322);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23916);
							match s {
								[] => true,
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
					super::push(out, 16323);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23917);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8959);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23918);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23919);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 31104);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23920);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23921);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 23922);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16325);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16326);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23923);
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
					super::push(out, 16327);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23924);
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

/// Words ending -сорить.
pub(crate) fn n3522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3330);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8961);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8962);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16329);
					match s {
						[] => true,
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
			super::push(out, 8963);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16330);
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
			super::push(out, 8964);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16332);
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
			super::push(out, 8965);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16333);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16334);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16335);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23925);
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
					super::push(out, 16336);
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
			super::push(out, 8966);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16337);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23926);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23927);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31105);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31106);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торить.
pub(crate) fn n3523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3331);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8967);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8968);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16339);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31107);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31108);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 36388);
											match s {
												[] => true,
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
									super::push(out, 31109);
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
		['в', s @ ..] => {
			super::push(out, 8969);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16340);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16341);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23929);
							match s {
								[] => true,
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
					super::push(out, 16342);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 8970);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16343);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31110);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31111);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31112);
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
		['о', s @ ..] => {
			super::push(out, 8971);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16344);
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
			super::push(out, 8972);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16345);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 16346);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23931);
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
				['г', s @ ..] => {
					super::push(out, 16347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23932);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23933);
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
		['ф', s @ ..] => {
			super::push(out, 8973);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 8974);
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

/// Words ending -форить.
pub(crate) fn n3524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3332);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8975);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16348);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31113);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31114);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36389);
											match s {
												[] => true,
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
									super::push(out, 31115);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36390);
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
		['с', s @ ..] => {
			super::push(out, 8976);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16349);
					match s {
						[] => true,
						['ф', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хорить.
pub(crate) fn n3525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3333);
	match s {
		[] => true,
		['в', 'м', 'а', 'к', '\u{1}', s @ ..] => {
			super::push(out, 8977);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8978);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'а', 'к', '\u{1}', s @ ..] => {
			super::push(out, 8979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ашорить.
pub(crate) fn n10834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8980);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пестрить.
pub(crate) fn n22663(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23944);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23945);
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
			super::push(out, 23946);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23947);
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
			super::push(out, 23948);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -острить.
pub(crate) fn n10840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8986);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16364);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16365);
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
			super::push(out, 16366);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23950);
					match s {
						[] => true,
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
			super::push(out, 16367);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23951);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23952);
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
					super::push(out, 23953);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 23954);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 16368);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23955);
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
			super::push(out, 16369);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16370);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16371);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16373);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устрить.
pub(crate) fn n10841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8987);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 16374);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23957);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 23958);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23959);
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

/// Words ending -ыстрить.
pub(crate) fn n10842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8988);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16375);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23960);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 23961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дежурить.
pub(crate) fn n22698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16394);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23967);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23968);
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
			super::push(out, 23969);
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

/// Words ending -тжурить.
pub(crate) fn n10863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9009);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зурить.
pub(crate) fn n3535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3343);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9010);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16397);
					match s {
						[] => true,
						['г', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9011);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16398);
					match s {
						[] => true,
						['ц', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -курить.
pub(crate) fn n3536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3344);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9012);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9013);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16400);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23970);
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
				['н', s @ ..] => {
					super::push(out, 16401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 23971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31121);
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
			super::push(out, 9014);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9016);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9017);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16403);
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
			super::push(out, 9018);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16404);
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
			super::push(out, 9019);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23972);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36391);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36392);
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
				['п', s @ ..] => {
					super::push(out, 16406);
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
		['р', s @ ..] => {
			super::push(out, 9020);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16407);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9021);
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
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 9022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16408);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16409);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9023);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16410);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мурить.
pub(crate) fn n3537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3345);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16411);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16412);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31123);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31124);
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
		['ж', s @ ..] => {
			super::push(out, 9025);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16413);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16414);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 9026);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16415);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16416);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23974);
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
				['и', s @ ..] => {
					super::push(out, 16417);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нурить.
pub(crate) fn n3538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3346);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9027);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 9028);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16419);
					match s {
						[] => true,
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
			super::push(out, 9029);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пурить.
pub(crate) fn n3539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3347);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9030);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 16421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -турить.
pub(crate) fn n3540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3348);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9031);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9032);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16422);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23976);
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
					super::push(out, 16423);
					match s {
						[] => true,
						['и', 'р', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31125);
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31126);
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31127);
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
									super::push(out, 31128);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36393);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16424);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31129);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 36394);
											match s {
												[] => true,
												['е', 'т', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83328(s, out),
														['а', 'з', '\u{1}', s @ ..] => super::t20::n83329(s, out),
														['о', 'п', '\u{1}', s @ ..] => super::t20::n83330(s, out),
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
									super::push(out, 31130);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 36395);
											match s {
												[] => true,
												['л', s @ ..] => {
													match s {
														[] => true,
														['б', 'о', '\u{1}', s @ ..] => super::t20::n83331(s, out),
														['о', '\u{1}', s @ ..] => super::t20::n83332(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9033);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16425);
					match s {
						[] => true,
						['ф', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16426);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 23977);
							match s {
								[] => true,
								['ъ', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9034);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16427);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31131);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31132);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 31133);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36396);
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
									super::push(out, 31134);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31135);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9035);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16428);
					match s {
						[] => true,
						['г', 'а', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16429);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23978);
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
		['о', s @ ..] => super::t20::n10890(s, out),
		['п', s @ ..] => {
			super::push(out, 9037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t20::n22749(s, out),
				['а', s @ ..] => super::t20::n22750(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t20::n10892(s, out),
		['ь', s @ ..] => super::t20::n10893(s, out),
		_ => false,
	}
}

/// Words ending -шурить.
pub(crate) fn n3541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3349);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9040);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16435);
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

/// Words ending -щурить.
pub(crate) fn n3542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9041);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9042);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9043);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9044);
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

/// Words ending -олесить.
pub(crate) fn n10978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9106);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 16502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24015);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24016);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31145);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 31146);
							match s {
								[] => true,
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
					super::push(out, 24017);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31147);
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
					super::push(out, 24018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31148);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31149);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 31150);
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
					super::push(out, 24019);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 31152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 16503);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24020);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36400);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36401);
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
		_ => false,
	}
}

/// Words ending -месить.
pub(crate) fn n3581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3388);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9107);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9108);
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
		['е', s @ ..] => {
			super::push(out, 9109);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9110);
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
			super::push(out, 9111);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скресить.
pub(crate) fn n22877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16505);
	s.is_empty()
}

/// The word прогундосить.
pub(crate) fn n76866(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ноголосить.
pub(crate) fn n53081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31159);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 36403);
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

/// The word отголосить.
pub(crate) fn n37737(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -полосить.
pub(crate) fn n22912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16519);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -носить.
pub(crate) fn n3597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3404);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9133);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24035);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24036);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31160);
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
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16521);
					match s {
						[] => true,
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
			super::push(out, 9134);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24037);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24038);
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
		['в', s @ ..] => {
			super::push(out, 9135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16523);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 24039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9136);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16524);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24040);
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
					super::push(out, 16525);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31161);
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9137);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9138);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					super::push(out, 16526);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 16527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31162);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31163);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36404);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16529);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24041);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9139);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16530);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 24042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9140);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24043);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24044);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 31164);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31165);
									match s {
										[] => true,
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
							super::push(out, 24045);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31166);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 39781);
													match s {
														[] => true,
														['о', s @ ..] => super::t20::n83333(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24046);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24047);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31167);
									match s {
										[] => true,
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
							super::push(out, 24048);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31168);
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
					super::push(out, 16533);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31169);
									s.is_empty()
								}
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
			super::push(out, 9141);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16534);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16535);
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
			super::push(out, 9142);
			match s {
				[] => true,
				['о', s @ ..] => super::t20::n22931(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9143);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t20::n11030(s, out),
		_ => false,
	}
}

/// Words ending -росить.
pub(crate) fn n3598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3405);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9145);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 9146);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16537);
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
								['о', 'п', s @ ..] => {
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
					super::push(out, 16538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24054);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24055);
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
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24056);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31172);
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
					super::push(out, 16540);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24057);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31173);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31174);
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
			super::push(out, 9147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24058);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24059);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31175);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31176);
									match s {
										[] => true,
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
							super::push(out, 24060);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24061);
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
				['н', s @ ..] => {
					super::push(out, 16542);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24062);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31177);
									match s {
										[] => true,
										['е', 'ъ', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24063);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24064);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24065);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31178);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36405);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36406);
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
				['т', s @ ..] => {
					super::push(out, 16544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24066);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24067);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31179);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t20::n11034(s, out),
		['т', s @ ..] => {
			super::push(out, 9149);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16551);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31181);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31182);
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

/// Words ending -сосить.
pub(crate) fn n3599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3406);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9150);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16552);
					match s {
						[] => true,
						['ы', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31183);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36407);
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
				['у', s @ ..] => {
					super::push(out, 16553);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 24077);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31184);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31185);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36408);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word недофорсить.
pub(crate) fn n76874(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -бтрусить.
pub(crate) fn n22997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16579);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24088);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дтрусить.
pub(crate) fn n22998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16580);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24089);
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

/// Words ending -етрусить.
pub(crate) fn n22999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16581);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 24090);
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

/// Words ending -итрусить.
pub(crate) fn n23000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16582);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 24091);
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

/// Words ending -отрусить.
pub(crate) fn n23001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16583);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 24092);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24093);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -струсить.
pub(crate) fn n23002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16584);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24094);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 24095);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word утрусить.
pub(crate) fn n37839(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ытрусить.
pub(crate) fn n23004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16586);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 24096);
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

/// Words ending -тусить.
pub(crate) fn n3609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3415);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9175);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9176);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16587);
					match s {
						[] => true,
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
			super::push(out, 9177);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16589);
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

/// The word обвиноватить.
pub(crate) fn n76875(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -акатить.
pub(crate) fn n11090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9196);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 16608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word обкатить.
pub(crate) fn n23042(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -вкатить.
pub(crate) fn n11092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9198);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дкатить.
pub(crate) fn n11093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9199);
	match s {
		[] => true,
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -екатить.
pub(crate) fn n11094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9200);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 16610);
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

/// Words ending -икатить.
pub(crate) fn n11095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9201);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окатить.
pub(crate) fn n11096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9202);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 16611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16613);
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

/// Words ending -скатить.
pub(crate) fn n11097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9203);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16614);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ткатить.
pub(crate) fn n11098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9204);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укатить.
pub(crate) fn n11099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9205);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкатить.
pub(crate) fn n11100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9206);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16617);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -латить.
pub(crate) fn n3619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3425);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9207);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16618);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24106);
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
		['з', s @ ..] => {
			super::push(out, 9208);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16619);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 24107);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9209);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16621);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16622);
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
					super::push(out, 16623);
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
								['е', s @ ..] => {
									match s {
										[] => true,
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
					super::push(out, 16624);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 16625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 16626);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24108);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9210);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16627);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31192);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31193);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36412);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9211);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16628);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24109);
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
		_ => false,
	}
}

/// Words ending -матить.
pub(crate) fn n3620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3426);
	match s {
		[] => true,
		['с', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16629);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16630);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24110);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31194);
									match s {
										[] => true,
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
							super::push(out, 24111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9212);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24112);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24113);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31195);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 31196);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 31197);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9213);
			match s {
				[] => true,
				['о', 'л', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24114);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24115);
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
						['о', 'п', '\u{1}', s @ ..] => {
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
								['и', s @ ..] => {
									super::push(out, 31198);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36413);
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
							super::push(out, 24116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натить.
pub(crate) fn n3621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3427);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9214);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24117);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24118);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31200);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36414);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9215);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16631);
					match s {
						[] => true,
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
			super::push(out, 9216);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16632);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31201);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 31202);
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
		_ => false,
	}
}

/// Words ending -оатить.
pub(crate) fn n3622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3428);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9217);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 16633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -патить.
pub(crate) fn n3623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3429);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9218);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24119);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24120);
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
					super::push(out, 16635);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31203);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 31204);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36415);
											match s {
												[] => true,
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
									super::push(out, 31205);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36416);
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
									super::push(out, 31206);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36417);
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
									super::push(out, 31207);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36418);
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
									super::push(out, 31208);
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
								['т', s @ ..] => {
									super::push(out, 31209);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 31210);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 31211);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36419);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16636);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратить.
pub(crate) fn n3624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3430);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9219);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16637);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9220);
			match s {
				[] => true,
				['з', 'а', 'р', s @ ..] => {
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
				['т', 'о', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 9221);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16638);
					match s {
						[] => true,
						['в', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31212);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31213);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9222);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16639);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24122);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24123);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9223);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16640);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24124);
							match s {
								[] => true,
								['д', 'д', 'о', 'п', '\u{1}', s @ ..] => {
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
						['р', s @ ..] => {
							super::push(out, 24125);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 31214);
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['а', 'з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['з', 'а', 'р', '\u{1}', s @ ..] => {
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
						['с', s @ ..] => {
							super::push(out, 24126);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9224);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16642);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16643);
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
					super::push(out, 16644);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24127);
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
							super::push(out, 24128);
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
				['с', s @ ..] => {
					super::push(out, 16645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24129);
							s.is_empty()
						}
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31215);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36420);
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
							super::push(out, 24130);
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
				['у', s @ ..] => {
					super::push(out, 16646);
					match s {
						[] => true,
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
			super::push(out, 9225);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16647);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24131);
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
		['х', s @ ..] => {
			super::push(out, 9226);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16648);
					match s {
						[] => true,
						['н', 'з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хатить.
pub(crate) fn n3625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3431);
	match s {
		[] => true,
		['о', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16649);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16650);
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
		['ю', s @ ..] => {
			super::push(out, 9227);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24132);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24133);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word предвосхитить.
pub(crate) fn n76886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39782);
	s.is_empty()
}

/// Words ending -глотить.
pub(crate) fn n11196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9297);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16736);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16737);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24190);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24191);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олотить.
pub(crate) fn n11197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9298);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16738);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24192);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24193);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 16739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24194);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24195);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 24196);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24197);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31240);
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
					super::push(out, 24198);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31241);
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
					super::push(out, 24199);
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
					super::push(out, 24200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31242);
							match s {
								[] => true,
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
					super::push(out, 24201);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31243);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 16740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24202);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24203);
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
					super::push(out, 24204);
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
					super::push(out, 24205);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 24206);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31244);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 16741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24207);
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
					super::push(out, 24208);
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
					super::push(out, 24209);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плотить.
pub(crate) fn n11198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9299);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16742);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16743);
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
			super::push(out, 16744);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24210);
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
			super::push(out, 16745);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
		['с', s @ ..] => {
			super::push(out, 16746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слотить.
pub(crate) fn n11199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9300);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мотить.
pub(crate) fn n3662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3468);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9301);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16748);
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

/// Words ending -нотить.
pub(crate) fn n3663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3469);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9302);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16749);
					match s {
						[] => true,
						['л', 'б', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ротить.
pub(crate) fn n3664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3470);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9303);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16750);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31246);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31247);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36424);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9304);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24212);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24213);
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
		['к', s @ ..] => {
			super::push(out, 9305);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16752);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24214);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36425);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36426);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9306);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24215);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24216);
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
							super::push(out, 24217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31248);
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
							super::push(out, 24218);
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
							super::push(out, 24219);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31249);
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
							super::push(out, 24220);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31250);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24221);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 24222);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31251);
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
				['с', s @ ..] => {
					super::push(out, 16756);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24223);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31252);
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
		_ => false,
	}
}

/// Words ending -сотить.
pub(crate) fn n3665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3471);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9307);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16757);
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
		['х', s @ ..] => {
			super::push(out, 9308);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16758);
					match s {
						[] => true,
						['в', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31253);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31254);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 36427);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31255);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хотить.
pub(crate) fn n3666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3472);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16759);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16760);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16761);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16762);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16763);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24224);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31256);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31257);
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
					super::push(out, 16764);
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

/// Words ending -шотить.
pub(crate) fn n3667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3473);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9310);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16765);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24225);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31258);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31259);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16766);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24226);
							match s {
								[] => true,
								['к', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36428);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -угуртить.
pub(crate) fn n23276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16789);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 24236);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -растить.
pub(crate) fn n11224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9324);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16804);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16805);
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
		['з', s @ ..] => {
			super::push(out, 16806);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16807);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16808);
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 16809);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24247);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31266);
							match s {
								[] => true,
								['к', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24248);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24249);
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
		['ы', s @ ..] => {
			super::push(out, 16810);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ачастить.
pub(crate) fn n23301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16812);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 24251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word причастить.
pub(crate) fn n38194(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -участить.
pub(crate) fn n23303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16814);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -естить.
pub(crate) fn n3675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3481);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9326);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16815);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 16816);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24252);
							match s {
								[] => true,
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
					super::push(out, 16817);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16818);
					match s {
						[] => true,
						['о', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 16819);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24253);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31267);
									match s {
										[] => true,
										['л', 'б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39783);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 39784);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t20::n83334(s, out),
														_ => false,
													}
												}
												['з', s @ ..] => {
													super::push(out, 39785);
													match s {
														[] => true,
														['а', s @ ..] => super::t20::n83335(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 39786);
													match s {
														[] => true,
														['о', '\u{1}', s @ ..] => super::t20::n83336(s, out),
														_ => false,
													}
												}
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
							super::push(out, 24254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31268);
									match s {
										[] => true,
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
							super::push(out, 24255);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31269);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31270);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 36430);
											match s {
												[] => true,
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
									super::push(out, 31271);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16820);
					match s {
						[] => true,
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
			super::push(out, 9327);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9328);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16822);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 24256);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31272);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31273);
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
				['у', s @ ..] => {
					super::push(out, 16823);
					match s {
						[] => true,
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
			super::push(out, 9329);
			match s {
				[] => true,
				['в', s @ ..] => {
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
				['з', 'а', 'р', s @ ..] => {
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
				['ы', s @ ..] => {
					super::push(out, 16824);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9330);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24258);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24259);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 31274);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24260);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31275);
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
							super::push(out, 24261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									super::push(out, 31276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t20::n38214(s, out),
						['ы', s @ ..] => super::t20::n38215(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t20::n11231(s, out),
		['ч', s @ ..] => super::t20::n11232(s, out),
		_ => false,
	}
}

/// Words ending -истить.
pub(crate) fn n3676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3482);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9333);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16830);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24266);
							match s {
								[] => true,
								['р', 'у', 'ж', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16831);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16832);
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
					super::push(out, 16833);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24267);
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
				['д', s @ ..] => {
					super::push(out, 16834);
					match s {
						[] => true,
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
					super::push(out, 16835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
					super::push(out, 16836);
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
					super::push(out, 16837);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кстить.
pub(crate) fn n3677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3483);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9335);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лстить.
pub(crate) fn n3678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3484);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9336);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24270);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 24271);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мстить.
pub(crate) fn n3679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3485);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9337);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9338);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16838);
					match s {
						[] => true,
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
			super::push(out, 9339);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16839);
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
		['т', s @ ..] => {
			super::push(out, 9340);
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

/// Words ending -остить.
pub(crate) fn n3680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3486);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9341);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 16840);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24272);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36433);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 36434);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 36435);
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
							super::push(out, 24273);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36436);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 36437);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 36438);
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
							super::push(out, 24274);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31278);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 36439);
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
						['у', s @ ..] => {
							super::push(out, 24275);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31279);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36440);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36441);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 39787);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83337(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16841);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16842);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24276);
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
					super::push(out, 16843);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24277);
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
				['т', s @ ..] => {
					super::push(out, 16844);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24278);
							match s {
								[] => true,
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
					super::push(out, 16845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 16846);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24279);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16847);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16848);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31280);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31281);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36442);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 36443);
											match s {
												[] => true,
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
									super::push(out, 31282);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31283);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31284);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36444);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 36445);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16849);
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
		['л', s @ ..] => super::t20::n11244(s, out),
		['м', s @ ..] => super::t20::n11245(s, out),
		['п', s @ ..] => super::t20::n11246(s, out),
		['р', s @ ..] => super::t20::n11247(s, out),
		_ => false,
	}
}

/// Words ending -рстить.
pub(crate) fn n3681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3487);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9348);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24286);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24287);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31292);
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
							super::push(out, 24288);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31293);
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

/// Words ending -устить.
pub(crate) fn n3682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9349);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16862);
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
				['с', s @ ..] => {
					super::push(out, 16863);
					match s {
						[] => true,
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
			super::push(out, 9351);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16864);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9352);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16865);
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
			super::push(out, 9353);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24290);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 24291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9354);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16867);
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
					super::push(out, 16868);
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
					super::push(out, 16869);
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
				['т', 'о', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 16870);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24292);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9355);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16871);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24293);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24294);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24295);
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

/// Words ending -ьстить.
pub(crate) fn n3683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3489);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9356);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16872);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16873);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24296);
							match s {
								[] => true,
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
					super::push(out, 16874);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16875);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
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

/// Words ending -забаламутить.
pub(crate) fn n76905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39788);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -набаламутить.
pub(crate) fn n76906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39789);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word перебаламутить.
pub(crate) fn n76907(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word сбаламутить.
pub(crate) fn n76908(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -апутить.
pub(crate) fn n11273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9373);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 16891);
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

/// Words ending -рутить.
pub(crate) fn n3689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3495);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9374);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16892);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16893);
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
								_ => false,
							}
						}
						['н', s @ ..] => {
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
					super::push(out, 16894);
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
				['с', s @ ..] => {
					super::push(out, 16895);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 16896);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9375);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16897);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24307);
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
		_ => false,
	}
}

/// Words ending -тутить.
pub(crate) fn n3690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3496);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9376);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16898);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16899);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шутить.
pub(crate) fn n3691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3497);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9377);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9378);
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
			super::push(out, 9379);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9380);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16900);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24309);
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
			super::push(out, 9381);
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 9382);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щутить.
pub(crate) fn n3692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3498);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9383);
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

/// Words ending -запротиворечить.
pub(crate) fn n53514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31314);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уречить.
pub(crate) fn n11389(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16993);
			match s {
				[] => true,
				['и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16994);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щичить.
pub(crate) fn n3748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3554);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 9492);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17009);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word повыскочить.
pub(crate) fn n76911(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лочить.
pub(crate) fn n3758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3564);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9508);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 9509);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17039);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17040);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17041);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17042);
					match s {
						[] => true,
						['т', 'о', 'м', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9510);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17043);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 17044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24390);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24391);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24392);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24393);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31324);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9511);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17045);
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
		['к', s @ ..] => {
			super::push(out, 9512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17046);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17047);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9513);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24394);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24395);
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
						['з', s @ ..] => {
							super::push(out, 24396);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24397);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31325);
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
							super::push(out, 24398);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31326);
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
							super::push(out, 24399);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31327);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мочить.
pub(crate) fn n3759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3565);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9514);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9515);
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
			super::push(out, 9516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9517);
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
		['з', s @ ..] => {
			super::push(out, 9518);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24400);
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
		['и', s @ ..] => {
			super::push(out, 9519);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17051);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17052);
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
			super::push(out, 9521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9522);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 9523);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лночить.
pub(crate) fn n11447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9524);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17055);
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

/// Words ending -сночить.
pub(crate) fn n11448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9525);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17056);
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

/// Words ending -почить.
pub(crate) fn n3761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3567);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рочить.
pub(crate) fn n3762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3568);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17057);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17058);
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
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17059);
					match s {
						[] => true,
						['д', 'е', 'н', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 9528);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24402);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24403);
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
						['о', s @ ..] => {
							super::push(out, 24404);
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
					super::push(out, 17061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24405);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24406);
							match s {
								[] => true,
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
					super::push(out, 17062);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24407);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31328);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 36453);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17063);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24409);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24410);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 24411);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 24412);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24413);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31329);
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
							super::push(out, 24414);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31330);
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
							super::push(out, 24415);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24416);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31331);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17065);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17066);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17067);
					match s {
						[] => true,
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
			super::push(out, 9530);
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9531);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24417);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24418);
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
							super::push(out, 24419);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31332);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
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
						['с', s @ ..] => super::t20::n38589(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t20::n38590(s, out),
						['ы', s @ ..] => super::t20::n38591(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t20::n11457(s, out),
		_ => false,
	}
}

/// Words ending -сочить.
pub(crate) fn n3763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3569);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9533);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9534);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17072);
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
			super::push(out, 9535);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24424);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24425);
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
		['ы', s @ ..] => {
			super::push(out, 9536);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17074);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точить.
pub(crate) fn n3764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3570);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9537);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9538);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17076);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17077);
					match s {
						[] => true,
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
			super::push(out, 9539);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17078);
					match s {
						[] => true,
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
			super::push(out, 9540);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9541);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17079);
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
			super::push(out, 9542);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17080);
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
			super::push(out, 9543);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17081);
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
			super::push(out, 9544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17082);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24427);
							match s {
								[] => true,
								['р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24428);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24429);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 31336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36455);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31337);
									match s {
										[] => true,
										['с', 'о', 'с', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17084);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17086);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24431);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36456);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36457);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 39790);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83340(s, out),
														_ => false,
													}
												}
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
							super::push(out, 24432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17087);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17088);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 24433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17089);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24434);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31338);
									match s {
										[] => true,
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
							super::push(out, 24435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17090);
					match s {
						[] => true,
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
			super::push(out, 9546);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17091);
					match s {
						[] => true,
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
			super::push(out, 9547);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9548);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17092);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 24436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31339);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 31340);
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
					super::push(out, 17093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кучить.
pub(crate) fn n3782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3588);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9579);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24445);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24446);
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
		['с', s @ ..] => {
			super::push(out, 9581);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17122);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17123);
					match s {
						[] => true,
						['н', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17124);
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

/// Words ending -лучить.
pub(crate) fn n3783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3589);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9582);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9583);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9584);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17125);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24447);
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
				['о', s @ ..] => {
					super::push(out, 17126);
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
		['з', s @ ..] => {
			super::push(out, 9585);
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
			super::push(out, 9586);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9587);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 9588);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9589);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9590);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мучить.
pub(crate) fn n3784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3590);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9591);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9592);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 9593);
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
			super::push(out, 9594);
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
			super::push(out, 9595);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17128);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пучить.
pub(crate) fn n3786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3592);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9597);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9598);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17129);
					match s {
						[] => true,
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
			super::push(out, 9599);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17130);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24448);
							match s {
								[] => true,
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
					super::push(out, 17131);
					match s {
						[] => true,
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
			super::push(out, 9600);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ручить.
pub(crate) fn n3787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3593);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9601);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9602);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9603);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9604);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17135);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9605);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9606);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9607);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17138);
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
		_ => false,
	}
}

/// Words ending -сучить.
pub(crate) fn n3788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3594);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9608);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9609);
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
			super::push(out, 9610);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9611);
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
		['р', s @ ..] => {
			super::push(out, 9612);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17139);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9613);
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
			super::push(out, 9614);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тучить.
pub(crate) fn n3789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3595);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9615);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9616);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9617);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 9618);
			match s {
				[] => true,
				['д', 'н', 'у', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24451);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
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

/// Words ending -щучить.
pub(crate) fn n3790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3596);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9619);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 9620);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ыучить.
pub(crate) fn n3791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3597);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9621);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яучить.
pub(crate) fn n3792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3598);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 9622);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17141);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17142);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17143);
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

/// Words ending -окорячить.
pub(crate) fn n38725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24472);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скорячить.
pub(crate) fn n38726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24473);
	match s {
		[] => true,
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -морячить.
pub(crate) fn n23866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17173);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24474);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трячить.
pub(crate) fn n11597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9647);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 17174);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24476);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36459);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 36460);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 24477);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 31350);
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сячить.
pub(crate) fn n3812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3617);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9648);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24478);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24479);
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

/// Words ending -уячить.
pub(crate) fn n3813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3618);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 9649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17175);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17176);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24481);
							match s {
								[] => true,
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
					super::push(out, 17177);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24482);
							match s {
								[] => true,
								['е', 'п', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 17178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24483);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24484);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17180);
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

/// Words ending -ьячить.
pub(crate) fn n3814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3619);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9650);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9651);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24485);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31352);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тешить.
pub(crate) fn n3830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3634);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9682);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9683);
			match s {
				[] => true,
				['н', s @ ..] => {
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
			super::push(out, 9684);
			match s {
				[] => true,
				['т', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9685);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17210);
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
		['у', s @ ..] => {
			super::push(out, 9686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 9687);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17211);
					match s {
						[] => true,
						['е', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31353);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31354);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31355);
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

/// Words ending -рошить.
pub(crate) fn n3846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3650);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9707);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 17221);
					match s {
						[] => true,
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
			super::push(out, 9708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17222);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17223);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 17224);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24511);
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
		['к', s @ ..] => {
			super::push(out, 9709);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17225);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17226);
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
					super::push(out, 17227);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17228);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17229);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17230);
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
					super::push(out, 17231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24512);
							match s {
								[] => true,
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
					super::push(out, 17232);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24513);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9710);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24514);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24515);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31359);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31360);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['з', s @ ..] => {
							super::push(out, 24516);
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
					super::push(out, 17234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24517);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24518);
							match s {
								[] => true,
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
					super::push(out, 17235);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24519);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31361);
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
				['п', s @ ..] => {
					super::push(out, 17236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24520);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24521);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17237);
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
		['п', s @ ..] => {
			super::push(out, 9711);
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
		['т', s @ ..] => {
			super::push(out, 9712);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17238);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31362);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31363);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36462);
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
									super::push(out, 31364);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 36463);
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
									super::push(out, 31365);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36464);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атошить.
pub(crate) fn n11672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9713);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 17239);
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

/// Words ending -отошить.
pub(crate) fn n11673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9714);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24522);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24523);
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

/// Words ending -стошить.
pub(crate) fn n11674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9715);
	match s {
		[] => true,
		['у', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24524);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оушить.
pub(crate) fn n3863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3667);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9736);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17261);
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

/// Words ending -пушить.
pub(crate) fn n3864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3668);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9737);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9738);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17264);
					match s {
						[] => true,
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
			super::push(out, 9739);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9740);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24543);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24544);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24545);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31370);
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
			super::push(out, 9742);
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
		['ы', s @ ..] => {
			super::push(out, 9743);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17266);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рушить.
pub(crate) fn n3865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3669);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9744);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9745);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17267);
					match s {
						[] => true,
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
			super::push(out, 9746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17268);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17269);
					match s {
						[] => true,
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
			super::push(out, 9747);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17270);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31371);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 36465);
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
			super::push(out, 9748);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17271);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17272);
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
		['о', s @ ..] => {
			super::push(out, 9749);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17273);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сушить.
pub(crate) fn n3866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3670);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9750);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9751);
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
			super::push(out, 9752);
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
		['с', s @ ..] => {
			super::push(out, 9753);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17274);
					match s {
						[] => true,
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 9754);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24546);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31372);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушить.
pub(crate) fn n3867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9755);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9756);
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
			super::push(out, 9757);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24547);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24548);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31373);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17277);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24549);
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9758);
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
			super::push(out, 9759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17278);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24550);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
		_ => false,
	}
}

/// Words ending -ябнуть.
pub(crate) fn n3997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3762);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 9910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17371);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17372);
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
					super::push(out, 17373);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17374);
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
				['ы', s @ ..] => {
					super::push(out, 17375);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9911);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24592);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24593);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 24594);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24595);
							match s {
								[] => true,
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
					super::push(out, 17377);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17378);
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

/// Words ending -згнуть.
pub(crate) fn n4004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3768);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9922);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24609);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31394);
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
							super::push(out, 24610);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24611);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31395);
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
			super::push(out, 9923);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24612);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24613);
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
		['ю', s @ ..] => {
			super::push(out, 9924);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17393);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31396);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31397);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9925);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игнуть.
pub(crate) fn n4005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3769);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9926);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24614);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24615);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31398);
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
						['о', s @ ..] => {
							super::push(out, 24616);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31399);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 36467);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9927);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 9928);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24617);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24618);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31400);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9929);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
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
		['т', s @ ..] => {
			super::push(out, 9930);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17396);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24619);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31401);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36468);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -волгнуть.
pub(crate) fn n24312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17398);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24620);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 24621);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31402);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

