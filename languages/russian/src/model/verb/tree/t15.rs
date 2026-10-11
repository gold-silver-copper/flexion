use flexion::Path;

/// Words ending -огнуть.
pub(crate) fn n4007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3771);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17399);
					match s {
						[] => true,
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
			super::push(out, 9934);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9936);
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
		['м', s @ ..] => {
			super::push(out, 9937);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17400);
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
		['п', s @ ..] => {
			super::push(out, 9938);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9939);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24622);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24623);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24624);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 24625);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 31404);
									match s {
										[] => true,
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
							super::push(out, 24626);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31405);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31406);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36469);
											s.is_empty()
										}
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
					super::push(out, 17402);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17403);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24627);
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
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9940);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17404);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17405);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24628);
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
					super::push(out, 17406);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24629);
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
		['т', s @ ..] => {
			super::push(out, 9941);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргнуть.
pub(crate) fn n4008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3772);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9942);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 24630);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 24631);
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31407);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36470);
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											super::push(out, 36471);
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
								['п', s @ ..] => {
									super::push(out, 31408);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36472);
											s.is_empty()
										}
										['с', 'и', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 36473);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 36474);
											match s {
												[] => true,
												['и', 'н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 24632);
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							super::push(out, 24633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9943);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17408);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24634);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 24635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17409);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 31410);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 31411);
									match s {
										[] => true,
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
							super::push(out, 24636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угнуть.
pub(crate) fn n4009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3773);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9944);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 9945);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17410);
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
				['ы', s @ ..] => {
					super::push(out, 17411);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24637);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9946);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17412);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17413);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24638);
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
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 9947);
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

/// Words ending -ыгнуть.
pub(crate) fn n4010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3774);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9948);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9949);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 17415);
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
						['о', s @ ..] => {
							super::push(out, 24639);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24640);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31412);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9950);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17416);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17418);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24641);
							match s {
								[] => true,
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
					super::push(out, 17419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24642);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24643);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31413);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 36475);
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
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24644);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24645);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31414);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17421);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24647);
							match s {
								[] => true,
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
					super::push(out, 17422);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24648);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 31415);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -югнуть.
pub(crate) fn n4011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3775);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 9951);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17423);
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

/// Words ending -ягнуть.
pub(crate) fn n4012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3776);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9952);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9953);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17424);
					match s {
						[] => true,
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
			super::push(out, 9954);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вымерзнуть.
pub(crate) fn n53813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31418);
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

/// Words ending -ъерзнуть.
pub(crate) fn n24379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17441);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 24666);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узнуть.
pub(crate) fn n4028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3792);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9970);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17442);
					match s {
						[] => true,
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
			super::push(out, 9971);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24667);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24668);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31419);
									match s {
										[] => true,
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
							super::push(out, 24669);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31420);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17444);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24670);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31421);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ызнуть.
pub(crate) fn n4029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3793);
	match s {
		[] => true,
		['л', 'а', 'б', 'р', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9972);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24671);
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24672);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31423);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36476);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24673);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31424);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьзнуть.
pub(crate) fn n4030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3794);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9973);
			match s {
				[] => true,
				['о', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24675);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24676);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31425);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язнуть.
pub(crate) fn n4031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3795);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9974);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17447);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17448);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31426);
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24677);
							match s {
								[] => true,
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
					super::push(out, 17449);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9975);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24679);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24680);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24681);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24682);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -минуть.
pub(crate) fn n4037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3801);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9984);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9985);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17454);
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
			super::push(out, 9986);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17455);
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

/// Words ending -ринуть.
pub(crate) fn n4038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3802);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'и', s @ ..] => {
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
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явкнуть.
pub(crate) fn n12118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10000);
	match s {
		[] => true,
		['р', s @ ..] => {
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

/// Words ending -екнуть.
pub(crate) fn n4043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3807);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10001);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10002);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24694);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24695);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31431);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24696);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24697);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31432);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17474);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17475);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17476);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24698);
							match s {
								[] => true,
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
					super::push(out, 17477);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17478);
					match s {
						[] => true,
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
			super::push(out, 10004);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17479);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17480);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17481);
					match s {
						[] => true,
						['с', 'д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17482);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24700);
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
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10005);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17483);
					match s {
						[] => true,
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
			super::push(out, 10006);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10007);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 10008);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -икнуть.
pub(crate) fn n4044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3808);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10009);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 10010);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10011);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17486);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17487);
					match s {
						[] => true,
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
			super::push(out, 10014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10015);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17488);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 24702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17489);
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
						['о', s @ ..] => {
							super::push(out, 24703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 31433);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31434);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36477);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 24705);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31435);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['н', s @ ..] => {
			super::push(out, 10016);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17490);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17492);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17493);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 24706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17494);
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
					super::push(out, 17495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 17496);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10017);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10018);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17498);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24708);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 31436);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 24709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24710);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24711);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24712);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31437);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 31438);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['ы', s @ ..] => {
							super::push(out, 24715);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31439);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10019);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17500);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24716);
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
		['х', s @ ..] => {
			super::push(out, 10020);
			match s {
				[] => true,
				['и', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24717);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31440);
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
		['ч', s @ ..] => {
			super::push(out, 10021);
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
		['ш', s @ ..] => super::t20::n12141(s, out),
		['ы', s @ ..] => super::t20::n12142(s, out),
		_ => false,
	}
}

