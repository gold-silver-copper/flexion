use flexion::Path;

/// Words ending -одничать.
pub(crate) fn n18561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13864);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22267);
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['т', 'о', 'к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['х', 'е', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39656);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 41593);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36097);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 39657);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22268);
			match s {
				[] => true,
				['о', 'в', 'о', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 30380);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 30381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22269);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 30382);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36099);
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
			super::push(out, 22270);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30383);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 30384);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 36100);
							match s {
								[] => true,
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
					super::push(out, 30385);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36101);
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
		['р', s @ ..] => {
			super::push(out, 22271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30386);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 36102);
							match s {
								[] => true,
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
					super::push(out, 30387);
					match s {
						[] => true,
						['с', 'а', 'м', 'у', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39658);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 41594);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30388);
					match s {
						[] => true,
						['г', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30389);
					match s {
						[] => true,
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
			super::push(out, 22272);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30390);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36103);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удничать.
pub(crate) fn n18563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13866);
	match s {
		[] => true,
		['к', 'с', 'а', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 22273);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30391);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30392);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 22274);
			match s {
				[] => true,
				['т', 'о', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36105);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39659);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядничать.
pub(crate) fn n18566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13869);
	match s {
		[] => true,
		['е', 'н', 'у', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'г', 'о', 'х', 'р', 'е', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word повыпендрежничать.
pub(crate) fn n86768(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лужничать.
pub(crate) fn n33881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22288);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 30404);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ыжничать.
pub(crate) fn n18573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13876);
	match s {
		[] => true,
		['л', 'а', 'в', 'к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', 'е', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'а', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'а', 'х', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'а', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яжничать.
pub(crate) fn n18574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13877);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 22290);
			match s {
				[] => true,
				['о', 'р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36115);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36116);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39663);
									match s {
										[] => true,
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
							super::push(out, 36117);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 39665);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41597);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'и', 'т', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 22291);
			match s {
				[] => true,
				['р', 'о', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'р', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22292);
			match s {
				[] => true,
				['к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22293);
			match s {
				[] => true,
				['у', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39666);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 39667);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41598);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 36118);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 39668);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'л', 'о', 'х', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word пролюбезничать.
pub(crate) fn n86771(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ерзничать.
pub(crate) fn n33906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22303);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30413);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39671);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39672);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 30414);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узничать.
pub(crate) fn n18580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13883);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 22304);
			match s {
				[] => true,
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36131);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36132);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39674);
									match s {
										[] => true,
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
							super::push(out, 36133);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39675);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзничать.
