use flexion::Path;

/// Words ending -торный.
pub(crate) fn n3809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3670);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8728);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15624);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 15625);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 15626);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22586);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 22587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15627);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15628);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22589);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22590);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30779);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30780);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 32336);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15629);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22591);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 22592);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 15630);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27745);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 27746);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30781);
											s.is_empty()
										}
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
					super::push(out, 15631);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22593);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22594);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27747);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15632);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15633);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22595);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27748);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 22596);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22597);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15634);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22598);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 22599);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22600);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22601);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27750);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30782);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', 'с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30783);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30784);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													super::push(out, 32337);
													s.is_empty()
												}
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
							super::push(out, 22602);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27751);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 27752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 22603);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27753);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30785);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 27754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15635);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15636);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22604);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22605);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8729);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8730);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15638);
					match s {
						[] => true,
						['к', 'е', 'с', 'и', s @ ..] => {
							super::push(out, 22607);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27755);
									s.is_empty()
								}
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
			super::push(out, 8731);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15639);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15640);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15641);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t10::n9153(s, out),
		['о', s @ ..] => {
			super::push(out, 8733);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15646);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => super::t10::n29149(s, out),
								['с', s @ ..] => super::t10::n29150(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t10::n16438(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t10::n9155(s, out),
		['с', s @ ..] => super::t10::n9156(s, out),
		['у', s @ ..] => super::t10::n9157(s, out),
		['я', s @ ..] => super::t10::n9158(s, out),
		_ => false,
	}
}

/// Words ending -форный.
pub(crate) fn n3810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3671);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8738);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15654);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 8739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8740);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8741);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15657);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27774);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 27775);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 27776);
									s.is_empty()
								}
								['о', 'т', s @ ..] => {
									super::push(out, 27777);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30796);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хорный.
pub(crate) fn n3811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3672);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шорный.
pub(crate) fn n3812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3673);
	match s {
		[] => true,
		['ф', 'о', 'и', s @ ..] => {
			super::push(out, 8743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ускулатурный.
pub(crate) fn n33951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32342);
	s.is_empty()
}

/// Words ending -иттературный.
pub(crate) fn n33952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32343);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 33151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кутикулярный.
pub(crate) fn n33953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32344);
	s.is_empty()
}

/// Words ending -ронавирусный.
pub(crate) fn n33961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32352);
	s.is_empty()
}

/// Words ending -неметный.
pub(crate) fn n16622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15816);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 22722);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22723);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риметный.
pub(crate) fn n16624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15818);
	s.is_empty()
}

/// Words ending -ометный.
pub(crate) fn n9347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8915);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['ж', 'е', 'м', '\u{1}', s @ ..] => {
					super::push(out, 22724);
					s.is_empty()
				}
				['о', 'в', '\u{1}', s @ ..] => {
					super::push(out, 22725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', '\u{1}', s @ ..] => {
			super::push(out, 15819);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 15820);
			match s {
				[] => true,
				['и', 'м', '\u{1}', s @ ..] => {
					super::push(out, 22726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'к', 'с', 'и', '\u{1}', s @ ..] => {
			super::push(out, 15821);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 15822);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сметный.
pub(crate) fn n9348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8916);
	s.is_empty()
}

/// Words ending -нетный.
pub(crate) fn n3923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3780);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8917);
			match s {
				[] => true,
				['л', 'п', 'о', s @ ..] => {
					super::push(out, 15823);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22728);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22729);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27827);
									s.is_empty()
								}
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
			super::push(out, 8918);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 15824);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15825);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15826);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -петный.
pub(crate) fn n3924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3781);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8920);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8921);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15828);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретный.
pub(crate) fn n3925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3782);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8922);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15829);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 15831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8923);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15832);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'а', 'з', 'у', s @ ..] => {
			super::push(out, 8924);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сетный.
pub(crate) fn n3926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3783);
	s.is_empty()
}

/// Words ending -тетный.
pub(crate) fn n3927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3784);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 8925);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уетный.
pub(crate) fn n3928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3785);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 8926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фетный.
pub(crate) fn n3929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3786);
	s.is_empty()
}

/// Words ending -четный.
pub(crate) fn n3930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3787);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8927);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8928);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15835);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15836);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8929);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8930);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 22733);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27828);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 22734);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27829);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27830);
									match s {
										[] => true,
										['х', '\u{1}', s @ ..] => {
											super::push(out, 30824);
											s.is_empty()
										}
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
					super::push(out, 15837);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15838);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8931);
			match s {
				[] => true,
				['о', 'д', s @ ..] => {
					super::push(out, 15840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8932);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 15841);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22736);
							match s {
								[] => true,
								['н', 'н', 'е', 'о', 'в', '\u{1}', s @ ..] => {
									super::push(out, 27831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шетный.
pub(crate) fn n3931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3788);
	s.is_empty()
}

/// The word четырехвалентный.
pub(crate) fn n33963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32354);
	s.is_empty()
}

/// Words ending -центный.
pub(crate) fn n9403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8968);
	match s {
		[] => true,
		['о', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22758);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22759);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27845);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 22760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15883);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['н', 'и', s @ ..] => {
							super::push(out, 27847);
							match s {
								[] => true,
								['м', 'ю', 'л', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32355);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 32356);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22761);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онтный.
pub(crate) fn n3953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3809);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 8970);
			s.is_empty()
		}
		['м', 'е', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15885);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', 'у', 'р', s @ ..] => {
							super::push(out, 22762);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22763);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22764);
							match s {
								[] => true,
								['о', 'т', 'к', 'а', 'р', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30830);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30831);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'е', s @ ..] => {
							super::push(out, 22765);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22766);
							match s {
								[] => true,
								['е', 'л', 'о', 'т', 'р', 'е', 'в', '\u{1}', s @ ..] => {
									super::push(out, 27852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -энтный.
pub(crate) fn n3954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3810);
	s.is_empty()
}

/// Words ending -янтный.
pub(crate) fn n3955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3811);
	s.is_empty()
}

/// Words ending -остный.
pub(crate) fn n3978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9036);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9037);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9038);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15944);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15945);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15946);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9039);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9040);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 15947);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22809);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15948);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22811);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 22812);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 15949);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15950);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15951);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22815);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15952);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15953);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9041);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15955);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15956);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15957);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15958);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9042);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15959);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15960);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15961);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9043);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15962);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 15963);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22822);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15964);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22823);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22824);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15965);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 15966);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 15967);
					match s {
						[] => true,
						['и', 'л', 'ж', s @ ..] => {
							super::push(out, 22825);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 15968);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22826);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27865);
									s.is_empty()
								}
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
			super::push(out, 9044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15969);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15970);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15971);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ушерстный.
pub(crate) fn n23999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22830);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27869);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устный.
pub(crate) fn n3980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3836);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 9046);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9047);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстный.
pub(crate) fn n3981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3837);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9048);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word двадцатитрехминутный.
pub(crate) fn n33966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32357);
	s.is_empty()
}

/// The word тридцатитрехминутный.
pub(crate) fn n33967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32358);
	s.is_empty()
}

/// Words ending -сечный.
pub(crate) fn n4038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3893);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9117);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16038);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9118);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9119);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 9120);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -течный.
pub(crate) fn n4039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3894);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9121);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16041);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16042);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чечный.
pub(crate) fn n4040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3895);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9123);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16045);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16046);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22875);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9124);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шечный.
pub(crate) fn n4041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3896);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9125);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16048);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 16049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22878);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9126);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 9127);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16050);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16051);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9128);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9129);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16052);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9130);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16053);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22883);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22884);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'а', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22885);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16054);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 22887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22888);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16055);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22890);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 16056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9131);
			match s {
				[] => true,
				['в', 'д', s @ ..] => {
					super::push(out, 16057);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 9132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щечный.
pub(crate) fn n4042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3897);
	s.is_empty()
}

/// The word трехгодичный.
pub(crate) fn n33968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32359);
	s.is_empty()
}

/// Words ending -кличный.
pub(crate) fn n9594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9148);
	s.is_empty()
}

/// Words ending -лличный.
pub(crate) fn n9595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9149);
	match s {
		[] => true,
		['и', 'р', s @ ..] => {
			super::push(out, 16081);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оличный.
pub(crate) fn n9596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9150);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 16082);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 22911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сличный.
pub(crate) fn n9597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9151);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16083);
			match s {
				[] => true,
				['м', 'о', s @ ..] => {
					super::push(out, 22912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тличный.
pub(crate) fn n9598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9152);
	s.is_empty()
}

/// Words ending -уличный.
pub(crate) fn n9599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9153);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16085);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 16086);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 16087);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16088);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яличный.
pub(crate) fn n9600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9154);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мичный.
pub(crate) fn n4049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3904);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9155);
			s.is_empty()
		}
		['и', 'н', 'о', 'н', s @ ..] => {
			super::push(out, 9156);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9157);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ничный.