/// Words ending -йкнуть.
pub(crate) fn n4045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3809);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', '\u{1}', s @ ..] => {
					super::push(out, 17504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10025);
			match s {
				[] => true,
				['г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лкнуть.
pub(crate) fn n4046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3810);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10027);
			match s {
				[] => true,
				['ж', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 17505);
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
						['д', s @ ..] => {
							super::push(out, 24718);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31441);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36478);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24719);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31442);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10028);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 17506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24720);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24721);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31443);
									match s {
										[] => true,
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
							super::push(out, 24722);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31444);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36479);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31445);
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
					super::push(out, 17507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24723);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24724);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36480);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 36481);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 36482);
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
							super::push(out, 24725);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36483);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36484);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39791);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83341(s, out),
														_ => false,
													}
												}
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
							super::push(out, 24726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24727);
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
				['т', s @ ..] => {
					super::push(out, 17508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24728);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24729);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31448);
									match s {
										[] => true,
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
							super::push(out, 24730);
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
							super::push(out, 24731);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24732);
							match s {
								[] => true,
								['р', s @ ..] => super::t20::n53886(s, out),
								_ => false,
							}
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => super::t20::n39293(s, out),
						['с', s @ ..] => super::t20::n39294(s, out),
						['т', s @ ..] => super::t20::n39295(s, out),
						['ы', s @ ..] => super::t20::n39296(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мкнуть.
pub(crate) fn n4047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3811);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10029);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 17511);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17512);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17513);
					match s {
						[] => true,
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
			super::push(out, 10030);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10031);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10032);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17515);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17517);
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10033);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17519);
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
		_ => false,
	}
}

/// Words ending -окнуть.
pub(crate) fn n4048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3812);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10034);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 10035);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17520);
					match s {
						[] => true,
						['г', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10036);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
		['л', s @ ..] => {
			super::push(out, 10037);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17521);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24738);
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
		['м', s @ ..] => {
			super::push(out, 10038);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17522);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17523);
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
					super::push(out, 17524);
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
					super::push(out, 17525);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17526);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24739);
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
				['ч', s @ ..] => {
					super::push(out, 17527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24740);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24741);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31454);
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
					super::push(out, 17528);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10039);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 17529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10040);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17530);
					match s {
						[] => true,
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
			super::push(out, 10041);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17531);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24743);
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
			super::push(out, 10042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркнуть.
pub(crate) fn n4049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3813);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10043);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17532);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 24744);
							match s {
								[] => true,
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
					super::push(out, 17533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24745);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31455);
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
					super::push(out, 17534);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 17535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24746);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 31456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24747);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24748);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24749);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24750);
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
		['е', s @ ..] => {
			super::push(out, 10044);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17537);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31458);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36485);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24752);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24753);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31459);
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
							super::push(out, 24754);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31460);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36486);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31461);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31462);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24756);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24757);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 24758);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24759);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24760);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31463);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24761);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31464);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10045);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17540);
					match s {
						[] => true,
						['ц', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 17541);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17542);
					match s {
						[] => true,
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
			super::push(out, 10046);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t20::n39346(s, out),
						['е', s @ ..] => super::t20::n39347(s, out),
						['о', s @ ..] => super::t20::n39348(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t20::n24551(s, out),
				['т', s @ ..] => super::t20::n24552(s, out),
				['ф', s @ ..] => super::t20::n24553(s, out),
				['ш', s @ ..] => super::t20::n24554(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t20::n12166(s, out),
		['ы', s @ ..] => super::t20::n12167(s, out),
		['ю', s @ ..] => super::t20::n12168(s, out),
		_ => false,
	}
}

/// Words ending -скнуть.
pub(crate) fn n4050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3814);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10051);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17559);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10052);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 10053);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24773);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24774);
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
			super::push(out, 10054);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17563);
					match s {
						[] => true,
						['у', 'ч', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10055);
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

/// Words ending -ткнуть.
pub(crate) fn n4051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3815);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10056);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10057);
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10058);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17564);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24775);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
			super::push(out, 10059);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укнуть.
pub(crate) fn n4052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3816);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10060);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10061);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17567);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24776);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10062);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10063);
			match s {
				[] => true,
				['у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10064);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10065);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17569);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24777);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10066);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10067);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17570);
					match s {
						[] => true,
						['т', s @ ..] => {
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
		['т', s @ ..] => {
			super::push(out, 10068);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17572);
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
		['ф', s @ ..] => {
			super::push(out, 10069);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 10070);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17573);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыцкнуть.
pub(crate) fn n12193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10071);
	match s {
		[] => true,
		['ц', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чкнуть.
pub(crate) fn n4054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3818);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10072);
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

/// Words ending -шкнуть.
pub(crate) fn n4055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3819);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10073);
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