pub(crate) fn n18581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13884);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 22305);
			match s {
				[] => true,
				['а', 'х', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22306);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 30415);
					match s {
						[] => true,
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
			super::push(out, 22307);
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

/// The word развязничать.
pub(crate) fn n33911(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ентяйничать.
pub(crate) fn n65582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36141);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41601);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41602);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42641);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42642);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43246);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -люнтяйничать.
pub(crate) fn n76560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39682);
	match s {
		[] => true,
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word заскромничать.
pub(crate) fn n86774(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поскромничать.
pub(crate) fn n86775(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word постранничать.
pub(crate) fn n86776(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -инничать.
pub(crate) fn n18595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13896);
	match s {
		[] => true,
		['с', 'ь', 'л', 'е', 'п', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', 'к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ч', 'с', 'е', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', 'р', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['щ', 'р', 'е', 'т', 'а', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онничать.
pub(crate) fn n18596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13897);
	match s {
		[] => true,
		['в', 'з', 'о', 'т', 'с', 'у', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'а', 'л', 'е', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'е', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 22334);
			match s {
				[] => true,
				['а', 'з', 'з', 'е', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36158);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39695);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22335);
			match s {
				[] => true,
				['к', 'о', 'п', 'о', s @ ..] => {
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
		['м', s @ ..] => {
			super::push(out, 22336);
			match s {
				[] => true,
				['е', 'р', 'е', 'ц', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'л', '\u{1}', s @ ..] => {
					super::push(out, 30434);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унничать.
pub(crate) fn n18597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13898);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22337);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30435);
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

/// Words ending -ынничать.
pub(crate) fn n18598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13899);
	match s {
		[] => true,
		['т', s @ ..] => {
			match s {
				[] => true,
				['л', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'у', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янничать.
pub(crate) fn n18599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13900);
	match s {
		[] => true,
		['ь', s @ ..] => {
			match s {
				[] => true,
				['з', 'е', 'б', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 36159);
							match s {
								[] => true,
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
					super::push(out, 30436);
					match s {
						[] => true,
						['у', 'м', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терничать.
pub(crate) fn n33991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22351);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 30448);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36167);
					match s {
						[] => true,
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
			super::push(out, 30449);
			match s {
				[] => true,
				['з', 'у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -съерничать.
pub(crate) fn n51077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30450);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирничать.
pub(crate) fn n18602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13903);
	match s {
		[] => true,
		['ж', 'н', 'а', 'р', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', 'о', 'б', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орничать.
pub(crate) fn n18603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13904);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22354);
			match s {
				[] => true,
				['о', 'х', 'о', 'р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 22355);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30452);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36168);
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
			super::push(out, 22356);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30453);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36169);
							match s {
								[] => true,
								['п', 'с', 'е', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36170);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36171);
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
		['ш', s @ ..] => {
			super::push(out, 22357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урничать.
pub(crate) fn n18604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13905);
	match s {
		[] => true,
		['г', 'и', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', 'л', 'р', 'е', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ырничать.
pub(crate) fn n18605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13906);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30455);
					match s {
						[] => true,
						['а', 'к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30456);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39701);
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
		['т', 'с', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word авантюрничать.
pub(crate) fn n18606(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ярничать.
pub(crate) fn n18607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13907);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 22358);
			match s {
				[] => true,
				['а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 30457);
					match s {
						[] => true,
						['и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 39702);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						['к', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'п', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 22359);
			match s {
				[] => true,
				['л', 'и', 'м', 'а', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36173);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39703);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word пофокусничать.
pub(crate) fn n86777(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поделикатничать.
pub(crate) fn n86778(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ктничать.
pub(crate) fn n18616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13916);
	match s {
		[] => true,
		['е', 'ф', 'ф', 'э', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отничать.
pub(crate) fn n18617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13917);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22382);
			match s {
				[] => true,
				['б', 'у', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22383);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30470);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36192);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 22384);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36193);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39712);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'о', 'в', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 22385);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30471);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36194);
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
					super::push(out, 30472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртничать.
pub(crate) fn n18618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13918);
	match s {
		[] => true,
		['а', 'з', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стничать.
pub(crate) fn n18619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13919);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22386);
			match s {
				[] => true,
				['р', 'т', 'с', 'о', 'б', 'о', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22387);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30473);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36195);
							match s {
								[] => true,
								['с', 'с', 'е', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', 'о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22388);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30476);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', 'е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 22389);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30477);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 36196);
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
					super::push(out, 30478);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36197);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 41608);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 41609);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30479);
					match s {
						[] => true,
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
			super::push(out, 22390);
			match s {
				[] => true,
				['р', 'о', 'к', 'е', 'о', 'в', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утничать.
pub(crate) fn n18620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13920);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22391);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30480);
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
		['з', s @ ..] => {
			super::push(out, 22392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30481);
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
		['п', s @ ..] => {
			super::push(out, 22393);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 30482);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39713);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41610);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
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
		['т', s @ ..] => {
			super::push(out, 22394);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30483);
					match s {
						[] => true,
						['т', 'с', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытничать.
pub(crate) fn n18621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13921);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 22395);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36198);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 36199);
							match s {
								[] => true,
								['ю', 'л', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 41611);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42643);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n88614(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22396);
			match s {
				[] => true,
				['к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятничать.
pub(crate) fn n18622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13922);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22397);
			match s {
				[] => true,
				['е', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36200);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39714);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22398);
			match s {
				[] => true,
				['о', 'р', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word снасмешничать.
pub(crate) fn n86783(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word свеликодушничать.
pub(crate) fn n86784(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -смалодушничать.
pub(crate) fn n83258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41614);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -модушничать.
pub(crate) fn n65726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36213);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 39719);
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

/// Words ending -нодушничать.
pub(crate) fn n65727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36214);
	match s {
		[] => true,
		['в', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', 'к', 'е', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тодушничать.
pub(crate) fn n65728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36215);
	match s {
		[] => true,
		['с', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зушничать.
pub(crate) fn n34095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22417);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30495);
			match s {
				[] => true,
				['к', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кушничать.
pub(crate) fn n34096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22418);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 30496);
			match s {
				[] => true,
				['л', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30497);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36216);
					match s {
						[] => true,
						['т', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лушничать.
pub(crate) fn n34097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22419);
	match s {
		[] => true,
		['к', 'а', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мушничать.
pub(crate) fn n34098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22420);
	match s {
		[] => true,
		['о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рушничать.
pub(crate) fn n34099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22421);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 30498);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36217);
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
		['у', s @ ..] => {
			super::push(out, 30499);
			match s {
				[] => true,
				['в', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 39720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хушничать.
pub(crate) fn n34100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22422);
	match s {
		[] => true,
		['и', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ышничать.
pub(crate) fn n18637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13932);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 22423);
			match s {
				[] => true,
				['а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36218);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36219);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39721);
									match s {
										[] => true,
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
							super::push(out, 36220);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39722);
									match s {
										[] => true,
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
							super::push(out, 36221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 36222);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 39723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юшничать.
pub(crate) fn n18638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13933);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 22424);
			match s {
				[] => true,
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22425);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30500);
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

/// Words ending -циальничать.
pub(crate) fn n65745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36226);
	match s {
		[] => true,
		['и', 'ф', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кальничать.
pub(crate) fn n51189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30505);
	match s {
		[] => true,
		['и', 'д', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
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

/// Words ending -нальничать.
pub(crate) fn n51190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30506);
	match s {
		[] => true,
		['и', 'г', 'и', 'р', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39725);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ральничать.
pub(crate) fn n51191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30507);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36227);
			match s {
				[] => true,
				['б', 'и', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41615);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41616);
							match s {
								[] => true,
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
					super::push(out, 39727);
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
		['т', s @ ..] => {
			super::push(out, 36228);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39728);
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

/// The word сальничать.
pub(crate) fn n51192(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тальничать.
pub(crate) fn n51193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30508);
	match s {
		[] => true,
		['н', 'е', 'м', 'и', 'т', 'н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39729);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41617);
							match s {
								[] => true,
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
					super::push(out, 39730);
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

/// Words ending -уальничать.
pub(crate) fn n51194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30509);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 36229);
			match s {
				[] => true,
				['к', 'е', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хальничать.
pub(crate) fn n51195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30510);
	match s {
		[] => true,
		['а', 'н', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41618);
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
		_ => false,
	}
}

/// Words ending -ельничать.
pub(crate) fn n34107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22428);
	match s {
		[] => true,
		['б', 'а', 'р', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36230);
					match s {
						[] => true,
						['н', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', 'с', 'е', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 36231);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41619);
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
				['о', s @ ..] => {
					super::push(out, 36232);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39732);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 39733);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41620);
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
		['м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 30511);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 36233);
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

/// Words ending -ильничать.
pub(crate) fn n34108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22429);
	match s {
		[] => true,
		['в', 'р', 'е', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ж', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', 'а', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 30512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 39734);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41621);
									match s {
										[] => true,
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
							super::push(out, 39735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 30513);
			match s {
				[] => true,
				['е', 'п', 'е', 'щ', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', s @ ..] => {
					match s {
						[] => true,
						['ж', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ф', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -ольничать.
pub(crate) fn n34109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22430);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30514);
			match s {
				[] => true,
				['о', 'д', 'р', 'е', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 30515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 36234);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39736);
							match s {
								[] => true,
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
					super::push(out, 36235);
					match s {
						[] => true,
						['о', 'в', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 36236);
					match s {
						[] => true,
						['р', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36237);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39737);
							match s {
								[] => true,
								['а', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42644);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42645);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 42646);
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
							super::push(out, 39738);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 30516);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39739);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 41622);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30517);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36238);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41623);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42647);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36239);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39740);
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
				['ш', s @ ..] => {
					super::push(out, 36240);
					match s {
						[] => true,
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
			super::push(out, 30518);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36241);
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
			super::push(out, 30519);
			match s {
				[] => true,
				['о', 'б', 'е', 'л', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30520);
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

/// Words ending -ульничать.
pub(crate) fn n34110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22431);
	match s {
		[] => true,
		['ж', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 36242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 30521);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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

/// The word собутыльничать.
pub(crate) fn n34111(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word авосьничать.
pub(crate) fn n18642(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -межеваться.
pub(crate) fn n54579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31681);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36553);
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
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 36554);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 39802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 36555);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -режеваться.
pub(crate) fn n54580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31682);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36556);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39804);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жжеваться.
pub(crate) fn n40391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25171);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31683);
			match s {
				[] => true,
				['р', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зжеваться.
pub(crate) fn n40392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25172);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31684);
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