pub(crate) fn n4050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9158);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16091);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22915);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27899);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27900);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30845);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32360);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 22916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9159);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 9160);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16092);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27901);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 27902);
									s.is_empty()
								}
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
			super::push(out, 9161);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16093);
					match s {
						[] => true,
						['а', 'р', 'п', 'д', s @ ..] => {
							super::push(out, 22917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 16094);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22918);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9162);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 16096);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16097);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16098);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22921);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22922);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22923);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 27904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16100);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 16101);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 16102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 9163);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22924);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 22925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9164);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16103);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22926);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30846);
											s.is_empty()
										}
										['д', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32361);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 32362);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 32363);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16104);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16105);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 16106);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9165);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16107);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16108);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22929);
							match s {
								[] => true,
								['о', 'г', 'и', s @ ..] => {
									super::push(out, 27905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 16109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 9166);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9167);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 9168);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16111);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9169);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16112);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16113);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9171);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9172);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16116);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 16117);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9173);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 9174);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 9175);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22933);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27906);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 27907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22934);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27908);
									s.is_empty()
								}
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
			super::push(out, 9176);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16120);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16121);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16122);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16123);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оичный.
pub(crate) fn n4051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3906);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9177);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9178);
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					super::push(out, 16124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пичный.
pub(crate) fn n4052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3907);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9179);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16125);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9180);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9181);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ричный.
pub(crate) fn n4053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3908);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9182);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16127);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22937);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 22938);
							match s {
								[] => true,
								['п', '-', s @ ..] => {
									super::push(out, 27909);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30847);
											s.is_empty()
										}
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
					super::push(out, 16128);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 22939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9183);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9184);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9185);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16130);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 22940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16131);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 16132);
					match s {
						[] => true,
						['е', 'ц', 'о', 'г', s @ ..] => {
							super::push(out, 22941);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27910);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 27911);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30848);
											s.is_empty()
										}
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
					super::push(out, 16133);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сичный.
pub(crate) fn n4054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3909);
	match s {
		[] => true,
		['к', 'о', 'т', s @ ..] => {
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					super::push(out, 16134);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16135);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 16136);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тичный.
pub(crate) fn n4055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3910);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9186);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22945);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22946);
							match s {
								[] => true,
								['л', 'б', 'о', s @ ..] => {
									super::push(out, 27912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 22947);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27913);
									match s {
										[] => true,
										['н', 'э', '\u{1}', s @ ..] => {
											super::push(out, 30849);
											s.is_empty()
										}
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
					super::push(out, 16137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9187);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16138);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22948);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 27914);
									s.is_empty()
								}
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
			super::push(out, 9188);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9189);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16139);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16140);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16141);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9190);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 9191);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16142);
					match s {
						[] => true,
						['л', 'п', s @ ..] => {
							super::push(out, 22950);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									super::push(out, 27915);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30850);
											s.is_empty()
										}
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
					super::push(out, 16143);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16144);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22952);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									super::push(out, 27916);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 30851);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 32364);
													s.is_empty()
												}
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
							super::push(out, 22953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фичный.
pub(crate) fn n4056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3911);
	s.is_empty()
}

/// Words ending -хичный.
pub(crate) fn n4057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3912);
	s.is_empty()
}

/// Words ending -чичный.
pub(crate) fn n4058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3913);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9192);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 9193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щичный.
pub(crate) fn n4059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3914);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 9194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьичный.
pub(crate) fn n4060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3915);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 9195);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овочный.
pub(crate) fn n9653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9204);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 16158);
			match s {
				[] => true,
				['а', 'п', 'с', s @ ..] => {
					super::push(out, 22965);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22966);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 22967);
					match s {
						[] => true,
						['а', 'п', 'и', s @ ..] => {
							super::push(out, 27932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16159);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22968);
					match s {
						[] => true,
						['г', 'х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 27933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22969);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 22970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16160);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16161);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22971);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['в', 'и', s @ ..] => {
							super::push(out, 27936);
							s.is_empty()
						}
						['д', 'н', 'а', s @ ..] => {
							super::push(out, 27937);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 27938);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 27939);
							s.is_empty()
						}
						['м', 'м', s @ ..] => {
							super::push(out, 27940);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 30854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 27941);
							match s {
								[] => true,
								['а', 'л', 'п', '-', 'о', 'н', 'м', 'е', 'ъ', 'б', 'о', '\u{1}', s @ ..] => {
									super::push(out, 30855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'н', s @ ..] => {
							super::push(out, 27942);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'р', s @ ..] => {
							super::push(out, 27943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22972);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 27944);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 30857);
									s.is_empty()
								}
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
			super::push(out, 16162);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 16163);
			match s {
				[] => true,
				['н', 'и', s @ ..] => {
					super::push(out, 22973);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 16164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дочный.
pub(crate) fn n4065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3920);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9206);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22975);
							match s {
								[] => true,
								['н', 'о', s @ ..] => {
									super::push(out, 27945);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 30858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22976);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27946);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30859);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32365);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16165);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16166);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22977);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['-', 'о', 'н', 'т', 'е', 'л', 'з', 'в', '\u{1}', s @ ..] => {
											super::push(out, 30860);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30861);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 32366);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n34801(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 32367);
													match s {
														[] => true,
														['е', s @ ..] => super::t10::n34802(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 30862);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9207);
			match s {
				[] => true,
				['в', 'з', 'а', 'р', 'о', 'г', s @ ..] => {
					super::push(out, 16167);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16168);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22980);
							s.is_empty()
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
				['е', s @ ..] => {
					super::push(out, 16169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9209);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9210);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16170);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16171);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 22982);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27947);
									match s {
										[] => true,
										['р', 'е', 'к', 'и', 'л', '\u{1}', s @ ..] => {
											super::push(out, 30863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['а', s @ ..] => {
							super::push(out, 22983);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22984);
							match s {
								[] => true,
								['р', 'е', 'к', 'и', 'л', '\u{1}', s @ ..] => {
									super::push(out, 27949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16172);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'с', s @ ..] => {
			super::push(out, 9211);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 9212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лочный.
pub(crate) fn n4067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3922);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9218);
			match s {
				[] => true,
				['б', 'и', s @ ..] => {
					super::push(out, 16174);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16175);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16176);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9219);
			match s {
				[] => true,
				['-', 'о', 'н', 'м', 'е', 'ъ', 'б', 'о', '\u{1}', s @ ..] => {
					super::push(out, 16177);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16178);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 16179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9220);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16180);
					match s {
						[] => true,
						['б', 'о', 'о', s @ ..] => {
							super::push(out, 22989);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 22990);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22991);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22992);
							s.is_empty()
						}
						['т', 'о', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
							super::push(out, 22993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16181);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16182);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22994);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 22995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27952);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9221);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16184);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 9222);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9223);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9224);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16187);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16188);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22998);
							match s {
								[] => true,
								['р', 'п', 'о', s @ ..] => {
									super::push(out, 27953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 16189);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16190);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16191);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23000);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27954);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30864);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 27955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23001);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16193);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16194);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23003);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 23004);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27958);
									s.is_empty()
								}
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
			super::push(out, 9225);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16195);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 16196);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16197);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16198);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 16199);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9226);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16200);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23007);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мочный.
pub(crate) fn n4068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3923);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9227);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16201);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9228);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 16203);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							super::push(out, 23009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 16204);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9229);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9230);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16205);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16206);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16207);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16208);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9231);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9232);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 9233);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ночный.
pub(crate) fn n4069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3924);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9234);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16210);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 16211);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 16212);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16213);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16214);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 16215);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16216);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23017);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 23018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16217);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16218);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 16219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9235);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16220);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 16221);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16222);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23021);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 16223);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 16224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9236);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16225);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23023);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16226);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16227);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16228);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16229);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 16230);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23027);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9237);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16231);
					match s {
						[] => true,
						['ч', 'о', s @ ..] => {
							super::push(out, 23029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16232);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9238);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16233);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 16234);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23032);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16235);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16236);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							super::push(out, 23033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 16237);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 16238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9239);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9240);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					super::push(out, 16240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9241);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 9242);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16241);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нопочный.