/// Words ending -ыкнуть.
pub(crate) fn n4056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3820);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10074);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 10075);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17574);
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
		['в', s @ ..] => {
			super::push(out, 10076);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 17575);
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
				['и', s @ ..] => {
					super::push(out, 17576);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31472);
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
				['т', s @ ..] => {
					super::push(out, 17577);
					match s {
						[] => true,
						['о', s @ ..] => {
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
		['г', s @ ..] => {
			super::push(out, 10077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10078);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17579);
					match s {
						[] => true,
						['е', 'ъ', 'д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10079);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10080);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17580);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		['м', s @ ..] => {
			super::push(out, 10081);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17581);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17582);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24779);
							match s {
								[] => true,
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
					super::push(out, 17583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17585);
					match s {
						[] => true,
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
			super::push(out, 10082);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 17586);
					match s {
						[] => true,
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
			super::push(out, 10083);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24780);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24781);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 24782);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24783);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31473);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10085);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17588);
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
		['ц', s @ ..] => {
			super::push(out, 10086);
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

/// Words ending -ькнуть.
pub(crate) fn n4057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3821);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10087);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17589);
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
			super::push(out, 10088);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17590);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24784);
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
				['у', s @ ..] => {
					super::push(out, 17591);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10089);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31474);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 31475);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24787);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 31477);
									match s {
										[] => true,
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
							super::push(out, 24788);
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
		['с', s @ ..] => {
			super::push(out, 10090);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экнуть.
pub(crate) fn n4058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3822);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10091);
			match s {
				[] => true,
				['м', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкнуть.
pub(crate) fn n4059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3823);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10092);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10093);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 17593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'ю', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17594);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17595);
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
		['т', s @ ..] => {
			super::push(out, 10094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17596);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24789);
							match s {
								[] => true,
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -якнуть.
pub(crate) fn n4060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3824);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10095);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10096);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17597);
					match s {
						[] => true,
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
			super::push(out, 10097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17598);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17599);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24790);
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
		['л', s @ ..] => {
			super::push(out, 10099);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17600);
					match s {
						[] => true,
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
			super::push(out, 10100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17601);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 17602);
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 17603);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17604);
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
					super::push(out, 17605);
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
					super::push(out, 17606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24792);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31478);
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
				['т', s @ ..] => {
					super::push(out, 17607);
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
				['ш', s @ ..] => {
					super::push(out, 17608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10101);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24794);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31479);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24796);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24797);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31480);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24798);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 10102);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17611);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17612);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24799);
							match s {
								[] => true,
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
					super::push(out, 17613);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24800);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31481);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31482);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36489);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10103);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 17614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word поокрепнуть.
pub(crate) fn n76933(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ипнуть.
pub(crate) fn n4071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3833);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 10123);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10124);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17632);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17633);
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
					super::push(out, 17634);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17635);
					match s {
						[] => true,
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
				['х', s @ ..] => {
					super::push(out, 17636);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24813);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31484);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36491);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10125);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17637);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24815);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24816);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17639);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 10127);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17641);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24817);
							match s {
								[] => true,
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
					super::push(out, 17642);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17643);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17644);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17645);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24818);
							match s {
								[] => true,
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
					super::push(out, 17646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 17647);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опнуть.
pub(crate) fn n4073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3835);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10129);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10132);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17649);
					s.is_empty()
				}
				['ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 17650);
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
						['ы', s @ ..] => {
							super::push(out, 24822);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31485);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10133);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17652);
					match s {
						[] => true,
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
			super::push(out, 10134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17653);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17654);
					match s {
						[] => true,
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
			super::push(out, 10135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17655);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17656);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24823);
							match s {
								[] => true,
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
					super::push(out, 17657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 10136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпнуть.
pub(crate) fn n4074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3836);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10137);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24825);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24826);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 17659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24828);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24829);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 24830);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 31486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24831);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 24832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24833);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31487);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упнуть.
pub(crate) fn n4076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3838);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10139);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24834);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24835);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31488);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31489);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 31490);
									match s {
										[] => true,
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
							super::push(out, 24836);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31491);
									match s {
										[] => true,
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
							super::push(out, 24837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24838);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31492);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10140);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 17660);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юпнуть.
pub(crate) fn n4078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3840);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10143);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 17661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япнуть.
pub(crate) fn n4079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3841);
	match s {
		[] => true,
		['р', 'х', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
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

/// Words ending -тернуть.
pub(crate) fn n12281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10151);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17673);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирнуть.
pub(crate) fn n4082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3844);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 10152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10153);
			match s {
				[] => true,
				['н', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10154);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орнуть.
pub(crate) fn n4083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3845);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10155);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 10156);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10157);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17674);
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
		['к', s @ ..] => {
			super::push(out, 10158);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17675);
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
		['п', s @ ..] => {
			super::push(out, 10159);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17676);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24846);
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
		['т', s @ ..] => {
			super::push(out, 10160);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17677);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урнуть.
pub(crate) fn n4084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3846);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 10161);
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
		['т', s @ ..] => {
			super::push(out, 10162);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырнуть.
pub(crate) fn n4085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3847);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10163);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17678);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31494);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31495);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36492);
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
									super::push(out, 31496);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36493);
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
									super::push(out, 31497);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31498);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36494);
											match s {
												[] => true,
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
									super::push(out, 31499);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 31500);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36495);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17679);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24847);
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
									super::push(out, 31501);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36496);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24848);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24849);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 24850);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24851);
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
									super::push(out, 31502);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31503);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31504);
									match s {
										[] => true,
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31505);
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
							super::push(out, 24852);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31506);
									match s {
										[] => true,
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
							super::push(out, 24853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24854);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31507);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10164);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17681);
					match s {
						[] => true,
						['к', s @ ..] => {
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
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10165);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t20::n12297(s, out),
		['п', '\u{1}', s @ ..] => {
			super::push(out, 10167);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10168);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аплеснуть.
pub(crate) fn n39572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24866);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 31513);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word переплеснуть.
pub(crate) fn n39575(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word приплеснуть.
pub(crate) fn n39576(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оплеснуть.
pub(crate) fn n39577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24867);
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

/// Words ending -сплеснуть.
pub(crate) fn n39578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24868);
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