/// Words ending -мжеваться.
pub(crate) fn n40393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25173);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31685);
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

/// Words ending -нжеваться.
pub(crate) fn n40394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25174);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31686);
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

/// Words ending -ожеваться.
pub(crate) fn n40395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25175);
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

/// Words ending -ржеваться.
pub(crate) fn n40396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25176);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31687);
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

/// Words ending -сжеваться.
pub(crate) fn n40397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25177);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тжеваться.
pub(crate) fn n40398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25178);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужеваться.
pub(crate) fn n40399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25179);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 31689);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьжеваться.
pub(crate) fn n40400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25180);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 31690);
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

/// Words ending -зеваться.
pub(crate) fn n25820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18131);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25181);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25182);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31691);
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
				['р', s @ ..] => {
					super::push(out, 31692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36557);
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
		['з', s @ ..] => {
			super::push(out, 25183);
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
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -леваться.
pub(crate) fn n25821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18132);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25184);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36558);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36559);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 36560);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 36561);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 36562);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 39805);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 31695);
					match s {
						[] => true,
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
			super::push(out, 25185);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31696);
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
					super::push(out, 31697);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36563);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25186);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31698);
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
			super::push(out, 25187);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31699);
					match s {
						[] => true,
						['а', 'т', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 31700);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36564);
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
				['п', s @ ..] => {
					super::push(out, 31701);
					match s {
						[] => true,
						['м', 'е', 'т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39806);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 39807);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39808);
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
				['р', s @ ..] => {
					super::push(out, 31702);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 31703);
					match s {
						[] => true,
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
			super::push(out, 25188);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31704);
					match s {
						[] => true,
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
			super::push(out, 25189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31705);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 31706);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 36565);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 36566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 36567);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 39809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41643);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 41644);
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
				['б', s @ ..] => {
					super::push(out, 31707);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31708);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 36569);
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
					super::push(out, 31709);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 36570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39810);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 39811);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 41645);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31710);
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
					super::push(out, 31711);
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
				['ы', s @ ..] => {
					super::push(out, 31712);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t29::n40412(s, out),
		['п', s @ ..] => super::t29::n40413(s, out),
		['т', s @ ..] => super::t29::n40414(s, out),
		_ => false,
	}
}

/// Words ending -меваться.
pub(crate) fn n25822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18133);
	match s {
		[] => true,
		['т', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 25193);
			match s {
				[] => true,
				['з', 'а', 'р', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -неваться.
pub(crate) fn n25823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18134);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 25194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31720);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 31721);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31722);
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25195);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31723);
					match s {
						[] => true,
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
			super::push(out, 25196);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31724);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39813);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39814);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39815);
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
					super::push(out, 31725);
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

/// Words ending -оеваться.
pub(crate) fn n25824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18135);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31726);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 31727);
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
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 25198);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31728);
					match s {
						[] => true,
						['п', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -певаться.
pub(crate) fn n25825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18136);
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
		['р', 'е', 'т', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реваться.
pub(crate) fn n25826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18137);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 25199);
			match s {
				[] => true,
				['з', 'в', '\u{1}', s @ ..] => {
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
						['б', 'о', '\u{1}', s @ ..] => {
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['е', 'м', 'а', 'н', s @ ..] => {
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
		['з', s @ ..] => {
			super::push(out, 25200);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 25201);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36575);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 39816);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 39817);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25202);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31729);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36576);
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
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -севаться.
pub(crate) fn n25827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18138);
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
				_ => false,
			}
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
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

/// Words ending -теваться.
pub(crate) fn n25828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18139);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цеваться.
pub(crate) fn n25829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18140);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 25204);
			match s {
				[] => true,
				['у', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36577);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36578);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 36579);
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
		['и', s @ ..] => {
			super::push(out, 25205);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 36580);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 39818);
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
						['ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 25206);
			match s {
				[] => true,
				['а', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'л', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 25207);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31730);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36581);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 39819);
									match s {
										[] => true,
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39820);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39821);
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
								['с', s @ ..] => {
									super::push(out, 39822);
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
									super::push(out, 39823);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 41646);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31731);
					match s {
						[] => true,
						['в', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 31732);
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
		['п', s @ ..] => {
			super::push(out, 25208);
			match s {
				[] => true,
				['у', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25209);
			match s {
				[] => true,
				['а', 'в', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36582);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36583);
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
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36584);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36585);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36586);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25210);
			match s {
				[] => true,
				['а', 'в', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', 'л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 39824);
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
						['ф', s @ ..] => {
							super::push(out, 36587);
							match s {
								[] => true,
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
					super::push(out, 31733);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39825);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39826);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39827);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чеваться.
pub(crate) fn n25830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25211);
			match s {
				[] => true,
				['р', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 36588);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25212);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31734);
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
		['н', s @ ..] => {
			super::push(out, 25213);
			match s {
				[] => true,
				['а', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25214);
			match s {
				[] => true,
				['а', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31735);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 39828);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 41647);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25215);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31736);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39829);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39830);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39831);
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
			super::push(out, 25216);
			match s {
				[] => true,
				['л', 'и', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеваться.
pub(crate) fn n25831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18142);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25217);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31737);
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
			super::push(out, 25218);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31738);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36589);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 36590);
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