pub(crate) fn n17075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16243);
	s.is_empty()
}

/// Words ending -упочный.
pub(crate) fn n9694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9245);
	s.is_empty()
}

/// Words ending -рочный.
pub(crate) fn n4071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3926);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9246);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16244);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 16245);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23036);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23037);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 27962);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27963);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30865);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 32368);
													match s {
														[] => true,
														['к', 'е', 'л', 'э', '\u{1}', s @ ..] => super::t10::n34803(s, out),
														_ => false,
													}
												}
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
							super::push(out, 23038);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27964);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30866);
											s.is_empty()
										}
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
					super::push(out, 16246);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16247);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16248);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23040);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23041);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23042);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 27965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16249);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9247);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9248);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16251);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23044);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23045);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23046);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9249);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9250);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16253);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23047);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 23048);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23049);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									super::push(out, 27967);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30867);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32369);
													s.is_empty()
												}
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
							super::push(out, 23050);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16254);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16255);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23052);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16256);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16257);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16258);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 23055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16259);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23056);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27969);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 27970);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 30868);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9252);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9253);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9254);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16262);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16263);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16264);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 16265);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 16266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9255);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сочный.
pub(crate) fn n4072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3927);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9256);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9257);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16268);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16269);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27971);
									s.is_empty()
								}
								['х', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 30869);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 30870);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9258);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16270);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9259);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16272);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16273);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16274);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16275);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9260);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16277);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 16278);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16279);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 16280);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16281);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 16282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9261);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16283);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16284);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -точный.
pub(crate) fn n4073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3928);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9263);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9264);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16285);
					match s {
						[] => true,
						['е', 'р', 'е', 'п', 'о', 'м', 'е', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 23063);
							s.is_empty()
						}
						['з', 'а', 'р', 'о', s @ ..] => {
							super::push(out, 23064);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23065);
							match s {
								[] => true,
								['-', 'о', 'м', 'е', 'и', 'р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 27972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16287);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16288);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27973);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 27974);
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
							super::push(out, 23067);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16289);
					match s {
						[] => true,
						['с', 'о', 'д', s @ ..] => {
							super::push(out, 23069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 16290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9265);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16291);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23070);
							match s {
								[] => true,
								['р', 'к', '\u{1}', s @ ..] => {
									super::push(out, 27975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 23071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23072);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 23073);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27976);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 27977);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16293);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 16294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23074);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23075);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 27979);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30871);
											s.is_empty()
										}
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
					super::push(out, 16295);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 16296);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23076);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 27980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 16297);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23077);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27981);
									s.is_empty()
								}
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
			super::push(out, 9266);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16298);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 16299);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16300);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16301);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16302);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9267);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9268);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9269);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16304);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27982);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30872);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 27983);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16305);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 16306);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23082);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16308);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23084);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23086);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 23087);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23088);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27985);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 27986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30873);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 30874);
											s.is_empty()
										}
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
					super::push(out, 16310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23089);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 23090);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 23091);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16311);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 23092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t10::n17145(s, out),
				['р', s @ ..] => super::t10::n17146(s, out),
				['с', s @ ..] => super::t10::n17147(s, out),
				['х', s @ ..] => super::t10::n17148(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t10::n9719(s, out),
		['с', s @ ..] => super::t10::n9720(s, out),
		['у', s @ ..] => super::t10::n9721(s, out),
		['ы', s @ ..] => super::t10::n9722(s, out),
		['э', s @ ..] => super::t10::n9723(s, out),
		['я', s @ ..] => {
			super::push(out, 9275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шочный.
pub(crate) fn n4074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3929);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9276);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9277);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16329);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -альный.
pub(crate) fn n4157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4012);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9358);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 9359);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16379);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16380);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16381);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23128);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27997);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 30878);
											s.is_empty()
										}
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
					super::push(out, 16382);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 23129);
							match s {
								[] => true,
								['с', 'е', 'ф', 'д', s @ ..] => {
									super::push(out, 27998);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30879);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 23130);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27999);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30880);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 28000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16383);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23131);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28001);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 30881);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23132);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28002);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 30882);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23133);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23134);
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30883);
											match s {
												[] => true,
												['о', 'к', 'о', s @ ..] => {
													super::push(out, 32370);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 30884);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32371);
													match s {
														[] => true,
														['ь', s @ ..] => super::t10::n34804(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 16384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9360);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16385);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 16386);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 23135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9361);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16387);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16388);
					match s {
						[] => true,
						['о', 'р', s @ ..] => {
							super::push(out, 23136);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28003);
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
						['а', s @ ..] => {
							super::push(out, 23137);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16389);
					match s {
						[] => true,
						['е', 'ф', '-', s @ ..] => {
							super::push(out, 23139);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28005);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 30885);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 32372);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9362);
			match s {
				[] => true,
				['р', 'о', s @ ..] => {
					super::push(out, 16390);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 23141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9363);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16391);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23142);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 23143);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23144);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28008);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30886);
											s.is_empty()
										}
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
					super::push(out, 16392);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16393);
					match s {
						[] => true,
						['о', 'в', 'и', s @ ..] => {
							super::push(out, 23146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n9818(s, out),
		['к', s @ ..] => {
			super::push(out, 9365);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16405);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23161);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23162);
							match s {
								[] => true,
								['е', s @ ..] => super::t10::n29421(s, out),
								['и', s @ ..] => super::t10::n29422(s, out),
								_ => false,
							}
						}
						['т', 'р', s @ ..] => super::t10::n24347(s, out),
						_ => false,
					}
				}
				['о', 'ф', s @ ..] => super::t10::n17247(s, out),
				['р', 'е', 'з', 'з', s @ ..] => super::t10::n17248(s, out),
				['с', s @ ..] => super::t10::n17249(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t10::n9820(s, out),
		['н', s @ ..] => super::t10::n9821(s, out),
		['п', s @ ..] => super::t10::n9822(s, out),
		['р', s @ ..] => super::t10::n9823(s, out),
		['с', s @ ..] => super::t10::n9824(s, out),
		['т', s @ ..] => super::t10::n9825(s, out),
		['у', s @ ..] => super::t10::n9826(s, out),
		['ф', s @ ..] => super::t10::n9827(s, out),
		['х', 'к', s @ ..] => super::t10::n9828(s, out),
		['ч', s @ ..] => super::t10::n9829(s, out),
		_ => false,
	}
}

/// Words ending -ельный.
pub(crate) fn n4159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4014);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9376);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16445);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23213);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23214);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28060);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 28061);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28062);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 28063);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 30908);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28064);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 30909);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32378);
													match s {
														[] => true,
														['н', s @ ..] => super::t10::n34805(s, out),
														_ => false,
													}
												}
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
							super::push(out, 23215);
							match s {
								[] => true,
								['и', 'ф', s @ ..] => {
									super::push(out, 28065);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30910);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													super::push(out, 32379);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16446);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 23216);
							s.is_empty()
						}
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28066);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16447);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16449);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 16450);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23219);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 16451);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9377);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16452);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 23221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16453);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 23222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9378);
			match s {
				[] => true,
				['и', 'т', s @ ..] => {
					super::push(out, 16455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9379);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16456);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16457);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16458);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23223);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28069);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 30911);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => super::t10::n29468(s, out),
								['о', s @ ..] => super::t10::n29469(s, out),
								['х', s @ ..] => super::t10::n29470(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t10::n24413(s, out),
						['с', s @ ..] => {
							super::push(out, 23225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16459);
					match s {
						[] => true,
						['а', 'р', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => super::t10::n24415(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t10::n17304(s, out),
				['о', s @ ..] => super::t10::n17305(s, out),
				['с', s @ ..] => super::t10::n17306(s, out),
				['т', s @ ..] => super::t10::n17307(s, out),
				['у', s @ ..] => super::t10::n17308(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t10::n9834(s, out),
		['к', s @ ..] => super::t10::n9835(s, out),
		['л', s @ ..] => super::t10::n9836(s, out),
		['м', s @ ..] => super::t10::n9837(s, out),
		['н', s @ ..] => super::t10::n9838(s, out),
		['п', s @ ..] => super::t10::n9839(s, out),
		['р', s @ ..] => super::t10::n9840(s, out),
		['с', s @ ..] => super::t10::n9841(s, out),
		['т', s @ ..] => super::t10::n9842(s, out),
		['ф', s @ ..] => super::t10::n9843(s, out),
		['ц', s @ ..] => super::t10::n9844(s, out),
		['ч', s @ ..] => super::t10::n9845(s, out),
		['щ', s @ ..] => super::t10::n9846(s, out),
		_ => false,
	}
}

/// Words ending -леплавильный.
pub(crate) fn n34057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32442);
	s.is_empty()
}

/// Words ending -симильный.
pub(crate) fn n24506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23315);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 28177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омильный.
pub(crate) fn n17365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16517);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 23316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хмильный.
pub(crate) fn n17367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16519);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 23317);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нильный.
pub(crate) fn n9855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9401);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16520);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23318);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23319);
					match s {
						[] => true,
						['г', 'о', s @ ..] => {
							super::push(out, 28178);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31007);
									s.is_empty()
								}
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
			super::push(out, 16521);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16522);
			match s {
				[] => true,
				['б', 'р', 'а', 'к', 'о', s @ ..] => {
					super::push(out, 23320);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оильный.
pub(crate) fn n9856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9402);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16523);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', s @ ..] => {
			super::push(out, 16524);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пильный.