/// The word отплеснуть.
pub(crate) fn n39579(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыплеснуть.
pub(crate) fn n39580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24869);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлеснуть.
pub(crate) fn n24810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17694);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24870);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31516);
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
			super::push(out, 24871);
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 24872);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реснуть.
pub(crate) fn n12306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10175);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 17695);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17696);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24874);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24875);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24876);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24877);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31518);
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

/// Words ending -иснуть.
pub(crate) fn n4089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3851);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17697);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17698);
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
				['б', s @ ..] => {
					super::push(out, 17699);
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17700);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24878);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31519);
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
					super::push(out, 17701);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24879);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31520);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17702);
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
					super::push(out, 17703);
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
		['к', s @ ..] => {
			super::push(out, 10177);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17704);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17705);
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
				['е', s @ ..] => {
					super::push(out, 17706);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24880);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31521);
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
					super::push(out, 17707);
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
					super::push(out, 17708);
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
				['ы', s @ ..] => {
					super::push(out, 17709);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10178);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17710);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10179);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10180);
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
					super::push(out, 17712);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кснуть.
pub(crate) fn n4090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3852);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10181);
			match s {
				[] => true,
				['л', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оснуть.
pub(crate) fn n4091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3853);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10182);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24884);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
								['а', s @ ..] => {
									super::push(out, 31522);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36497);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 36498);
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
									super::push(out, 31523);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36499);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36500);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 36501);
											match s {
												[] => true,
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
									super::push(out, 31524);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36502);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
		['с', s @ ..] => {
			super::push(out, 10183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рснуть.
pub(crate) fn n4092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3854);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10184);
			match s {
				[] => true,
				['к', 'и', 'н', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10185);
			match s {
				[] => true,
				['ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10186);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уснуть.
pub(crate) fn n4093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3855);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыснуть.
pub(crate) fn n4094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3856);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10187);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17714);
					match s {
						[] => true,
						['л', 'х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10188);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17715);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10189);
			match s {
				[] => true,
				['п', s @ ..] => {
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
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24886);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31525);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юснуть.
pub(crate) fn n4095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3857);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10190);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24887);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -яснуть.
pub(crate) fn n4096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3858);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10191);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10192);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24888);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31526);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отнуть.
pub(crate) fn n4101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3863);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10201);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17730);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17731);
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
		['г', 'о', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'е', 'щ', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10202);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24902);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24903);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24904);
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
		['м', s @ ..] => {
			super::push(out, 10203);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17732);
					s.is_empty()
				}
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'о', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 10204);
			match s {
				[] => true,
				['о', 'х', s @ ..] => {
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
		['ш', s @ ..] => {
			super::push(out, 10205);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17733);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24905);
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

/// Words ending -стнуть.
pub(crate) fn n4102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3864);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10206);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17734);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 31530);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36503);
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
			super::push(out, 10207);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17735);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24906);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 31531);
									match s {
										[] => true,
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31532);
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
									super::push(out, 31533);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 31534);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 31535);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36504);
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36505);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 36506);
											match s {
												[] => true,
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
									super::push(out, 31536);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 36507);
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
									super::push(out, 31537);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 31538);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36508);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10208);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17736);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 31539);
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
				['л', s @ ..] => {
					super::push(out, 17737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24908);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31540);
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
							super::push(out, 24909);
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
					super::push(out, 17738);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24910);
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
		['й', s @ ..] => {
			super::push(out, 10209);
			match s {
				[] => true,
				['э', 'п', 'и', 'п', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 17739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10210);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17740);
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
		['у', s @ ..] => {
			super::push(out, 10211);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17741);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24911);
							match s {
								[] => true,
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 24912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10212);
			match s {
				[] => true,
				['б', s @ ..] => super::t20::n24893(s, out),
				['л', s @ ..] => super::t20::n24894(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t20::n12355(s, out),
		_ => false,
	}
}

/// Words ending -ахтнуть.
pub(crate) fn n12359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10217);
	match s {
		[] => true,
		['д', 'у', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьтнуть.
pub(crate) fn n4105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3867);
	match s {
		[] => true,
		['л', 'у', 'с', 'н', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чахнуть.
pub(crate) fn n12383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10237);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17770);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17771);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 24929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17772);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17773);
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
			super::push(out, 17774);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					super::push(out, 24930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ехнуть.
pub(crate) fn n4113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3874);
	match s {
		[] => true,
		['л', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10238);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -ихнуть.
pub(crate) fn n4114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3875);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10239);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17775);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24931);
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
					super::push(out, 17776);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24932);
							match s {
								[] => true,
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
					super::push(out, 17777);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17778);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17779);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17780);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24934);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17781);
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
				['у', s @ ..] => {
					super::push(out, 17782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 17783);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17784);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17785);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31546);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31547);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 31548);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31549);
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
					super::push(out, 17786);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17787);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31550);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31551);
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
				['с', s @ ..] => {
					super::push(out, 17788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24937);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24938);
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
					super::push(out, 17789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24939);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24940);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
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
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 24941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 10242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17790);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 17791);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -охнуть.
pub(crate) fn n4115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3876);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10243);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10244);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17792);
					match s {
						[] => true,
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
			super::push(out, 10245);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17793);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 17795);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24944);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['и', s @ ..] => {
					super::push(out, 17796);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17797);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24946);
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
					super::push(out, 17798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17799);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24947);
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
				['ы', s @ ..] => {
					super::push(out, 17800);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24948);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10246);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24949);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24950);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36511);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36512);
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
							super::push(out, 24951);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24952);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24953);
							match s {
								[] => true,
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
					super::push(out, 17802);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 24954);
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
				['о', s @ ..] => {
					super::push(out, 17803);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31552);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36513);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10247);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17804);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17805);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24956);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31553);
									s.is_empty()
								}
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
			super::push(out, 10248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17806);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17807);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17808);
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
						['р', 'п', s @ ..] => {
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
				['с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => super::t20::n24990(s, out),
				['у', s @ ..] => super::t20::n24991(s, out),
				['ы', s @ ..] => super::t20::n24992(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пхнуть.
pub(crate) fn n4116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3877);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10249);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17811);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рхнуть.
pub(crate) fn n4117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3878);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10250);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24958);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31554);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10251);
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
						['ы', s @ ..] => {
							super::push(out, 24959);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31555);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10252);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 17812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ухнуть.