/// Words ending -щеваться.
pub(crate) fn n25832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18143);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25219);
			match s {
				[] => true,
				['в', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паиваться.
pub(crate) fn n40483(s: &[char], _out: &mut Path<'_>) -> bool {
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раиваться.
pub(crate) fn n40484(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
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
				['о', 'д', '\u{1}', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
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
								['р', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
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
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'у', 'р', 'т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таиваться.
pub(crate) fn n40485(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
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
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
								['у', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -биваться.
pub(crate) fn n25840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18151);
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
						['о', s @ ..] => {
							super::push(out, 36592);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39832);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 31742);
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
			super::push(out, 25228);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36593);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39833);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36594);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39834);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25229);
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
				['о', s @ ..] => {
					super::push(out, 31743);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36595);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
				['о', s @ ..] => {
					super::push(out, 31744);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
				['о', s @ ..] => {
					super::push(out, 31745);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
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
				['о', s @ ..] => {
					super::push(out, 31746);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36598);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -виваться.
pub(crate) fn n25841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18152);
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 25230);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['и', 'р', 'п', s @ ..] => {
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
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гиваться.
pub(crate) fn n25842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18153);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['р', 'т', 'с', s @ ..] => {
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
						['с', 'и', '\u{1}', s @ ..] => {
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
				['ш', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'ы', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
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
						['д', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'м', s @ ..] => {
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
		['о', 'р', 'т', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['е', 'д', s @ ..] => {
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
						['б', 'о', '\u{1}', s @ ..] => super::t29::n67200(s, out),
						['в', '\u{1}', s @ ..] => super::t29::n67201(s, out),
						['д', 'о', 'п', '\u{1}', s @ ..] => super::t29::n67202(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t29::n67203(s, out),
						['з', s @ ..] => super::t29::n67204(s, out),
						['и', 'р', 'п', '\u{1}', s @ ..] => super::t29::n67205(s, out),
						['о', s @ ..] => super::t29::n67206(s, out),
						['с', '\u{1}', s @ ..] => super::t29::n67207(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t29::n67208(s, out),
						['ы', 'в', s @ ..] => super::t29::n67209(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t29::n40510(s, out),
		['ы', s @ ..] => super::t29::n40511(s, out),
		['я', 'т', s @ ..] => super::t29::n40512(s, out),
		_ => false,
	}
}

/// Words ending -еиваться.
pub(crate) fn n25843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18154);
	match s {
		[] => true,
		['в', s @ ..] => {
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
				['с', '\u{1}', s @ ..] => {
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
		['д', 'а', 'н', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'к', s @ ..] => {
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
		['м', s @ ..] => {
			super::push(out, 25231);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36599);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['п', 'о', 'р', 'в', 'е', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
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
		['т', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -живаться.
pub(crate) fn n25844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18155);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25232);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', s @ ..] => {
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
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
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
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
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
								['е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 31747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t29::n54831(s, out),
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => super::t29::n77171(s, out),
								_ => false,
							}
						}
						['р', 'о', 'т', s @ ..] => super::t29::n67284(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t29::n54833(s, out),
				['р', s @ ..] => super::t29::n54834(s, out),
				['с', s @ ..] => super::t29::n54835(s, out),
				['х', s @ ..] => super::t29::n54836(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t29::n40521(s, out),
		['в', s @ ..] => super::t29::n40522(s, out),
		['е', s @ ..] => super::t29::n40523(s, out),
		['ж', 'а', 'в', s @ ..] => super::t29::n40524(s, out),
		['з', s @ ..] => super::t29::n40525(s, out),
		['и', s @ ..] => super::t29::n40526(s, out),
		['л', 'а', 'д', 'о', '\u{1}', s @ ..] => super::t29::n40527(s, out),
		['о', s @ ..] => super::t29::n40528(s, out),
		['р', s @ ..] => super::t29::n40529(s, out),
		['с', s @ ..] => super::t29::n40530(s, out),
		['т', s @ ..] => super::t29::n40531(s, out),
		['у', s @ ..] => super::t30::n40532(s, out),
		['ы', s @ ..] => super::t30::n40533(s, out),
		['ю', s @ ..] => super::t30::n40534(s, out),
		['я', 'р', s @ ..] => super::t30::n40535(s, out),
		_ => false,
	}
}

/// Words ending -зиваться.
pub(crate) fn n25845(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'у', 'ж', 'р', 'у', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'л', 'е', 'ж', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'м', 'е', 'р', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'е', 'в', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -киваться.
pub(crate) fn n25846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18156);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
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
								['т', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							match s {
								[] => true,
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
				['м', s @ ..] => {
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
				['т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'а', 'р', 'а', 'к', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
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
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'к', s @ ..] => {
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
				['х', 'и', 'х', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', s @ ..] => {
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
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
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
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => super::t30::n77279(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t30::n67434(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t30::n67435(s, out),
						_ => false,
					}
				}
				['е', 'щ', s @ ..] => super::t30::n54899(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t30::n40544(s, out),
		['о', 'ч', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t30::n40545(s, out),
		['п', 'е', 'р', 'у', 'к', 'с', 'и', '\u{1}', s @ ..] => super::t30::n40546(s, out),
		['р', s @ ..] => super::t30::n40547(s, out),
		['с', s @ ..] => super::t30::n40548(s, out),
		['у', s @ ..] => super::t30::n40549(s, out),
		['ч', s @ ..] => super::t30::n40550(s, out),
		['ы', s @ ..] => super::t30::n40551(s, out),
		['ь', 'с', 'у', s @ ..] => super::t30::n40552(s, out),
		['ю', s @ ..] => super::t30::n40553(s, out),
		_ => false,
	}
}

/// Words ending -ливаться.
pub(crate) fn n25847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18157);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25245);
			match s {
				[] => true,
				['б', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 31770);
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
						['х', s @ ..] => {
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'н', 'а', s @ ..] => {
					match s {
						[] => true,
						['к', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 31771);
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
					super::push(out, 31772);
					match s {
						[] => true,
						['а', 'з', s @ ..] => super::t30::n67504(s, out),
						['б', 'о', '\u{1}', s @ ..] => super::t30::n67505(s, out),
						['д', 'о', 'п', '\u{1}', s @ ..] => super::t30::n67506(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t30::n67507(s, out),
						['о', s @ ..] => super::t30::n67508(s, out),
						['с', 'а', 'р', '\u{1}', s @ ..] => super::t30::n67509(s, out),
						['ш', 'а', 'з', '\u{1}', s @ ..] => super::t30::n67510(s, out),
						['ы', s @ ..] => super::t30::n67511(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t30::n54929(s, out),
				['н', s @ ..] => super::t30::n54930(s, out),
				['п', s @ ..] => super::t30::n54931(s, out),
				['р', 'т', s @ ..] => super::t30::n54932(s, out),
				['с', s @ ..] => super::t30::n54933(s, out),
				['т', s @ ..] => super::t30::n54934(s, out),
				['ч', s @ ..] => super::t30::n54935(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t30::n40555(s, out),
		['в', s @ ..] => super::t30::n40556(s, out),
		['г', s @ ..] => super::t30::n40557(s, out),
		['д', s @ ..] => super::t30::n40558(s, out),
		['е', s @ ..] => super::t30::n40559(s, out),
		['з', s @ ..] => super::t30::n40560(s, out),
		['и', s @ ..] => super::t30::n40561(s, out),
		['к', s @ ..] => super::t30::n40562(s, out),
		['м', s @ ..] => super::t30::n40563(s, out),
		['о', s @ ..] => super::t30::n40564(s, out),
		['п', s @ ..] => super::t30::n40565(s, out),
		['р', s @ ..] => super::t30::n40566(s, out),
		['с', s @ ..] => super::t30::n40567(s, out),
		['т', s @ ..] => super::t30::n40568(s, out),
		['у', s @ ..] => super::t30::n40569(s, out),
		['х', s @ ..] => super::t30::n40570(s, out),
		['ш', s @ ..] => super::t30::n40571(s, out),
		['ы', s @ ..] => super::t30::n40572(s, out),
		['ю', 'д', 'з', 'и', 'п', 'о', '\u{1}', s @ ..] => super::t30::n40573(s, out),
		['я', s @ ..] => super::t30::n40574(s, out),
		_ => false,
	}
}

/// Words ending -ниваться.
pub(crate) fn n25848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18158);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25260);
			match s {
				[] => true,
				['б', 'а', s @ ..] => {
					match s {
						[] => true,
						['р', 'а', 'б', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
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
						['л', 'о', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
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
						['ы', 'ц', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'т', 'с', 'и', 'р', 'х', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', 'т', 'с', s @ ..] => {
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
						['е', 'ч', s @ ..] => {
							match s {
								[] => true,
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
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'а', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', 'к', 'и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
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
										['с', 'а', 'б', 'о', '\u{1}', s @ ..] => {
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
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t30::n55024(s, out),
				['с', 'о', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['щ', 'е', 'м', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => super::t30::n67727(s, out),
						['о', '\u{1}', s @ ..] => super::t30::n67728(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'а', 'р', s @ ..] => super::t30::n40576(s, out),
		['д', 'е', 'б', 'и', 'р', 'п', '\u{1}', s @ ..] => super::t30::n40577(s, out),
		['е', s @ ..] => super::t30::n40578(s, out),
		['ж', 'а', 'р', 'о', 'п', s @ ..] => super::t30::n40579(s, out),
		['з', s @ ..] => super::t30::n40580(s, out),
		['и', s @ ..] => super::t30::n40581(s, out),
		['о', s @ ..] => super::t30::n40582(s, out),
		['р', 'е', 'ч', 'ы', 'в', '\u{1}', s @ ..] => super::t30::n40583(s, out),
		['с', s @ ..] => super::t30::n40584(s, out),
		['у', s @ ..] => super::t30::n40585(s, out),
		['ч', 'а', 'р', 'з', 'о', 'р', 'п', 'о', '\u{1}', s @ ..] => super::t30::n40586(s, out),
		['ш', 'а', 'м', 'о', 'д', 'о', '\u{1}', s @ ..] => super::t30::n40587(s, out),
		['ы', 'т', 'с', 'у', 'п', 'о', '\u{1}', s @ ..] => super::t30::n40588(s, out),
		['ю', s @ ..] => super::t30::n40589(s, out),
		['я', s @ ..] => super::t30::n40590(s, out),
		_ => false,
	}
}

/// Words ending -оиваться.
pub(crate) fn n25849(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', 'с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'о', 'п', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -пиваться.
pub(crate) fn n25850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18159);
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

/// Words ending -риваться.
pub(crate) fn n25851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18160);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25262);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31802);
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
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41658);
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
										['б', 'о', '\u{1}', s @ ..] => super::t30::n83673(s, out),
										['д', 'о', 'п', '\u{1}', s @ ..] => super::t30::n83674(s, out),
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t30::n83675(s, out),
										['з', 'а', 'р', '\u{1}', s @ ..] => super::t30::n83676(s, out),
										['и', 'р', 'п', '\u{1}', s @ ..] => super::t30::n83677(s, out),
										['о', s @ ..] => super::t30::n83678(s, out),
										['с', '\u{1}', s @ ..] => super::t30::n83679(s, out),
										['т', 'о', '\u{1}', s @ ..] => super::t30::n83680(s, out),
										['у', '\u{1}', s @ ..] => super::t30::n83681(s, out),
										['ы', 'в', '\u{1}', s @ ..] => super::t30::n83682(s, out),
										_ => false,
									}
								}
								['д', s @ ..] => super::t30::n77664(s, out),
								['р', 'п', '\u{1}', s @ ..] => super::t30::n77665(s, out),
								['т', s @ ..] => super::t30::n77666(s, out),
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => super::t30::n67805(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t30::n67806(s, out),
						['у', '\u{1}', s @ ..] => super::t30::n67807(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t30::n67808(s, out),
						_ => false,
					}
				}
				['г', 'е', 'ч', 'о', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => super::t30::n55086(s, out),
				['д', s @ ..] => super::t30::n55087(s, out),
				['ж', s @ ..] => super::t30::n55088(s, out),
				['з', s @ ..] => super::t30::n55089(s, out),
				['м', s @ ..] => super::t30::n55090(s, out),
				['п', s @ ..] => super::t30::n55091(s, out),
				['с', s @ ..] => super::t30::n55092(s, out),
				['т', s @ ..] => super::t30::n55093(s, out),
				['у', 'т', 'а', 'с', 'к', 'и', 'ф', s @ ..] => super::t30::n55094(s, out),
				['х', s @ ..] => super::t30::n55095(s, out),
				['ш', s @ ..] => super::t30::n55096(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t30::n40605(s, out),
		['г', 'а', 'б', s @ ..] => super::t30::n40606(s, out),
		['д', s @ ..] => super::t30::n40607(s, out),
		['е', s @ ..] => super::t30::n40608(s, out),
		['з', s @ ..] => super::t30::n40609(s, out),
		['и', s @ ..] => super::t30::n40610(s, out),
		['о', s @ ..] => super::t30::n40611(s, out),
		['т', s @ ..] => super::t30::n40612(s, out),
		['у', s @ ..] => super::t30::n40613(s, out),
		['х', s @ ..] => super::t30::n40614(s, out),
		['ы', s @ ..] => super::t30::n40615(s, out),
		['ю', 'т', 'в', '\u{1}', s @ ..] => super::t30::n40616(s, out),
		['я', s @ ..] => super::t30::n40617(s, out),
		_ => false,
	}
}

/// Words ending -чуиваться.
pub(crate) fn n25853(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хиваться.
pub(crate) fn n25854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18161);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
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
						['з', 'в', '\u{1}', s @ ..] => {
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
				['п', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', 'б', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
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
				['ч', s @ ..] => {
					match s {
						[] => true,
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
		['р', 'е', 'п', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
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
			match s {
				[] => true,
				['д', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			match s {
				[] => true,
				['л', 'п', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => super::t30::n77810(s, out),
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => super::t30::n68001(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t30::n68002(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', 'р', 'т', s @ ..] => super::t30::n40627(s, out),
		_ => false,
	}
}

/// Words ending -чиваться.
pub(crate) fn n25855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18162);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25271);
			match s {
				[] => true,
				['б', 'о', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => super::t30::n68011(s, out),
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'а', 'з', 'о', '\u{1}', s @ ..] => super::t30::n55179(s, out),
				['з', 'а', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => super::t30::n55180(s, out),
				['к', s @ ..] => super::t30::n55181(s, out),
				['л', s @ ..] => super::t30::n55182(s, out),
				['м', s @ ..] => super::t30::n55183(s, out),
				['н', s @ ..] => super::t30::n55184(s, out),
				['п', s @ ..] => super::t30::n55185(s, out),
				['р', s @ ..] => super::t30::n55186(s, out),
				['с', s @ ..] => super::t30::n55187(s, out),
				['т', s @ ..] => super::t30::n55188(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t30::n40629(s, out),
		['и', s @ ..] => super::t30::n40630(s, out),
		['л', s @ ..] => super::t30::n40631(s, out),
		['н', s @ ..] => super::t30::n40632(s, out),
		['о', s @ ..] => super::t30::n40633(s, out),
		['п', 'а', 'к', s @ ..] => super::t30::n40634(s, out),
		['р', 'е', s @ ..] => super::t30::n40635(s, out),
		['у', s @ ..] => super::t30::n40636(s, out),
		['ы', s @ ..] => super::t30::n40637(s, out),
		['ь', 'н', 'я', 'н', 'ы', 'в', '\u{1}', s @ ..] => super::t30::n40638(s, out),
		['ю', s @ ..] => super::t30::n40639(s, out),
		['я', s @ ..] => super::t30::n40640(s, out),
		_ => false,
	}
}

/// Words ending -шиваться.
pub(crate) fn n25856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18163);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25275);
			match s {
				[] => true,
				['в', 'к', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 31833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
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
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', s @ ..] => {
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t30::n55258(s, out),
				['п', 'о', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => super::t30::n55260(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 25276);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t30::n68247(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t30::n40643(s, out),
		['д', s @ ..] => super::t30::n40644(s, out),
		['е', s @ ..] => super::t30::n40645(s, out),
		['и', s @ ..] => super::t30::n40646(s, out),
		['о', s @ ..] => super::t30::n40647(s, out),
		['р', 'е', 'в', s @ ..] => super::t30::n40648(s, out),
		['с', s @ ..] => super::t30::n40649(s, out),
		['т', s @ ..] => super::t30::n40650(s, out),
		['у', s @ ..] => super::t30::n40651(s, out),
		['ы', s @ ..] => super::t30::n40652(s, out),
		['я', 'м', 'е', 'т', 'в', '\u{1}', s @ ..] => super::t30::n40653(s, out),
		_ => false,
	}
}

/// Words ending -щиваться.
pub(crate) fn n25857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18164);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25286);
			match s {
				[] => true,
				['в', s @ ..] => {
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
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
				['к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
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
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['х', s @ ..] => {
									match s {
										[] => true,
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
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31849);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 39865);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 41665);
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
						_ => false,
					}
				}
				['р', s @ ..] => {
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
								['т', 'ы', 'в', '\u{1}', s @ ..] => {
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
						['т', s @ ..] => {
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', 'з', '\u{1}', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25287);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31850);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36659);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 39866);
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
				['л', s @ ..] => {
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
				['р', s @ ..] => super::t30::n55299(s, out),
				['ч', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t30::n40656(s, out),
		['р', 'о', s @ ..] => super::t30::n40657(s, out),
		['у', 'л', s @ ..] => super::t30::n40658(s, out),
		['ю', 'л', 'п', s @ ..] => super::t30::n40659(s, out),
		_ => false,
	}
}

/// Words ending -воваться.
pub(crate) fn n25859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18166);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25294);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36664);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36665);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36666);
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
			super::push(out, 25295);
			match s {
				[] => true,
				['р', 'е', 'ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36667);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36668);
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
				['с', s @ ..] => super::t30::n55320(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -говаться.
pub(crate) fn n25860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18167);
	match s {
		[] => true,
		['з', 'о', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31859);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 31860);
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
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31861);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39885);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39886);
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
						_ => false,
					}
				}
				['р', 'т', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36680);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36681);
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
		['н', s @ ..] => {
			super::push(out, 25296);
			match s {
				[] => true,
				['а', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'л', 'е', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36682);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 39887);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					match s {
						[] => true,
						['л', 'д', 'у', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', 'ю', 'ь', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'о', 'х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'м', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'р', 'а', 'п', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39888);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41687);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
								['и', s @ ..] => {
									super::push(out, 39889);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 41688);
											match s {
												[] => true,
												['а', 'н', '\u{1}', s @ ..] => {
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
								['й', 'е', 'р', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25297);
			match s {
				[] => true,
				['т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25298);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36683);
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
						['о', s @ ..] => {
							super::push(out, 36684);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31862);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 31863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 25299);
			match s {
				[] => true,
				['л', 'е', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -доваться.
pub(crate) fn n25861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18168);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25300);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36685);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36686);
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
							super::push(out, 36687);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 36688);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36689);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 36690);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 39890);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 41689);
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
				['с', 'о', 'д', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25301);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31864);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36691);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39891);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 31865);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 39892);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39893);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 41690);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42664);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t30::n88649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31866);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 41691);
											match s {
												[] => true,
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
									super::push(out, 39894);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 41692);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42665);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 42666);
													match s {
														[] => true,
														['д', s @ ..] => super::t30::n88650(s, out),
														['п', '\u{1}', s @ ..] => super::t30::n88651(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => super::t30::n78167(s, out),
								['о', s @ ..] => super::t30::n78168(s, out),
								['с', s @ ..] => super::t30::n78169(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t30::n55338(s, out),
				['с', s @ ..] => super::t30::n55339(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t30::n40676(s, out),
		['и', s @ ..] => super::t30::n40677(s, out),
		['л', s @ ..] => super::t30::n40678(s, out),
		['н', s @ ..] => super::t30::n40679(s, out),
		['о', s @ ..] => super::t30::n40680(s, out),
		['р', s @ ..] => super::t30::n40681(s, out),
		['у', s @ ..] => super::t30::n40682(s, out),
		_ => false,
	}
}

/// Words ending -зоваться.
pub(crate) fn n25862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18169);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25309);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 31878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31879);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 39916);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 41704);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 39917);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 41705);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25310);
			match s {
				[] => true,
				['р', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t30::n40685(s, out),
		['м', s @ ..] => {
			super::push(out, 25312);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31888);
					match s {
						[] => true,
						['п', s @ ..] => {
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
		['ь', s @ ..] => {
			super::push(out, 25313);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31889);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39947);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39948);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41742);
											match s {
												[] => true,
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
									super::push(out, 39949);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 41743);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 42698);
													match s {
														[] => true,
														['р', 'е', 'п', '\u{1}', s @ ..] => super::t30::n88696(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 42699);
													match s {
														[] => true,
														['д', s @ ..] => super::t30::n88697(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 41744);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 42700);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t30::n88698(s, out),
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
									super::push(out, 39950);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 41745);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25314);
			match s {
				[] => true,
				['л', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коваться.
pub(crate) fn n25863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18170);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25315);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25316);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36715);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36716);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36717);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 36718);
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
				['м', s @ ..] => {
					super::push(out, 31891);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31892);
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
							super::push(out, 36719);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36720);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 36721);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 36722);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 36723);
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
				['р', s @ ..] => {
					super::push(out, 31893);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39951);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39952);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 39953);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 39954);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 41746);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31894);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 25318);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31896);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25319);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25320);
			match s {
				[] => true,
				['л', 'б', 'у', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36726);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36727);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 36728);
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
				['н', s @ ..] => {
					super::push(out, 31897);
					match s {
						[] => true,
						['а', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31898);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 36729);
							match s {
								[] => true,
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
					super::push(out, 31899);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36730);
							match s {
								[] => true,
								['а', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 41747);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39955);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 39956);
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
						['к', 'а', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39957);
									s.is_empty()
								}
								['а', s @ ..] => super::t30::n78295(s, out),
								['о', s @ ..] => super::t30::n78296(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t30::n55381(s, out),
				['ш', s @ ..] => super::t30::n55382(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t30::n40695(s, out),
		['м', s @ ..] => super::t30::n40696(s, out),
		['н', s @ ..] => super::t30::n40697(s, out),
		['о', s @ ..] => super::t30::n40698(s, out),
		['р', s @ ..] => super::t30::n40699(s, out),
		['с', s @ ..] => super::t30::n40700(s, out),
		['т', s @ ..] => super::t30::n40701(s, out),
		['у', s @ ..] => super::t30::n40702(s, out),
		['ч', s @ ..] => super::t30::n40703(s, out),
		['ы', s @ ..] => super::t30::n40704(s, out),
		['ю', 'т', s @ ..] => super::t30::n40705(s, out),
		_ => false,
	}
}