pub(crate) fn n9857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9403);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16526);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16527);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16528);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 16529);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23325);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рильный.
pub(crate) fn n9858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9404);
	s.is_empty()
}

/// Words ending -сильный.
pub(crate) fn n9859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9405);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16530);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16531);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16532);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16533);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16534);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23327);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тильный.
pub(crate) fn n9860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9406);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16536);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16537);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 23328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16538);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23329);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23330);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'р', s @ ..] => {
			super::push(out, 16539);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23331);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фильный.
pub(crate) fn n9861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9407);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23332);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23333);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23334);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 23335);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23336);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 23337);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23338);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28185);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 28186);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 28187);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 28188);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28189);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цильный.
pub(crate) fn n9862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9408);
	s.is_empty()
}

/// Words ending -чильный.
pub(crate) fn n9863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9409);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16540);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16541);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шильный.
pub(crate) fn n9864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9410);
	s.is_empty()
}

/// Words ending -щильный.
pub(crate) fn n9865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9411);
	s.is_empty()
}

/// Words ending -йльный.
pub(crate) fn n4161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4016);
	s.is_empty()
}

/// Words ending -лльный.
pub(crate) fn n4162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4017);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9412);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23341);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9413);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16542);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольный.
pub(crate) fn n4163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4018);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9414);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16543);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23344);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 23345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16544);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16545);
					match s {
						[] => true,
						['е', 'к', 'с', 'а', 'б', '\u{1}', s @ ..] => {
							super::push(out, 23346);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 23347);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 28192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 23348);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 23349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9415);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16546);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16547);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 16548);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16549);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16550);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23350);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23351);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28193);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31008);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32443);
													s.is_empty()
												}
												_ => false,
											}
										}
										['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 31009);
											s.is_empty()
										}
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
					super::push(out, 16551);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23352);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23353);
							match s {
								[] => true,
								['б', 'о', 'д', 'е', s @ ..] => {
									super::push(out, 28194);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31010);
											s.is_empty()
										}
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
					super::push(out, 16552);
					match s {
						[] => true,
						['с', 'д', s @ ..] => {
							super::push(out, 23354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16553);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16554);
					match s {
						[] => true,
						['к', 'л', 'а', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28195);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 28196);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28197);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 31011);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32444);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 31012);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32445);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23355);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23356);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28198);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 28199);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 31013);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 28200);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 31014);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 28201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 23357);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23358);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 28202);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31015);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32446);
													s.is_empty()
												}
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
							super::push(out, 23359);
							match s {
								[] => true,
								['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 28203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 16556);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 23360);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28204);
									s.is_empty()
								}
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
			super::push(out, 9417);
			match s {
				[] => true,
				['и', 'л', s @ ..] => {
					super::push(out, 16557);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23361);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28205);
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
						['н', s @ ..] => {
							super::push(out, 23362);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 23363);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 28206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 16558);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 28207);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 28208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 16559);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 16560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9418);
			match s {
				[] => true,
				['о', 'р', s @ ..] => {
					super::push(out, 16561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t10::n9873(s, out),
		['м', s @ ..] => super::t10::n9874(s, out),
		['н', s @ ..] => {
			super::push(out, 9421);
			match s {
				[] => true,
				['а', s @ ..] => super::t10::n17422(s, out),
				['е', s @ ..] => super::t10::n17423(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t10::n9876(s, out),
		['р', s @ ..] => super::t10::n9877(s, out),
		['с', s @ ..] => super::t10::n9878(s, out),
		['т', s @ ..] => super::t10::n9879(s, out),
		['у', s @ ..] => super::t10::n9880(s, out),
		['ф', s @ ..] => super::t10::n9881(s, out),
		['х', s @ ..] => super::t10::n9882(s, out),
		_ => false,
	}
}

/// Words ending -ульный.
pub(crate) fn n4164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4019);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9429);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 9430);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 9431);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16588);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9432);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 23401);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28224);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31020);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 23402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9433);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9434);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23403);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 28226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 23404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9435);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 9436);
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16590);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 16591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9437);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16592);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16593);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9438);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9439);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 9440);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыльный.
pub(crate) fn n4165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4020);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9441);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16597);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16598);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9442);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 9443);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 9444);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9445);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 9446);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16600);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16601);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16602);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эльный.
pub(crate) fn n4166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4021);
	s.is_empty()
}

/// Words ending -юльный.
pub(crate) fn n4167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4022);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9448);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9449);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 9450);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яльный.
pub(crate) fn n4168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4023);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9451);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16605);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 23408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9452);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word трехзубчатый.
pub(crate) fn n34063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32448);
	s.is_empty()
}

/// Words ending -стинчатый.
pub(crate) fn n24693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23495);
	s.is_empty()
}

/// Words ending -утинчатый.
pub(crate) fn n24694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23496);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ончатый.
pub(crate) fn n10052(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 16702);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -очатый.
pub(crate) fn n4305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4148);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 9585);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16704);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16705);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пчатый.
pub(crate) fn n4306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4149);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9586);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9587);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9588);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9589);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16708);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9590);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рчатый.
pub(crate) fn n4307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4150);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9591);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16709);
					s.is_empty()
				}
				['в', 'т', 'с', 'х', s @ ..] => {
					super::push(out, 16710);
					match s {
						[] => true,
						['е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 23500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', 'у', 'б', '\u{1}', s @ ..] => {
					super::push(out, 16711);
					s.is_empty()
				}
				['з', 'у', 'о', s @ ..] => {
					super::push(out, 16712);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9592);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -счатый.
pub(crate) fn n4308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4151);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9593);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16713);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9594);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16715);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тчатый.
pub(crate) fn n4309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4152);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9595);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16716);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28274);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16717);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 16718);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 16719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -учатый.
pub(crate) fn n4310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4153);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9597);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16720);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шчатый.
pub(crate) fn n4311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4154);
	s.is_empty()
}

/// Words ending -ьчатый.
pub(crate) fn n4312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4155);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16721);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23503);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28276);
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
						['б', s @ ..] => {
							super::push(out, 23504);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23505);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28277);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 31031);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 28278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16722);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', 'у', s @ ..] => {
							super::push(out, 23506);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 23507);
							s.is_empty()
						}
						['к', '\u{1}', s @ ..] => {
							super::push(out, 23508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зистый.
pub(crate) fn n4417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9740);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9741);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 9742);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16804);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16805);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -листый.
pub(crate) fn n4418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4256);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9743);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16806);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16807);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23543);
							match s {
								[] => true,
								['м', 'с', 'е', s @ ..] => {
									super::push(out, 28293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28294);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16808);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16809);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9744);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9745);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16811);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16812);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 16813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9746);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16814);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 16815);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16816);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16817);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16818);
					s.is_empty()
				}
				['щ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23549);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23550);
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
				['у', s @ ..] => {
					super::push(out, 16819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9748);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16820);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 16821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9749);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16822);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16823);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9750);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23552);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 23553);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16825);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23555);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16826);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23556);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28297);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 31036);
											s.is_empty()
										}
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
					super::push(out, 16827);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16828);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9751);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9752);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9753);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16832);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28298);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 28299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16833);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9754);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16834);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мистый.
pub(crate) fn n4419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4257);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9755);
			match s {
				[] => true,
				['з', 'о', s @ ..] => {
					super::push(out, 16836);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23560);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16837);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9756);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 9757);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16838);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 23562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9758);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16839);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16840);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 23563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16841);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нистый.