pub(crate) fn n4118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3879);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10253);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 10254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17813);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17814);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24960);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24961);
							match s {
								[] => true,
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
					super::push(out, 17815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17816);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24962);
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
					super::push(out, 17817);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24963);
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
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17818);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24964);
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
					super::push(out, 17819);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24965);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10255);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17820);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17821);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24966);
							match s {
								[] => true,
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10256);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17822);
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
				['и', s @ ..] => {
					super::push(out, 17823);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
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
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17824);
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
			super::push(out, 10257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17825);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17826);
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
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 31556);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36514);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10258);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17827);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17828);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24967);
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
				['д', s @ ..] => {
					super::push(out, 17829);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17830);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17831);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24968);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31557);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31558);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 36515);
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
						['р', s @ ..] => {
							super::push(out, 24969);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31559);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 10259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыхнуть.
pub(crate) fn n4120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3881);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10260);
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
		['л', s @ ..] => {
			super::push(out, 10261);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17834);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24970);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 36516);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 36517);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 24971);
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
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10262);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 10263);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17835);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17836);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24972);
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
					super::push(out, 17837);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24973);
							match s {
								[] => true,
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
					super::push(out, 17838);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24974);
							match s {
								[] => true,
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
					super::push(out, 17839);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24975);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10264);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24976);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24977);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24978);
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
			super::push(out, 10265);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17841);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24979);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31560);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 31561);
									match s {
										[] => true,
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
							super::push(out, 24980);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31562);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36518);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эхнуть.
pub(crate) fn n4121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3882);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юхнуть.
pub(crate) fn n4122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3883);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10266);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17842);
					match s {
						[] => true,
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
			super::push(out, 10267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яхнуть.
pub(crate) fn n4123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3884);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17843);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24981);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24982);
							match s {
								[] => true,
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
					super::push(out, 17844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24984);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24985);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31563);
									match s {
										[] => true,
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
							super::push(out, 24986);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 24987);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24988);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31564);
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
							super::push(out, 24989);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31565);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36519);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 24991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31566);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 31567);
									match s {
										[] => true,
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
							super::push(out, 24992);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24993);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31568);
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
		_ => false,
	}
}

/// The word подкашлянуть.
pub(crate) fn n76955(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выкашлянуть.
pub(crate) fn n76956(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рянуть.
pub(crate) fn n4139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3900);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 10289);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17879);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10290);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17880);
					match s {
						[] => true,
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
			super::push(out, 10291);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17881);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25015);
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
					super::push(out, 17882);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25016);
							match s {
								[] => true,
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
					super::push(out, 17883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 25017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25018);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31582);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 17884);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25019);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10292);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17885);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тянуть.
pub(crate) fn n4140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3901);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10293);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10294);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25021);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31583);
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
							super::push(out, 25022);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31584);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						['д', s @ ..] => {
							super::push(out, 25023);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31585);
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
			super::push(out, 10295);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10296);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10297);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10298);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10299);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10300);
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
			super::push(out, 10301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17886);
					match s {
						[] => true,
						['р', s @ ..] => {
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
					super::push(out, 17887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10302);
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
			super::push(out, 10303);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17888);
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

/// Words ending -заправлять.
pub(crate) fn n54281(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word направлять.
pub(crate) fn n66857(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перенаправлять.
pub(crate) fn n66858(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word вправлять.
pub(crate) fn n39944(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оправлять.
pub(crate) fn n39945(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -справлять.
pub(crate) fn n39946(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -отправлять.
pub(crate) fn n39947(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -управлять.
pub(crate) fn n39948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25032);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31589);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31590);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -травлять.
pub(crate) fn n25259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17922);
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
		_ => false,
	}
}

/// Words ending -тавлять.
pub(crate) fn n12676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10372);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25033);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31591);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 31592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31593);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 36523);
									match s {
										[] => true,
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
							super::push(out, 31594);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 36524);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 39792);
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
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 31595);
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
							super::push(out, 31596);
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
							super::push(out, 31597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36525);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39793);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', 'и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -евлять.
pub(crate) fn n4325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4022);
	match s {
		[] => true,
		['н', 'г', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25035);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31598);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 36526);
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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