/// Words ending -ловаться.
pub(crate) fn n25864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18171);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25331);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36741);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36742);
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
							super::push(out, 36743);
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
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 31914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36744);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36745);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 36746);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 39968);
									match s {
										[] => true,
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
							super::push(out, 36747);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 36748);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36749);
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
				['ц', s @ ..] => {
					super::push(out, 31915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 31916);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36750);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41752);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 41753);
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
		['е', s @ ..] => {
			super::push(out, 25332);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 31917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36751);
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
							super::push(out, 36752);
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
		['и', s @ ..] => {
			super::push(out, 25333);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36753);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36754);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39969);
									match s {
										[] => true,
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
							super::push(out, 36755);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39970);
									match s {
										[] => true,
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
							super::push(out, 36756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25334);
			match s {
				[] => true,
				['е', 'т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'ц', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25335);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36757);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36758);
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
			super::push(out, 25336);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31918);
					match s {
						[] => true,
						['в', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25337);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31919);
					match s {
						[] => true,
						['и', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -формоваться.
pub(crate) fn n68522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36760);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39971);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
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

/// Words ending -рифмоваться.
pub(crate) fn n40714(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 31922);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word шельмоваться.
pub(crate) fn n55427(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -новаться.
pub(crate) fn n25866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18173);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25340);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 36761);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39972);
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
		['г', s @ ..] => {
			super::push(out, 25341);
			match s {
				[] => true,
				['и', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36762);
							match s {
								[] => true,
								['р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 39973);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 41754);
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
								['р', s @ ..] => {
									super::push(out, 39974);
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
		['д', s @ ..] => {
			super::push(out, 25342);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31924);
					match s {
						[] => true,
						['а', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39975);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39976);
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
									super::push(out, 39977);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 41755);
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
								['о', 'д', '\u{1}', s @ ..] => {
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
		['е', s @ ..] => {
			super::push(out, 25343);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', 'к', 'э', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41756);
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 41757);
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
										_ => false,
									}
								}
								['н', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41758);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 41759);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 42702);
													match s {
														[] => true,
														['р', 'п', '\u{1}', s @ ..] => super::t30::n88699(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 41760);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39978);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39979);
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
						['ы', 'з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25344);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31925);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t30::n55433(s, out),
				['л', s @ ..] => super::t30::n55434(s, out),
				['м', s @ ..] => super::t30::n55435(s, out),
				['р', s @ ..] => super::t30::n55436(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t30::n40721(s, out),
		['о', s @ ..] => super::t30::n40722(s, out),
		['р', s @ ..] => super::t30::n40723(s, out),
		['с', s @ ..] => super::t30::n40724(s, out),
		_ => false,
	}
}