pub(crate) fn n4420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4258);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9760);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16842);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16843);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16844);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9761);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 9762);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 9763);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9764);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16845);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16846);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 23565);
							match s {
								[] => true,
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 28301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16847);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16848);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9765);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16850);
					match s {
						[] => true,
						['г', '-', s @ ..] => {
							super::push(out, 23566);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9766);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9767);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9768);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							super::push(out, 23567);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9769);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 9770);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9771);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23569);
							s.is_empty()
						}
						['з', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 28304);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31037);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 32451);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 31038);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 28305);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31039);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32452);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 31040);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28306);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28307);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 31041);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9772);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 9773);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 16855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9774);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 9775);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 9776);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16856);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23572);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 23573);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23574);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 28308);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31042);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 28309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 16857);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16858);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23575);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 28310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 16859);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оистый.
pub(crate) fn n4421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4259);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9777);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9778);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 23578);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пистый.
pub(crate) fn n4422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4260);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9779);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9780);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 9781);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16861);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9782);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16862);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ристый.
pub(crate) fn n4423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4261);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9783);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16863);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23582);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23583);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 16864);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 16865);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23585);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 28313);
									s.is_empty()
								}
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
			super::push(out, 9784);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16866);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16867);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 23586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9785);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16868);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 16869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9786);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16870);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23587);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16871);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 16872);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 23588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 16873);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28315);
									s.is_empty()
								}
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
			super::push(out, 9787);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9788);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16874);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9789);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16876);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 16877);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16878);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 16879);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 28316);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 28317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16880);
					match s {
						[] => true,
						['х', 'х', s @ ..] => {
							super::push(out, 23590);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28318);
									match s {
										[] => true,
										['р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 31043);
											s.is_empty()
										}
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
					super::push(out, 16881);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16882);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16883);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16884);
					match s {
						[] => true,
						['ф', '\u{1}', s @ ..] => {
							super::push(out, 23593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 16885);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23594);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28319);
									s.is_empty()
								}
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
			super::push(out, 9790);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 9791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -систый.
pub(crate) fn n4424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4262);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9792);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16886);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16887);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 23595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9793);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16888);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23596);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16889);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9794);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16891);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28320);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 28321);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 28322);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31044);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31045);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 32453);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16892);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16893);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23599);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28323);
									s.is_empty()
								}
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
			super::push(out, 9795);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 9796);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 9797);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16894);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тистый.
pub(crate) fn n4425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9798);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16895);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16896);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16897);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 16898);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9799);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16899);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23602);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 23603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9800);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16900);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 23604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16901);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16902);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 16903);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 23605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 16904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9801);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16906);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 28324);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 28325);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 28326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9802);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9803);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 9804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чистый.
pub(crate) fn n4427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4265);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9805);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9806);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16909);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 16910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16911);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23608);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28327);
									s.is_empty()
								}
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
			super::push(out, 9807);
			match s {
				[] => true,
				['л', 'з', s @ ..] => {
					super::push(out, 16912);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шистый.
pub(crate) fn n4428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4266);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9808);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23610);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23611);
							match s {
								[] => true,
								['а', 'р', 'е', s @ ..] => {
									super::push(out, 28328);
									match s {
										[] => true,
										['н', s @ ..] => {
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
				['п', s @ ..] => {
					super::push(out, 16914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9810);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16916);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9811);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 9812);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 9813);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вшийся.
pub(crate) fn n4523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4359);
	match s {
		[] => true,
		['а', s @ ..] => super::t10::n10391(s, out),
		['е', s @ ..] => {
			super::push(out, 9916);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17017);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23756);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 28471);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 28472);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23757);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 23758);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 28474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17018);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17019);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17020);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n10393(s, out),
		['о', s @ ..] => {
			super::push(out, 9918);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 9919);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17038);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17039);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23883);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 23884);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 23885);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28699);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28700);
									s.is_empty()
								}
								['и', 'т', s @ ..] => {
									super::push(out, 28701);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31348);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28702);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 28703);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31349);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 23886);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23887);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28704);
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32609);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 32610);
													match s {
														[] => true,
														['в', s @ ..] => super::t10::n34874(s, out),
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
									super::push(out, 28705);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 28706);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31350);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32611);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => super::t10::n25110(s, out),
						['о', s @ ..] => {
							super::push(out, 23889);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23890);
							match s {
								[] => true,
								['а', s @ ..] => super::t10::n30153(s, out),
								['и', s @ ..] => {
									super::push(out, 28716);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t10::n25113(s, out),
						['с', s @ ..] => super::t10::n25114(s, out),
						['т', s @ ..] => super::t10::n25115(s, out),
						['у', s @ ..] => super::t10::n25116(s, out),
						['х', s @ ..] => super::t10::n25117(s, out),
						['ч', s @ ..] => super::t10::n25118(s, out),
						['ь', s @ ..] => super::t10::n25119(s, out),
						['я', s @ ..] => super::t10::n25120(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t10::n10396(s, out),
		['я', s @ ..] => super::t10::n10397(s, out),
		_ => false,
	}
}

/// Words ending -ющийся.
pub(crate) fn n4535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4371);
	match s {
		[] => true,
		['а', s @ ..] => super::t10::n10423(s, out),
		['е', s @ ..] => {
			super::push(out, 9944);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17103);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24007);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17104);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9945);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9946);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9947);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9948);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24010);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 24011);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 28814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 17109);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17110);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17111);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24014);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28815);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 28816);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31430);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 28817);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31431);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24015);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17112);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24017);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24018);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 24019);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 28819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24020);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17114);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24022);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									super::push(out, 28820);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 28821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24023);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 28822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17115);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17116);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17117);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28823);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 28824);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31432);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31433);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32651);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 28825);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 28826);
									match s {
										[] => true,
										['р', 'о', 'ф', '\u{1}', s @ ..] => {
											super::push(out, 31434);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => super::t10::n30271(s, out),
								['п', s @ ..] => super::t10::n30272(s, out),
								['р', s @ ..] => super::t10::n30273(s, out),
								['с', s @ ..] => super::t10::n30274(s, out),
								['т', s @ ..] => super::t10::n30275(s, out),
								['ц', s @ ..] => super::t10::n30276(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t10::n25254(s, out),
						['ф', s @ ..] => super::t10::n25255(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t10::n18016(s, out),
				['т', s @ ..] => super::t10::n18017(s, out),
				['х', s @ ..] => super::t10::n18018(s, out),
				['ч', s @ ..] => super::t10::n18019(s, out),
				['ш', s @ ..] => super::t10::n18020(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t10::n10429(s, out),
		['ю', s @ ..] => super::t10::n10430(s, out),
		['я', s @ ..] => super::t10::n10431(s, out),
		_ => false,
	}
}

/// Words ending -ящийся.
pub(crate) fn n4536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9952);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 9953);
			match s {
				[] => true,
				['о', 'р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24039);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17129);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17130);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 17131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9954);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17132);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24043);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 24044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17133);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17134);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24046);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24047);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24048);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28840);
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
							super::push(out, 24049);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24050);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24051);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17135);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9955);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17136);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'о', s @ ..] => {
					super::push(out, 17137);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17138);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17139);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', 'о', s @ ..] => {
									super::push(out, 28844);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 28845);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28846);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 24056);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24057);
							s.is_empty()
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									super::push(out, 28848);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17140);
					s.is_empty()
				}
				['у', 'с', s @ ..] => {
					super::push(out, 17141);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17142);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 17143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9956);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9957);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17145);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17146);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17147);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24059);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 24060);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 28850);
									s.is_empty()
								}
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
			super::push(out, 9958);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28851);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 28852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31443);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31444);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 24061);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28853);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 24062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17148);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 24063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17149);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17150);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24064);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24065);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24066);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 24067);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17151);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17152);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24068);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17153);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t10::n18056(s, out),
				['с', s @ ..] => super::t10::n18057(s, out),
				['у', s @ ..] => super::t10::n18058(s, out),
				['ы', s @ ..] => super::t10::n18059(s, out),
				['я', s @ ..] => super::t10::n18060(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t10::n10439(s, out),
		['н', s @ ..] => super::t10::n10440(s, out),
		['о', s @ ..] => super::t10::n10441(s, out),
		['п', s @ ..] => super::t10::n10442(s, out),
		['р', s @ ..] => super::t10::n10443(s, out),
		['с', s @ ..] => super::t10::n10444(s, out),
		['т', s @ ..] => super::t10::n10445(s, out),
		['у', s @ ..] => super::t10::n10446(s, out),
		_ => false,
	}
}