/// Words ending -звлять.
pub(crate) fn n4326(s: &[char], _out: &mut Path<'_>) -> bool {
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
		['я', 'ъ', 'з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ивлять.
pub(crate) fn n4327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4023);
	match s {
		[] => true,
		['ж', s @ ..] => {
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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
		['р', 'к', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 17923);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25036);
							match s {
								[] => true,
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

/// Words ending -молвлять.
pub(crate) fn n4328(s: &[char], _out: &mut Path<'_>) -> bool {
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
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
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

/// Words ending -овлять.
pub(crate) fn n4329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4024);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 10373);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25037);
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
		['д', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10374);
			match s {
				[] => true,
				['р', 'е', 'ц', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17925);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25038);
							match s {
								[] => true,
								['я', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10375);
			match s {
				[] => true,
				['а', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25039);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31599);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
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
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', 'о', 'д', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10376);
			match s {
				[] => true,
				['о', 'н', s @ ..] => {
					super::push(out, 17926);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25040);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10377);
			match s {
				[] => true,
				['о', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25041);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31601);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 31602);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36527);
											s.is_empty()
										}
										_ => false,
									}
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

/// Words ending -твлять.
pub(crate) fn n4330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4025);
	match s {
		[] => true,
		['е', 'в', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17927);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25042);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 31603);
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
				['е', s @ ..] => {
					super::push(out, 17928);
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
						['д', 'ж', 'о', 'т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['а', 'д', 'у', 'с', 'о', 'г', s @ ..] => {
							match s {
								[] => true,
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
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

/// Words ending -являть.
pub(crate) fn n4331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4026);
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
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['з', s @ ..] => {
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дострелять.
pub(crate) fn n54337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31608);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пострелять.
pub(crate) fn n54338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31609);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word прострелять.
pub(crate) fn n54339(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -сстрелять.
pub(crate) fn n40003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25048);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36528);
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
			super::push(out, 31610);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тстрелять.
pub(crate) fn n40004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25049);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстрелять.
pub(crate) fn n40005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25050);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -селять.
pub(crate) fn n4341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4033);
	match s {
		[] => true,
		['а', 'н', s @ ..] => {
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
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10381);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25051);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31613);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36529);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
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

/// Words ending -целять.
pub(crate) fn n4342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4034);
	s.is_empty()
}

/// The word расщелять.
pub(crate) fn n4343(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word позалеплять.
pub(crate) fn n76967(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поналеплять.
pub(crate) fn n76968(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word повылеплять.
pub(crate) fn n76969(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -цеплять.
pub(crate) fn n12770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10401);
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

/// Words ending -сщеплять.
pub(crate) fn n25375(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word отщеплять.
pub(crate) fn n25376(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выщеплять.
pub(crate) fn n25377(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оплять.
pub(crate) fn n4368(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
		['р', 'к', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -уплять.
pub(crate) fn n4369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4046);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'о', 'в', 'о', 'с', s @ ..] => {
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
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', 'о', '\u{1}', s @ ..] => {
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
		['т', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
						['п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -поразмышлять.
pub(crate) fn n76970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39794);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -напромышлять.
pub(crate) fn n76971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39795);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -попромышлять.
pub(crate) fn n76972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39796);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вымышлять.
pub(crate) fn n40097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25075);
	s.is_empty()
}

/// The word поупражнять.
pub(crate) fn n76973(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word просочинять.
pub(crate) fn n66913(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отчинять.
pub(crate) fn n12913(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поразгонять.
pub(crate) fn n76974(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -аронять.
pub(crate) fn n12942(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 18023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азронять.
pub(crate) fn n25602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18024);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оронять.
pub(crate) fn n12944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10487);
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 18025);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'с', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['х', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пронять.
pub(crate) fn n12945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10488);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тонять.
pub(crate) fn n4458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4110);
	match s {
		[] => true,
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

/// Words ending -пошвырять.
pub(crate) fn n40267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25124);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сшвырять.
pub(crate) fn n25714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18072);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25125);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тшвырять.
pub(crate) fn n25715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18073);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышвырять.
pub(crate) fn n25716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18074);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25127);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зырять.
pub(crate) fn n4519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4146);
	match s {
		[] => true,
		['о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18075);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18076);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25128);
							match s {
								[] => true,
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
					super::push(out, 18077);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25129);
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
					super::push(out, 18078);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25130);
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
					super::push(out, 18079);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нырять.
pub(crate) fn n4520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4147);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10545);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10546);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18080);
					match s {
						[] => true,
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
			super::push(out, 10547);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18081);
					match s {
						[] => true,
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
			super::push(out, 10548);
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
		['ш', s @ ..] => {
			super::push(out, 10549);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18082);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18083);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25132);
							match s {
								[] => true,
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
					super::push(out, 18084);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25133);
							match s {
								[] => true,
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
					super::push(out, 18085);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25134);
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
					super::push(out, 18086);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25135);
							match s {
								[] => true,
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
					super::push(out, 18087);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25136);
							match s {
								[] => true,
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
					super::push(out, 18088);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пырять.
pub(crate) fn n4521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4148);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10550);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10551);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ебаться.
pub(crate) fn n13186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10589);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18103);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18104);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25140);
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
					super::push(out, 25141);
					match s {
						[] => true,
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
			super::push(out, 18105);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25142);
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
			super::push(out, 18106);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25143);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31650);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36539);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36540);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36541);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 36542);
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
				['х', s @ ..] => {
					super::push(out, 25144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31651);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 18107);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
			super::push(out, 18108);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25146);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['к', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
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
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18109);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31652);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31653);
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
			super::push(out, 18110);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 18111);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 18112);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25148);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шибаться.