/// Words ending -поваться.
pub(crate) fn n25867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18174);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25349);
			match s {
				[] => true,
				['ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 25350);
			match s {
				[] => true,
				['а', 'т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36772);
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

/// Words ending -роваться.
pub(crate) fn n25868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18175);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25351);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31934);
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
						['с', s @ ..] => {
							super::push(out, 36773);
							match s {
								[] => true,
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
					super::push(out, 31935);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36774);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36775);
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
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 31936);
					match s {
						[] => true,
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
			super::push(out, 25352);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31937);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 31938);
					match s {
						[] => true,
						['л', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 39989);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 41761);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25353);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31939);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36776);
							match s {
								[] => true,
								['л', 'а', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25354);
			match s {
				[] => true,
				['г', 'й', 'е', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'е', 'р', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'е', 'р', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 31940);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['е', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'л', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'ч', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31941);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39990);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 41762);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['о', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 36777);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 41763);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 42703);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t30::n88700(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31942);
					match s {
						[] => true,
						['а', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 39991);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 41764);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'с', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'у', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t30::n40731(s, out),
		['о', s @ ..] => {
			super::push(out, 25356);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31965);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36912);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42024);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42025);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 42978);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t30::n89229(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t30::n55489(s, out),
				['д', s @ ..] => super::t30::n55490(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t30::n40733(s, out),
		['у', s @ ..] => super::t30::n40734(s, out),
		['ф', s @ ..] => super::t30::n40735(s, out),
		['ю', s @ ..] => super::t30::n40736(s, out),
		_ => false,
	}
}