/// The word прадедовский.
pub(crate) fn n34270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32653);
	s.is_empty()
}

/// Words ending -апрадедовский.
pub(crate) fn n34271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32654);
	s.is_empty()
}

/// Words ending -ковский.
pub(crate) fn n4680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4503);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10103);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17283);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 17284);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17285);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17286);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17287);
					match s {
						[] => true,
						['р', 'о', 'к', '\u{1}', s @ ..] => {
							super::push(out, 24175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24176);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24177);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17289);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 17290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10104);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 17291);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10105);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10106);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17292);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17293);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 24180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17294);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24181);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17295);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24182);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 24183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 17296);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10107);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10108);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 10109);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17298);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17299);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24185);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17300);
					match s {
						[] => true,
						['в', 'т', '\u{1}', s @ ..] => {
							super::push(out, 24186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10110);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17301);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17302);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 24187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10111);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17303);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17304);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17305);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17306);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17307);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10112);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 10113);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10114);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 10115);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 17309);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловский.
pub(crate) fn n4681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4504);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10116);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10117);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10118);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 10119);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10120);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17312);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17313);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10121);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10122);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 10123);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10124);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17315);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 17316);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 24193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10125);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17317);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17318);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10126);
			match s {
				[] => true,
				['о', 'г', s @ ..] => {
					super::push(out, 17320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10127);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10128);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10129);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 10130);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 17323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омовский.
pub(crate) fn n10619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10131);
	match s {
		[] => true,
		['д', 'о', 'л', 'е', 'б', '-', 'о', 'к', 'с', 'в', 'е', 'л', 'м', 'е', 'р', 'к', '\u{1}', s @ ..] => {
			super::push(out, 17324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рмовский.
pub(crate) fn n10620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10132);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17325);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24194);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямовский.
pub(crate) fn n10621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10133);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17326);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новский.
pub(crate) fn n4683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4506);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10134);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17327);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17328);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17329);
					match s {
						[] => true,
						['е', 'р', '\u{1}', s @ ..] => {
							super::push(out, 24196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17330);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17331);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24198);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24199);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 24200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10135);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17332);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17333);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24201);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17334);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24202);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28898);
									match s {
										[] => true,
										['д', 'н', 'а', '\u{1}', s @ ..] => {
											super::push(out, 31447);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10136);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17335);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17336);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17337);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17338);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10137);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10138);
			match s {
				[] => true,
				['д', 'н', s @ ..] => {
					super::push(out, 17339);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17340);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 17341);
					s.is_empty()
				}
				['к', 'и', s @ ..] => {
					super::push(out, 17342);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17343);
					s.is_empty()
				}
				['м', 'и', s @ ..] => {
					super::push(out, 17344);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17345);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17346);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17347);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24208);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28899);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24209);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24210);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17348);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24211);
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28901);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 28902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 24212);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 24213);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 28903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17349);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10139);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 10140);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17350);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10141);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17351);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10142);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17353);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17354);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17355);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10143);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17357);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 17358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10144);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17359);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 17360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -повский.
pub(crate) fn n4684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4507);
	match s {
		[] => true,
		['е', 'д', '\u{1}', s @ ..] => {
			super::push(out, 10146);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10147);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17361);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровский.
pub(crate) fn n4685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4508);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10149);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17363);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24220);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28904);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 24221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17364);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17365);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17367);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10150);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10151);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10152);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24225);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24226);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24227);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 24228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', 'и', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24229);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 24230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 24231);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24232);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17369);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17370);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24234);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28906);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24237);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24238);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 24239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17371);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17372);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 24240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17373);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 17374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10153);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10154);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17375);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10155);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17377);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17378);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 24242);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28908);
									s.is_empty()
								}
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
			super::push(out, 10156);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10157);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17380);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17381);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17382);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17383);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17384);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24247);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10158);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 10159);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -совский.
pub(crate) fn n4686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4509);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10160);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17385);
					match s {
						[] => true,
						['о', 'т', '\u{1}', s @ ..] => {
							super::push(out, 24249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10161);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17386);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24250);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17387);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24251);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24252);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 28911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17388);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 17389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10162);
			s.is_empty()
		}
		['к', 'и', s @ ..] => {
			super::push(out, 10163);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', s @ ..] => {
			super::push(out, 10164);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'у', s @ ..] => {
			super::push(out, 10165);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10166);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товский.
pub(crate) fn n4687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4510);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10168);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 10169);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17394);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 17395);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10170);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17396);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10171);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17398);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17399);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17400);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17401);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17402);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', s @ ..] => {
			super::push(out, 10173);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17404);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17406);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24259);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 24260);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17407);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24261);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 28913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 24262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17408);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 17409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10174);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17410);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 24263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17411);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24264);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28914);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17412);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17413);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17414);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 24267);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17415);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 17416);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10175);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10176);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17418);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фовский.
pub(crate) fn n4688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4511);
	s.is_empty()
}

/// Words ending -ховский.
pub(crate) fn n4689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4512);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17419);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10177);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10178);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10179);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 10180);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17422);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовский.
pub(crate) fn n4690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4513);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10181);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17423);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10182);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шовский.
pub(crate) fn n4691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4514);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10184);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -знический.
pub(crate) fn n18367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17449);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 24295);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 28930);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 24296);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инический.
pub(crate) fn n18368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17450);
	s.is_empty()
}

/// Words ending -йнический.
pub(crate) fn n18369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17451);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24297);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кнический.
pub(crate) fn n18370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17452);
	s.is_empty()
}

/// Words ending -мнический.
pub(crate) fn n18371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17453);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24298);
			s.is_empty()
		}
		['и', 'л', s @ ..] => {
			super::push(out, 24299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ннический.
pub(crate) fn n18372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17454);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28932);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31456);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28933);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28934);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24301);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28936);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28937);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28938);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28939);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 28940);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 28941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24302);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 24303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онический.
pub(crate) fn n18373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17455);
	s.is_empty()
}

/// Words ending -пнический.
pub(crate) fn n18374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17456);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 24304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнический.
pub(crate) fn n18375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17457);
	match s {
		[] => true,
		['е', 'ч', s @ ..] => {
			super::push(out, 24305);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24306);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 24307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -снический.
pub(crate) fn n18376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17458);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24308);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24309);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнический.
pub(crate) fn n18377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17459);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24310);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28945);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24311);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28947);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24312);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31459);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 31460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 28949);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 28950);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28951);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31462);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 31463);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 32661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 31464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32662);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 32663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 24313);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28953);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 24314);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 24315);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унический.
pub(crate) fn n18378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17460);
	s.is_empty()
}

/// Words ending -хнический.
pub(crate) fn n18379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17461);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24316);
			match s {
				[] => true,
				['т', '-', 'о', 'н', 'т', 'е', 'л', '\u{1}', s @ ..] => {
					super::push(out, 28955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнический.
pub(crate) fn n18380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17462);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24317);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28956);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28957);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шнический.
pub(crate) fn n18381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17463);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 24318);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щнический.
pub(crate) fn n18382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17464);
	s.is_empty()
}

/// Words ending -ьнический.
pub(crate) fn n18383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17465);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 24319);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пический.
pub(crate) fn n10701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10208);
	match s {
		[] => true,
		['о', 'к', 'с', 'о', 'д', s @ ..] => {
			super::push(out, 17466);
			match s {
				[] => true,
				['й', 'е', 'л', 'а', 'к', '\u{1}', s @ ..] => {
					super::push(out, 24320);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рический.
pub(crate) fn n10702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10209);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17467);
			match s {
				[] => true,
				['ф', 'с', s @ ..] => {
					super::push(out, 24321);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28959);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31467);
									s.is_empty()
								}
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
			super::push(out, 17468);
			match s {
				[] => true,
				['т', 'с', 'и', s @ ..] => {
					super::push(out, 24322);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28960);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31468);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32664);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сический.
pub(crate) fn n10703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10210);
	s.is_empty()
}