pub(crate) fn n25803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18114);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
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
				['о', s @ ..] => {
					super::push(out, 31655);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36543);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25149);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36544);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39797);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лбаться.
pub(crate) fn n13188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10591);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18115);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31656);
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

/// Words ending -убаться.
pub(crate) fn n13189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10592);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 18116);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25150);
					match s {
						[] => true,
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
			super::push(out, 18117);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25151);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 31657);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
					super::push(out, 25152);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31658);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36545);
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

/// Words ending -ыбаться.
pub(crate) fn n13190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10593);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 18118);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18119);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31659);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31660);
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
						['о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ябаться.
pub(crate) fn n13191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10594);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18120);
			match s {
				[] => true,
				['х', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18121);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25154);
					match s {
						[] => true,
						['к', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31661);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31662);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ваться.
pub(crate) fn n4656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4216);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10595);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25155);
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
						['з', s @ ..] => {
							super::push(out, 25156);
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
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', 'о', 'в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 25157);
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
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
										['о', s @ ..] => {
											super::push(out, 36546);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 39798);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n83348(s, out),
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
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25158);
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => super::t20::n40361(s, out),
						['у', '\u{1}', s @ ..] => super::t20::n40362(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t20::n40363(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t20::n25812(s, out),
				['н', s @ ..] => super::t20::n25813(s, out),
				['т', s @ ..] => super::t20::n25814(s, out),
				['х', s @ ..] => super::t20::n25815(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t20::n13193(s, out),
		['з', s @ ..] => super::t20::n13194(s, out),
		['и', s @ ..] => super::t20::n13195(s, out),
		['о', s @ ..] => super::t20::n13196(s, out),
		['р', s @ ..] => super::t20::n13197(s, out),
		['у', s @ ..] => super::t20::n13198(s, out),
		['ы', s @ ..] => super::t20::n13199(s, out),
		_ => false,
	}
}

/// Words ending -гаться.
pub(crate) fn n4657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4217);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10603);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18199);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
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
						['з', 'и', s @ ..] => {
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
						['о', s @ ..] => {
							match s {
								[] => true,
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
											match s {
												[] => true,
												['в', 'и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['к', 'у', 'р', '\u{1}', s @ ..] => {
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
												['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', 'в', 'и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 18200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25460);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25461);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25462);
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
		['е', s @ ..] => {
			super::push(out, 10604);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25463);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25464);
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
							super::push(out, 25465);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25466);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32086);
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
							super::push(out, 25467);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									super::push(out, 32087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32088);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 32089);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 32090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 25468);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25469);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25471);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18202);
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
				['р', s @ ..] => super::t20::n25897(s, out),
				['т', s @ ..] => super::t20::n25898(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t20::n13202(s, out),
		['и', s @ ..] => super::t20::n13203(s, out),
		['л', s @ ..] => super::t20::n13204(s, out),
		['о', s @ ..] => super::t20::n13205(s, out),
		['р', s @ ..] => super::t20::n13206(s, out),
		['у', s @ ..] => super::t20::n13207(s, out),
		['ы', s @ ..] => super::t20::n13208(s, out),
		['ю', s @ ..] => super::t20::n13209(s, out),
		['я', s @ ..] => super::t20::n13210(s, out),
		_ => false,
	}
}

/// Words ending -даться.
pub(crate) fn n4658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4218);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10614);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10615);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25507);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', s @ ..] => {
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18227);
					match s {
						[] => true,
						['в', 'о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18228);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25509);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32123);
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
				['р', s @ ..] => {
					super::push(out, 18229);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32124);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 37027);
											match s {
												[] => true,
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
								['ы', s @ ..] => {
									super::push(out, 32125);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37028);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10616);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18230);
					match s {
						[] => true,
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
			super::push(out, 10617);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18231);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18232);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25510);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 10618);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18233);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32126);
									s.is_empty()
								}
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
			super::push(out, 10619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18234);
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
								['о', s @ ..] => {
									super::push(out, 32127);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 37029);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t20::n25932(s, out),
				['е', s @ ..] => super::t20::n25933(s, out),
				['и', s @ ..] => {
					super::push(out, 18237);
					s.is_empty()
				}
				['н', s @ ..] => super::t20::n25935(s, out),
				['о', s @ ..] => super::t20::n25936(s, out),
				['р', s @ ..] => super::t20::n25937(s, out),
				['с', s @ ..] => super::t20::n25938(s, out),
				['у', s @ ..] => super::t20::n25939(s, out),
				['ъ', s @ ..] => super::t20::n25940(s, out),
				['ы', s @ ..] => super::t20::n25941(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t20::n13217(s, out),
		['з', s @ ..] => super::t20::n13218(s, out),
		['и', s @ ..] => super::t20::n13219(s, out),
		['н', s @ ..] => super::t20::n13220(s, out),
		['о', s @ ..] => super::t20::n13221(s, out),
		['с', s @ ..] => super::t20::n13222(s, out),
		['т', s @ ..] => super::t20::n13223(s, out),
		['у', s @ ..] => super::t20::n13224(s, out),
		['ы', s @ ..] => super::t20::n13225(s, out),
		['ю', s @ ..] => super::t20::n13226(s, out),
		['я', s @ ..] => super::t20::n13227(s, out),
		_ => false,
	}
}