/// Words ending -соваться.
pub(crate) fn n25869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18176);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25361);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25362);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 31979);
					match s {
						[] => true,
						['г', 'о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 40247);
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
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36925);
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
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36926);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
				['т', s @ ..] => {
					super::push(out, 31980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36927);
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
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36928);
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
			super::push(out, 25363);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31981);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36929);
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
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36930);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42028);
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
						['е', s @ ..] => {
							super::push(out, 36931);
							match s {
								[] => true,
								['т', 'н', 'и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42029);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42030);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 42031);
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
		['и', s @ ..] => {
			super::push(out, 25364);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36932);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36933);
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
							super::push(out, 36934);
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
							super::push(out, 36935);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 40248);
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
								['р', s @ ..] => {
									super::push(out, 40249);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42032);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
						['ы', s @ ..] => {
							super::push(out, 36936);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40250);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25365);
			match s {
				[] => true,
				['е', s @ ..] => super::t30::n55512(s, out),
				['о', 'к', s @ ..] => super::t30::n55513(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t30::n40742(s, out),
		['п', 'и', 'г', s @ ..] => super::t30::n40743(s, out),
		['р', s @ ..] => super::t30::n40744(s, out),
		['с', s @ ..] => super::t30::n40745(s, out),
		['у', s @ ..] => super::t30::n40746(s, out),
		['ю', s @ ..] => super::t30::n40747(s, out),
		_ => false,
	}
}