/// Words ending -тический.
pub(crate) fn n10704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10211);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17470);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17471);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 17472);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28961);
							match s {
								[] => true,
								['н', 'а', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 32665);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32666);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33237);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'д', s @ ..] => {
											super::push(out, 32667);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33238);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'а', 'п', '-', s @ ..] => {
									super::push(out, 31469);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32668);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'а', 'т', '-', 'о', 'н', 'т', 'е', 'л', '\u{1}', s @ ..] => {
			super::push(out, 17473);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17474);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 17475);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					super::push(out, 24324);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 28963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17476);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', 'а', 'н', s @ ..] => {
							super::push(out, 28964);
							match s {
								[] => true,
								['о', 'и', 'ц', 'а', 'н', 'р', s @ ..] => {
									super::push(out, 31470);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32669);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 33239);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'о', s @ ..] => {
							super::push(out, 28965);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31471);
									s.is_empty()
								}
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
			super::push(out, 17477);
			match s {
				[] => true,
				['о', 'п', '-', s @ ..] => {
					super::push(out, 24325);
					match s {
						[] => true,
						['о', 'н', 'д', s @ ..] => {
							super::push(out, 28966);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фический.
pub(crate) fn n10705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10212);
	s.is_empty()
}

/// Words ending -хический.
pub(crate) fn n10706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10213);
	s.is_empty()
}

/// Words ending -щический.
pub(crate) fn n10707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10214);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 17478);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24326);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -валинский.
pub(crate) fn n18491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17572);
	s.is_empty()
}

/// Words ending -калинский.
pub(crate) fn n18492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17573);
	s.is_empty()
}

/// Words ending -талинский.
pub(crate) fn n18493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17574);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 24352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -халинский.
pub(crate) fn n18494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17575);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24353);
			match s {
				[] => true,
				['с', 'о', s @ ..] => {
					super::push(out, 28974);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31473);
							match s {
								[] => true,
								['ж', 'ю', '\u{1}', s @ ..] => {
									super::push(out, 32670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ублинский.
pub(crate) fn n10865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10359);
	s.is_empty()
}

/// Words ending -глинский.
pub(crate) fn n10866(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17576);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17577);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -злинский.
pub(crate) fn n10867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10360);
	s.is_empty()
}

/// Words ending -йлинский.
pub(crate) fn n10868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10361);
	s.is_empty()
}

/// Words ending -уклинский.
pub(crate) fn n10869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10362);
	s.is_empty()
}

/// Words ending -ллинский.
pub(crate) fn n10870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10363);
	s.is_empty()
}

/// Words ending -олинский.
pub(crate) fn n10871(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 17578);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 17579);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлинский.
pub(crate) fn n10872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10364);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слинский.
pub(crate) fn n10873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10365);
	s.is_empty()
}

/// Words ending -улинский.
pub(crate) fn n10874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10366);
	s.is_empty()
}

/// Words ending -минский.
pub(crate) fn n4814(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10367);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10368);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10369);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 10370);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нинский.
pub(crate) fn n4815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4629);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10371);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17581);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10372);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17583);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10373);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 10374);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'е', s @ ..] => {
			super::push(out, 10375);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10376);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10377);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оинский.
pub(crate) fn n4816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4630);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пинский.
pub(crate) fn n4817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4631);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10379);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10380);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 10381);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10382);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 10383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ринский.
pub(crate) fn n4818(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17590);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17591);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24356);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24357);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17593);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10384);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10385);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10386);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17595);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 17596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -синский.
pub(crate) fn n4819(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 10388);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 10389);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 10390);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 10391);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тинский.
pub(crate) fn n4820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4632);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17597);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24360);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10392);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17598);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10393);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10394);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10395);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 10396);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24363);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 24364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10397);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10398);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17602);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', 'у', s @ ..] => {
			super::push(out, 10399);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уинский.
pub(crate) fn n4821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4633);
	s.is_empty()
}

/// Words ending -финский.
pub(crate) fn n4822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4634);
	s.is_empty()
}

/// Words ending -хинский.
pub(crate) fn n4823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4635);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10400);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10401);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10402);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17606);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цинский.
pub(crate) fn n4824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10403);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10404);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10405);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10406);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17608);
					match s {
						[] => true,
						['е', '-', 'о', 'к', 'с', 'в', 'е', 'ч', 'а', 'б', 'р', 'о', 'г', '\u{1}', s @ ..] => {
							super::push(out, 24367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чинский.
pub(crate) fn n4825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4637);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10407);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17609);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17611);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10408);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шинский.
pub(crate) fn n4826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4638);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10409);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 10410);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10411);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'т', 'е', s @ ..] => {
			super::push(out, 10413);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10414);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щинский.
pub(crate) fn n4827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4639);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 10415);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьинский.
pub(crate) fn n4828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4640);
	s.is_empty()
}

/// Words ending -перский.
pub(crate) fn n4898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4702);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10518);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10519);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 10520);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10521);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10522);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 17692);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17693);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10523);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17694);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рерский.
pub(crate) fn n4899(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10524);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 17695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -серский.
pub(crate) fn n4900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4703);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 10526);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'о', 'б', 'к', s @ ..] => {
			super::push(out, 10527);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 10528);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17697);
					match s {
						[] => true,
						['ж', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 28978);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31474);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28979);
									s.is_empty()
								}
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
			super::push(out, 10529);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17698);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терский.
pub(crate) fn n4901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4704);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10530);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10531);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10532);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17700);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10533);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10534);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10535);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17702);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 17703);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10536);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17704);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17705);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17706);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17707);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', s @ ..] => {
			super::push(out, 10537);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10538);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10539);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17711);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24385);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 24386);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 24387);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24388);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 17712);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17713);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24389);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 28981);
									s.is_empty()
								}
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
			super::push(out, 10540);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17714);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10541);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ферский.
pub(crate) fn n4902(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10544);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10545);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 10546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -херский.
pub(crate) fn n4903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4705);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10547);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -церский.
pub(crate) fn n4904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4706);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10548);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -черский.
pub(crate) fn n4905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4707);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10549);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10550);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шерский.
pub(crate) fn n4906(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10551);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 10552);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10553);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 10554);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аленький.
pub(crate) fn n11234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10707);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 17834);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 17835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24452);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29000);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иленький.
pub(crate) fn n11235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10708);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 17836);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 17837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24453);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упленький.
pub(crate) fn n11236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10709);
	match s {
		[] => true,
		['щ', s @ ..] => {
			super::push(out, 17838);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -менький.
pub(crate) fn n5035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4829);
	s.is_empty()
}

/// Words ending -ненький.
pub(crate) fn n5036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4830);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10710);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17839);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17840);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10711);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10712);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 10713);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17841);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17842);
					match s {
						[] => true,
						['р', 'с', '\u{1}', s @ ..] => {
							super::push(out, 24455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17843);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24456);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17844);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17845);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24457);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 31485);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 31486);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 24458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17846);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10714);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 10715);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10716);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 10717);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10718);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10719);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17848);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17849);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24461);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24462);
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
				['а', s @ ..] => {
					super::push(out, 17850);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24463);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24464);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 29004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17851);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 24465);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24466);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17852);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 17853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10720);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10721);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 24467);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 24468);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17855);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 17856);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10722);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17857);
					s.is_empty()
				}
				['у', 'к', 'в', 'е', s @ ..] => {
					super::push(out, 17858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10723);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17859);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24470);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 10724);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24471);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 24472);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29008);
									s.is_empty()
								}
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
			super::push(out, 10725);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 10726);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10727);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17862);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 29009);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 29010);
									s.is_empty()
								}
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
			super::push(out, 10728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пенький.
pub(crate) fn n5037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4831);
	match s {
		[] => true,
		['у', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17863);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17864);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренький.
pub(crate) fn n5038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4832);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10729);
			s.is_empty()
		}
		['б', 'о', 'д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17865);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10730);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10731);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10732);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10733);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17868);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 24473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24474);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29011);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24475);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24476);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 29013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 24477);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 29014);
									s.is_empty()
								}
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
				['а', s @ ..] => {
					super::push(out, 17869);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17870);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сенький.
pub(crate) fn n5039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4833);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 10734);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 10735);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17871);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10736);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 10737);
			match s {
				[] => true,
				['л', 'а', 'м', '\u{1}', s @ ..] => {
					super::push(out, 17873);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17874);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенький.
pub(crate) fn n5040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10738);
			match s {
				[] => true,
				['в', 'о', 'р', s @ ..] => {
					super::push(out, 17875);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17876);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 17877);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17878);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 24481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24482);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 24483);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29015);
									s.is_empty()
								}
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
			super::push(out, 10739);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					super::push(out, 17879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10740);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24485);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24486);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24487);
							s.is_empty()
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29016);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29017);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 29018);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 29019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 24488);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 24489);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24490);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24491);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 24492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 17880);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17881);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17882);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24494);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 24495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10741);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17883);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хенький.
pub(crate) fn n5041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4835);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10742);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17884);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченький.
pub(crate) fn n5042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4836);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 10743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шенький.
pub(crate) fn n5043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4837);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10745);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10746);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17886);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17887);
					match s {
						[] => true,
						['о', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29020);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29021);
									s.is_empty()
								}
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
			super::push(out, 10747);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щенький.