/// Words ending -жаться.
pub(crate) fn n4659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4219);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10631);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10632);
			match s {
				[] => true,
				['в', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32157);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37053);
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
						['в', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25563);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25564);
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
					super::push(out, 18277);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'ы', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25565);
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', s @ ..] => {
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
				['с', s @ ..] => {
					super::push(out, 18278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25566);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25567);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 25568);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25569);
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
		['б', s @ ..] => {
			super::push(out, 10633);
			match s {
				[] => true,
				['а', 'н', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25570);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 25571);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10635);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18280);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10636);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25573);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25574);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 25575);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 37054);
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
							super::push(out, 25576);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25577);
							match s {
								[] => true,
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
					super::push(out, 18282);
					match s {
						[] => true,
						['с', s @ ..] => super::t20::n41120(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t20::n25985(s, out),
				['м', s @ ..] => super::t20::n25986(s, out),
				['н', s @ ..] => super::t20::n25987(s, out),
				['р', s @ ..] => super::t20::n25988(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t20::n13234(s, out),
		['з', s @ ..] => super::t20::n13235(s, out),
		['и', s @ ..] => super::t20::n13236(s, out),
		['л', s @ ..] => super::t20::n13237(s, out),
		['о', s @ ..] => super::t20::n13238(s, out),
		['р', s @ ..] => super::t20::n13239(s, out),
		['с', s @ ..] => super::t20::n13240(s, out),
		['т', s @ ..] => super::t20::n13241(s, out),
		['у', s @ ..] => super::t20::n13242(s, out),
		['ы', s @ ..] => super::t20::n13243(s, out),
		['я', s @ ..] => super::t20::n13244(s, out),
		_ => false,
	}
}

/// Words ending -заться.
pub(crate) fn n4660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4220);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10648);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25615);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25616);
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
						['о', s @ ..] => {
							super::push(out, 25617);
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
							super::push(out, 25618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 32180);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 37064);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 37065);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 40323);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t20::n85143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
								['ы', s @ ..] => {
									super::push(out, 32181);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37066);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 25619);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 25621);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32182);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25622);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32183);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25623);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25624);
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
							super::push(out, 25625);
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
							super::push(out, 25626);
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
										['о', 'р', 'и', 'м', '\u{1}', s @ ..] => {
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 25627);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32184);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10649);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18318);
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
		['е', s @ ..] => super::t20::n13247(s, out),
		['и', s @ ..] => super::t20::n13248(s, out),
		['л', s @ ..] => {
			super::push(out, 10652);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18325);
					match s {
						[] => true,
						['п', s @ ..] => super::t20::n41255(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t20::n13250(s, out),
		['р', s @ ..] => super::t20::n13251(s, out),
		['ы', s @ ..] => super::t20::n13252(s, out),
		['ь', s @ ..] => super::t20::n13253(s, out),
		['ю', s @ ..] => super::t20::n13254(s, out),
		['я', s @ ..] => super::t20::n13255(s, out),
		_ => false,
	}
}

/// Words ending -каться.
pub(crate) fn n4661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4221);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10659);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18337);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 18338);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25665);
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
				['д', s @ ..] => {
					super::push(out, 18339);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 25666);
							match s {
								[] => true,
								['а', 'х', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25667);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 25668);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32212);
									match s {
										[] => true,
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
							super::push(out, 25669);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32213);
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
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 37074);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 37075);
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
									super::push(out, 32214);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 37076);
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
				['л', s @ ..] => {
					super::push(out, 18341);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25670);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32215);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37077);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 37078);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 40325);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t20::n85145(s, out),
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
									super::push(out, 32216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 25671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32217);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32218);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 37079);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 37080);
											match s {
												[] => true,
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
									super::push(out, 32219);
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
									super::push(out, 32220);
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
								['т', s @ ..] => {
									super::push(out, 32221);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 37081);
											match s {
												[] => true,
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
									super::push(out, 32222);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37082);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25672);
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
					super::push(out, 18343);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25673);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32223);
									match s {
										[] => true,
										['а', 'з', 'а', 'н', '\u{1}', s @ ..] => super::t20::n69813(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t20::n26054(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t20::n13257(s, out),
		['в', s @ ..] => super::t20::n13258(s, out),
		['е', s @ ..] => super::t20::n13259(s, out),
		['и', s @ ..] => super::t20::n13260(s, out),
		['л', s @ ..] => super::t20::n13261(s, out),
		['м', s @ ..] => super::t20::n13262(s, out),
		['н', s @ ..] => super::t20::n13263(s, out),
		['о', s @ ..] => super::t20::n13264(s, out),
		['п', s @ ..] => super::t20::n13265(s, out),
		['р', s @ ..] => super::t20::n13266(s, out),
		['с', s @ ..] => super::t20::n13267(s, out),
		['т', s @ ..] => super::t20::n13268(s, out),
		['у', s @ ..] => super::t21::n13269(s, out),
		['ц', s @ ..] => super::t21::n13270(s, out),
		['ч', s @ ..] => super::t21::n13271(s, out),
		['ш', s @ ..] => super::t21::n13272(s, out),
		['ы', s @ ..] => super::t21::n13273(s, out),
		['ь', s @ ..] => super::t21::n13274(s, out),
		['ю', s @ ..] => super::t21::n13275(s, out),
		['я', s @ ..] => super::t21::n13276(s, out),
		_ => false,
	}
}