/// Words ending -товаться.
pub(crate) fn n25870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18177);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25371);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31993);
					match s {
						[] => true,
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
			super::push(out, 25372);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31994);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36950);
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
				['в', s @ ..] => {
					super::push(out, 31995);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36951);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42040);
											s.is_empty()
										}
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42041);
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
				['п', s @ ..] => {
					super::push(out, 31996);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36952);
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
		['и', s @ ..] => {
			super::push(out, 25373);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31997);
					match s {
						[] => true,
						['е', 'р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40257);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 40258);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42042);
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
								['к', s @ ..] => {
									super::push(out, 40259);
									match s {
										[] => true,
										['а', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 42982);
													match s {
														[] => true,
														['р', 'е', 'п', '\u{1}', s @ ..] => super::t30::n89233(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31998);
					match s {
						[] => true,
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
			super::push(out, 25374);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31999);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42043);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 42983);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t30::n89234(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 32000);
					match s {
						[] => true,
						['л', 'п', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40260);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 40261);
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
								['у', s @ ..] => {
									super::push(out, 40262);
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
						_ => false,
					}
				}
				['и', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36953);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36954);
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
				['н', s @ ..] => {
					super::push(out, 32001);
					match s {
						[] => true,
						['у', 'п', 'а', 'р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 25375);
			match s {
				[] => true,
				['о', 'б', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t30::n40753(s, out),
		['о', s @ ..] => {
			super::push(out, 25377);
			match s {
				[] => true,
				['г', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t30::n40755(s, out),
		['р', s @ ..] => super::t30::n40756(s, out),
		['с', s @ ..] => super::t30::n40757(s, out),
		['у', s @ ..] => super::t30::n40758(s, out),
		['ф', s @ ..] => {
			super::push(out, 25382);
			match s {
				[] => true,
				['а', 'р', 'д', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => super::t30::n40760(s, out),
		['ю', s @ ..] => super::t30::n40761(s, out),
		['я', s @ ..] => super::t30::n40762(s, out),
		_ => false,
	}
}

/// Words ending -фоваться.
pub(crate) fn n25871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18178);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25386);
			match s {
				[] => true,
				['р', 'т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36974);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36975);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'л', 'б', 'о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 25387);
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36976);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36977);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 36978);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40289);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25388);
			match s {
				[] => true,
				['о', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховаться.
pub(crate) fn n25872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18179);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25389);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25390);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32015);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40290);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40291);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 40292);
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
			super::push(out, 25391);
			match s {
				[] => true,
				['р', 'т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36980);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36981);
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
				['с', s @ ..] => {
					super::push(out, 32016);
					match s {
						[] => true,
						['п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25392);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цоваться.
pub(crate) fn n25873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18180);
	match s {
		[] => true,
		['и', 'р', 'п', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 25393);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 36982);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40293);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'а', 'в', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ь', 'л', 'а', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -далбываться.
pub(crate) fn n55592(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 36992);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40299);
					match s {
						[] => true,
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
		_ => false,
	}
}

/// Words ending -обываться.
pub(crate) fn n40803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25409);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 32031);
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
		_ => false,
	}
}

/// Words ending -тбываться.
pub(crate) fn n40805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25411);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word расхлябываться.
pub(crate) fn n40806(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