pub(crate) fn n5044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4838);
	s.is_empty()
}

/// Words ending -ивавший.
pub(crate) fn n5179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4962);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10848);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 10849);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10850);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 10851);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10852);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 10853);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17948);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 24519);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24521);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 17949);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17950);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24524);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29034);
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
						['л', s @ ..] => {
							super::push(out, 24525);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24526);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29035);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29036);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31489);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 24527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17951);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24528);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24530);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24532);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 24533);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24534);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 24535);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24536);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 24537);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24538);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17953);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17954);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17955);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17956);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17957);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24541);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 17958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10854);
			match s {
				[] => true,
				['а', 'з', s @ ..] => {
					super::push(out, 17959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17960);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17961);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 24544);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 24545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17962);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10855);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17964);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24546);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29040);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 24548);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 24549);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24550);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 17965);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24551);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 24552);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24553);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 29042);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31490);
											s.is_empty()
										}
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
					super::push(out, 17966);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17967);
					match s {
						[] => true,
						['о', s @ ..] => super::t10::n25805(s, out),
						_ => false,
					}
				}
				['е', 'р', s @ ..] => super::t10::n18902(s, out),
				['з', s @ ..] => super::t10::n18903(s, out),
				['и', s @ ..] => super::t10::n18904(s, out),
				['м', s @ ..] => {
					super::push(out, 17970);
					s.is_empty()
				}
				['о', s @ ..] => super::t10::n18906(s, out),
				['п', s @ ..] => super::t10::n18907(s, out),
				['с', 'а', s @ ..] => super::t10::n18908(s, out),
				['т', s @ ..] => super::t10::n18909(s, out),
				['у', s @ ..] => super::t10::n18910(s, out),
				['ы', s @ ..] => super::t10::n18911(s, out),
				['я', s @ ..] => super::t10::n18912(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t10::n11392(s, out),
		['п', s @ ..] => super::t10::n11393(s, out),
		['р', s @ ..] => super::t10::n11394(s, out),
		['х', s @ ..] => super::t10::n11395(s, out),
		['ч', s @ ..] => super::t10::n11396(s, out),
		['ш', s @ ..] => super::t10::n11397(s, out),
		['щ', s @ ..] => super::t10::n11398(s, out),
		_ => false,
	}
}

/// Words ending -овавший.
pub(crate) fn n5180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4963);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18011);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24604);
							match s {
								[] => true,
								['т', 'ы', s @ ..] => {
									super::push(out, 29069);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31498);
											s.is_empty()
										}
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
					super::push(out, 18012);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24605);
							match s {
								[] => true,
								['р', 'т', 'ы', s @ ..] => {
									super::push(out, 29070);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31499);
											s.is_empty()
										}
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
					super::push(out, 18013);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18014);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 18015);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10863);
			match s {
				[] => true,
				['т', s @ ..] => super::t10::n18951(s, out),
				['х', s @ ..] => {
					super::push(out, 18017);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24611);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29084);
									s.is_empty()
								}
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
			super::push(out, 10864);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18018);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24612);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24613);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 29086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18019);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 18020);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 18021);
					match s {
						[] => true,
						['о', 'т', 'ы', s @ ..] => {
							super::push(out, 24614);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18023);
					s.is_empty()
				}
				['е', s @ ..] => super::t10::n18959(s, out),
				['ж', s @ ..] => {
					super::push(out, 18025);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18026);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 18027);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t10::n18963(s, out),
				['о', s @ ..] => super::t10::n18964(s, out),
				['р', s @ ..] => super::t10::n18965(s, out),
				['у', s @ ..] => super::t10::n18966(s, out),
				['я', s @ ..] => super::t10::n18967(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t10::n11403(s, out),
		['к', s @ ..] => super::t10::n11404(s, out),
		['л', s @ ..] => super::t10::n11405(s, out),
		['м', s @ ..] => super::t10::n11406(s, out),
		['н', s @ ..] => super::t10::n11407(s, out),
		['п', s @ ..] => super::t10::n11408(s, out),
		['р', s @ ..] => super::t10::n11409(s, out),
		['с', s @ ..] => super::t10::n11410(s, out),
		['т', s @ ..] => super::t10::n11411(s, out),
		['ф', s @ ..] => super::t10::n11412(s, out),
		['х', s @ ..] => super::t10::n11413(s, out),
		_ => false,
	}
}

/// Words ending -ывавший.
pub(crate) fn n5183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4966);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10880);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18084);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18085);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24691);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24692);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 29178);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31589);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 29179);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 29180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18086);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18087);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18088);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18089);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18090);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18091);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18092);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 18093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10881);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18094);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 18095);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24698);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18096);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18097);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18098);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29182);
									s.is_empty()
								}
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
			super::push(out, 10882);
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18099);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24701);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 24702);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 24703);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 18100);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 18101);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18102);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24706);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18103);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24707);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24708);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24709);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18104);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24710);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18105);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18106);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18107);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 18108);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 18109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10883);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18110);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18111);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18112);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 18113);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18114);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18115);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 24716);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24717);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 18116);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 18117);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 24720);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24721);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24722);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 24723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18119);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18120);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18121);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24725);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 24726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18123);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10885);
			match s {
				[] => true,
				['а', s @ ..] => super::t10::n19062(s, out),
				['е', s @ ..] => super::t10::n19063(s, out),
				['з', s @ ..] => super::t10::n19064(s, out),
				['у', s @ ..] => super::t10::n19065(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t10::n11424(s, out),
		['р', s @ ..] => super::t10::n11425(s, out),
		['с', s @ ..] => super::t10::n11426(s, out),
		['т', s @ ..] => super::t10::n11427(s, out),
		_ => false,
	}
}

/// Words ending -лкавший.
pub(crate) fn n5227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5008);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10977);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10978);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 18251);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 24814);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29217);
									s.is_empty()
								}
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
			super::push(out, 10979);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18252);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18253);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 24815);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окавший.
pub(crate) fn n5229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5010);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10980);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10981);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10982);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18254);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18255);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 18256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10983);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 10984);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 10985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ркавший.
pub(crate) fn n5230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5011);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10986);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18257);
					s.is_empty()
				}
				['х', 'ы', s @ ..] => {
					super::push(out, 18258);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'ы', s @ ..] => {
					super::push(out, 18259);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10987);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18260);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24818);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 18261);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24820);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24821);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29220);
									s.is_empty()
								}
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
			super::push(out, 10988);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10989);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18262);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24822);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29221);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29222);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31595);
											s.is_empty()
										}
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
					super::push(out, 18263);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 18264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10990);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 10991);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18265);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29223);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 29224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 18266);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скавший.
pub(crate) fn n5231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5012);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10992);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18267);
					s.is_empty()
				}
				['т', 'ы', s @ ..] => {
					super::push(out, 18268);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10993);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18269);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29225);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29226);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31596);
											s.is_empty()
										}
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
					super::push(out, 18270);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 24824);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29227);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29228);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31597);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18271);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18272);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18273);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18274);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18275);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18276);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18277);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10995);
			match s {
				[] => true,
				['л', 'о', 'п', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24829);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24830);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29229);
									s.is_empty()
								}
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
			super::push(out, 10996);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10997);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10998);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18280);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 18281);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18282);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 18283);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24833);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 29230);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31598);
											s.is_empty()
										}
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
					super::push(out, 18284);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18285);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10999);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ткавший.
pub(crate) fn n5232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5013);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11000);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11001);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укавший.
pub(crate) fn n5233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5014);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11002);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18288);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11003);
			match s {
				[] => true,
				['с', 'ы', s @ ..] => {
					super::push(out, 18289);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 11004);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 11005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чкавший.
pub(crate) fn n5234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5015);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11006);
			match s {
				[] => true,
				['п', 'ы', s @ ..] => {
					super::push(out, 18290);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11007);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шкавший.
pub(crate) fn n5235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5016);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкавший.
pub(crate) fn n5236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5017);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18291);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18292);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 18293);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 18294);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11010);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11011);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 18297);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 18298);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 18299);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18300);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18301);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 18302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11012);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 18303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'б', s @ ..] => {
			super::push(out, 11013);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 18304);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 18305);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 18306);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 18307);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24840);
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 24841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24842);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18308);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					super::push(out, 18309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 11014);
			s.is_empty()
		}
		_ => false,
	}
}

