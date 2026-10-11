use flexion::Path;

/// Words ending -ужаешь.
pub(crate) fn n592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 589);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2890);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8324);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2891);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8325);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2892);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8326);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8327);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8328);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8329);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2893);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8331);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8332);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22940);
									s.is_empty()
								}
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
			super::push(out, 2894);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 2895);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8333);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -южаешь.
pub(crate) fn n594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2896);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8334);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжаешь.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рзаешь.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 599);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8365);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8366);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8367);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8368);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15811);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22944);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28161);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31330);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 15812);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 15813);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15814);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15815);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15816);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15817);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15818);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 15819);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8370);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15821);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15823);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 8372);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15825);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 15826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2907);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скользаешь.
pub(crate) fn n2968(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8374);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8375);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8376);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юзаешь.
pub(crate) fn n606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2910);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2911);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2912);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8378);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язаешь.
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2913);
			s.is_empty()
		}
		['р', 'г', s @ ..] => {
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 8379);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2914);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2915);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8382);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8383);
					match s {
						[] => true,
						['и', 'о', s @ ..] => {
							super::push(out, 15830);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огавкаешь.
pub(crate) fn n16726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15850);
	s.is_empty()
}

/// Words ending -кавкаешь.
pub(crate) fn n8792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8408);
	s.is_empty()
}

/// Words ending -чавкаешь.
pub(crate) fn n8793(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15851);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15852);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15853);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 15854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явкаешь.
pub(crate) fn n2989(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15855);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8409);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15857);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15858);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -екаешь.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2929);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2930);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2931);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2932);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2933);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2934);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8414);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8415);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8416);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 15861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8417);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15862);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8418);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15864);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8419);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15865);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22955);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22956);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22957);
									s.is_empty()
								}
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
			super::push(out, 2935);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2936);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2937);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8420);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15866);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 15867);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22958);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31331);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 31332);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 31333);
													s.is_empty()
												}
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
							super::push(out, 15868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8421);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8422);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8423);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 15869);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22959);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28164);
											s.is_empty()
										}
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
					super::push(out, 8424);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8425);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22960);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 28165);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 22961);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28166);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 22962);
									s.is_empty()
								}
								['о', 'п', s @ ..] => {
									super::push(out, 22963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8426);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15870);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22964);
									s.is_empty()
								}
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
			super::push(out, 2938);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8427);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15871);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 15872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8428);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8429);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8430);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 8431);
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 8432);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8433);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8434);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15874);
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22965);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					super::push(out, 8435);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8436);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2940);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8438);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 15875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 2941);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 2942);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8440);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8441);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2943);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -икаешь.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2944);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2945);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2946);
			match s {
				[] => true,
				['и', 'ч', 'а', s @ ..] => {
					super::push(out, 8444);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8445);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2948);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2949);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2950);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2951);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8447);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15879);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 15880);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22968);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22969);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15881);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 15882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22971);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15883);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15884);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 22973);
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									super::push(out, 22974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 15885);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15886);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15887);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15888);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8449);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15890);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22975);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28167);
											s.is_empty()
										}
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
					super::push(out, 8450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2952);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8451);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8452);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15891);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15892);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 22976);
									s.is_empty()
								}
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
			super::push(out, 2953);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8454);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8455);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8456);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2955);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8458);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15895);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 22977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 15896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22978);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22979);
									s.is_empty()
								}
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
			super::push(out, 2956);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8459);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8460);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8461);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2957);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8463);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8464);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8465);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 8466);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15897);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8467);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22980);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22981);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8468);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 2959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8469);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8470);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8471);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8472);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8473);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8474);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 8475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 2961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йкаешь.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2962);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15902);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15904);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2963);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2964);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкаешь.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2965);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2966);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 8481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15906);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15907);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15908);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15909);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22983);
									s.is_empty()
								}
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
			super::push(out, 2967);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8482);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15910);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15911);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 15912);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15913);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15914);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22984);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 22985);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28168);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15915);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мкаешь.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2968);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8484);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 8485);
					s.is_empty()
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15916);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8486);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 8487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15918);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15919);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2969);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15921);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15922);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2970);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8488);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 8489);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8490);
					match s {
						[] => true,
						['х', '\u{1}', s @ ..] => {
							super::push(out, 15924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15925);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2971);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нкаешь.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2972);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окаешь.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 614);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2973);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2974);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2975);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 8494);
					match s {
						[] => true,
						['а', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22987);
									s.is_empty()
								}
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
				['н', s @ ..] => {
					super::push(out, 8495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8496);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8497);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2977);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8498);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8499);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2978);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8500);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8501);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8502);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8503);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15930);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8504);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 15932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8505);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15933);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22989);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 8506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15935);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15936);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2979);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2980);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 8508);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15938);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 8509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2981);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 8510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 2982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8511);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8512);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8513);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8515);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ркаешь.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8516);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 8517);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 15940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 8518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15941);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15942);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8519);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15943);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15944);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15946);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15947);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 15948);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15949);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 8521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15950);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15951);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15952);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15953);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22993);
									s.is_empty()
								}
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
			super::push(out, 2986);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8522);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15954);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 22994);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28169);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 28170);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22995);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22996);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8523);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15957);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15958);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 8524);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15960);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15961);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15962);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15963);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23000);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15964);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15965);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23002);
									s.is_empty()
								}
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
			super::push(out, 2987);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8525);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 15966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8526);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 8527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 8528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15969);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2988);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 8530);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15972);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8531);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23004);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23005);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23006);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28171);
											s.is_empty()
										}
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
					super::push(out, 8532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15974);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15975);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15976);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8533);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 8534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15978);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15979);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2989);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8535);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8536);
					s.is_empty()
				}
				['г', s @ ..] => super::t7::n8928(s, out),
				['т', s @ ..] => super::t7::n8929(s, out),
				['ш', s @ ..] => super::t7::n8930(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t7::n3051(s, out),
		['ю', s @ ..] => super::t7::n3052(s, out),
		_ => false,
	}
}

/// Words ending -скаешь.
pub(crate) fn n619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2992);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15990);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 15991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15992);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15993);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15994);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15995);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15996);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23013);
									s.is_empty()
								}
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
			super::push(out, 2993);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8547);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15997);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23014);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23015);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 28174);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 23016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8548);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23017);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23018);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28175);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 23019);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23020);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28176);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2994);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8549);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15999);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8550);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16002);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16003);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23022);
									s.is_empty()
								}
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
			super::push(out, 2995);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2996);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8554);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23023);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23024);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 28177);
											s.is_empty()
										}
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
					super::push(out, 8555);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2997);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8556);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 8557);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 16005);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16006);
							match s {
								[] => true,
								['и', 'е', s @ ..] => {
									super::push(out, 23025);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 28178);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 31336);
													s.is_empty()
												}
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
							super::push(out, 16007);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 23026);
									match s {
										[] => true,
										['п', s @ ..] => {
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
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2998);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8558);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16008);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8559);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16009);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 16010);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23028);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23029);
									match s {
										[] => true,
										['в', s @ ..] => {
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
				['ф', s @ ..] => {
					super::push(out, 8560);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 16011);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 23030);
									s.is_empty()
								}
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
			super::push(out, 2999);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8561);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3000);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8562);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16013);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 16014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8563);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 16015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ткаешь.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 617);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3001);
			match s {
				[] => true,
				['ф', '\u{1}', s @ ..] => {
					super::push(out, 8564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укаешь.
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 618);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3002);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8565);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8566);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3004);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8568);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8569);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8570);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 16017);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 23032);
									s.is_empty()
								}
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
			super::push(out, 3005);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8571);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8572);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8573);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 16020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3006);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3007);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8575);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 16021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3008);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8576);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3009);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3010);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3011);
			match s {
				[] => true,
				['п', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16023);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8577);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16026);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16027);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16028);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16029);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23033);
									s.is_empty()
								}
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
			super::push(out, 3013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8580);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8581);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8582);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3014);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8584);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3015);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8585);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8586);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23034);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23035);
									s.is_empty()
								}
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
			super::push(out, 3016);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16032);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16033);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цкаешь.
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 619);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3017);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3018);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 8588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чкаешь.
pub(crate) fn n623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 620);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3019);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16035);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16036);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16037);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16038);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16039);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16040);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23036);
									s.is_empty()
								}
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
			super::push(out, 3020);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16041);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкаешь.
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 621);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3021);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16043);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8590);
					match s {
						[] => true,
						['е', 'т', '\u{1}', s @ ..] => {
							super::push(out, 16045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3022);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3023);
			match s {
				[] => true,
				['к', 'а', s @ ..] => {
					super::push(out, 8592);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 3024);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыкаешь.
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3025);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3026);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16047);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 16048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8596);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8597);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8598);
					match s {
						[] => true,
						['о', 'и', s @ ..] => {
							super::push(out, 16049);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8599);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8600);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 16050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3028);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 8602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3029);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8603);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 16051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8604);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 16052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8605);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							super::push(out, 16053);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23038);
									s.is_empty()
								}
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
			super::push(out, 3030);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3031);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3032);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8606);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8607);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 23039);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 23040);
									s.is_empty()
								}
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
			super::push(out, 3033);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8608);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8609);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 8610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16056);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16057);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8611);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8612);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23042);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23043);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28181);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 16058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8613);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8614);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16060);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 16061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8615);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8616);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 16062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8617);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 8618);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16063);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23044);
									s.is_empty()
								}
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
			super::push(out, 3034);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8619);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23045);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3035);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8621);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16065);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16066);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8623);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 16068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3036);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8624);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8625);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16069);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 16070);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8626);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 16071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16072);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16073);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8628);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 16075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8629);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8630);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8631);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 16078);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 16079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8632);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 8633);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16080);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => super::t7::n9029(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t7::n3098(s, out),
		['х', s @ ..] => super::t7::n3099(s, out),
		['ц', s @ ..] => super::t7::n3100(s, out),
		_ => false,
	}
}

/// Words ending -ькаешь.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3040);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 8639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3041);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8640);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16081);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23049);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23050);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8641);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16082);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23052);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 16083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3042);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8642);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 16084);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 23054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 16085);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 23055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8643);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16086);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 16087);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28182);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 28183);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28184);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 23056);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28185);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 28186);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 16088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8644);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16089);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 23057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 16090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8645);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 8646);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3043);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8647);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16094);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экаешь.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3044);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3045);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юкаешь.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3047);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16096);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 16097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3048);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16098);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23058);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 23059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 16099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16100);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 16101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3049);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8650);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 8651);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8652);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 8653);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23060);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 23061);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 28188);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 31337);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ю', s @ ..] => {
									super::push(out, 23062);
									s.is_empty()
								}
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
			super::push(out, 3050);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8654);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8656);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 8657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16103);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16104);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3051);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 8658);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23063);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23064);
									s.is_empty()
								}
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
			super::push(out, 3052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8659);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8660);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16106);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 16107);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23065);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28189);
											s.is_empty()
										}
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
					super::push(out, 8661);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8662);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 8663);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3053);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 8664);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -якаешь.
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3054);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3055);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8665);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16110);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16111);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3056);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3057);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8668);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16113);
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
							super::push(out, 16114);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23067);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23068);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8669);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 8670);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3058);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8671);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8672);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8673);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 8674);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8675);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16117);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 16118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8676);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8677);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8678);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8679);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 8680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3059);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3060);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16120);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16121);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23070);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 23071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16122);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16123);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 16124);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16125);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8682);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16126);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16127);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3061);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8683);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							super::push(out, 16129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3062);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3063);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16130);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 16131);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 16132);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -думаешь.
pub(crate) fn n3154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3093);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8726);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 8727);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8728);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8729);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8730);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чумаешь.
pub(crate) fn n3155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3094);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8731);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16158);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дымаешь.
pub(crate) fn n3156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3095);
	s.is_empty()
}

/// Words ending -озапоминаешь.
pub(crate) fn n33221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31338);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 32807);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -авспоминаешь.
pub(crate) fn n33222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31339);
	s.is_empty()
}

/// Words ending -овспоминаешь.
pub(crate) fn n33223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31340);
	s.is_empty()
}

/// Words ending -апинаешь.
pub(crate) fn n9161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8758);
	s.is_empty()
}

/// Words ending -впинаешь.
pub(crate) fn n9162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8759);
	s.is_empty()
}

/// Words ending -опинаешь.
pub(crate) fn n9163(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 16167);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тпинаешь.
pub(crate) fn n9164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8760);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чинаешь.
pub(crate) fn n3175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3114);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8761);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8762);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16170);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йнаешь.
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['а', 'м', 'с', s @ ..] => {
			super::push(out, 3115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онаешь.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3116);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнаешь.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8764);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8765);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8766);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8767);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнаешь.
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['я', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8768);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8769);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8770);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унаешь.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окипаешь.
pub(crate) fn n9200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8796);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16187);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16188);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укипаешь.
pub(crate) fn n9202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8798);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16190);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word выкипаешь.
pub(crate) fn n9203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8799);
	s.is_empty()
}

/// Words ending -липаешь.
pub(crate) fn n3194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3131);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8800);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8801);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 8802);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8803);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8804);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 8806);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16193);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16194);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рипаешь.
pub(crate) fn n3195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3132);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 8807);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щипаешь.
pub(crate) fn n3196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3133);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8808);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 8809);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8810);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16196);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йпаешь.
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 662);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3134);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опаешь.
pub(crate) fn n666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 663);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 3135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8813);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8814);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8815);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8816);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8817);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8818);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8819);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8820);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 8821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16198);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16199);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16200);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16201);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23104);
									s.is_empty()
								}
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
			super::push(out, 3138);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8822);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 8823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16202);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16203);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3139);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8824);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8825);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8826);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 16205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8827);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 8828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рпаешь.
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 664);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3141);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16207);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 16208);
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
				['с', s @ ..] => {
					super::push(out, 8829);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 8830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16209);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16210);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16211);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16212);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23105);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 16213);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23107);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28196);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 16214);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16215);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упаешь.
pub(crate) fn n668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 665);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8831);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8832);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8833);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8834);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16216);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 16217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23109);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'и', s @ ..] => {
					super::push(out, 8835);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8836);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3143);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8837);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8838);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8839);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23111);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23112);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23113);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28197);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3144);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 8840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3145);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8841);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23114);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 23115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 16219);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							super::push(out, 16220);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 16221);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16222);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 16223);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16224);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23118);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 23119);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23120);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28198);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							super::push(out, 16225);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 16226);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 3146);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8842);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8843);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8844);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8845);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыпаешь.
pub(crate) fn n669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 666);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3147);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8846);
					match s {
						[] => true,
						['з', 'у', s @ ..] => {
							super::push(out, 16229);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 16230);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23122);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8847);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 8848);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8849);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							super::push(out, 16231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8850);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8851);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8852);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8853);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8854);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8855);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8856);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 16232);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юпаешь.
pub(crate) fn n670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 667);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3149);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					super::push(out, 8858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япаешь.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 668);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3150);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3151);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16233);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8860);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', '\u{1}', s @ ..] => {
			super::push(out, 3152);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыдираешь.
pub(crate) fn n9292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8884);
	match s {
		[] => true,
		['в', 'о', s @ ..] => {
			super::push(out, 16251);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жираешь.
pub(crate) fn n3229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3165);
	s.is_empty()
}

/// Words ending -зираешь.
pub(crate) fn n3230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3166);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8885);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8886);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8887);
			match s {
				[] => true,
				['р', 'п', 'а', s @ ..] => {
					super::push(out, 16253);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мираешь.
pub(crate) fn n3231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3167);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8889);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8890);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 16254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8891);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16255);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8892);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8893);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16258);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16259);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8894);
			match s {
				[] => true,
				['в', 'о', s @ ..] => {
					super::push(out, 16260);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пираешь.
pub(crate) fn n3232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3168);
	match s {
		[] => true,
		['а', 'н', s @ ..] => {
			super::push(out, 8895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'д', s @ ..] => {
			super::push(out, 8896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16262);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сираешь.
pub(crate) fn n3233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3169);
	s.is_empty()
}

/// Words ending -тираешь.
pub(crate) fn n3234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3170);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8897);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8898);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 8899);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8900);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8901);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8902);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23135);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 23136);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 16263);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16264);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 16265);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16266);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23138);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 23139);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23140);
							match s {
								[] => true,
								['п', 'с', s @ ..] => {
									super::push(out, 28199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16267);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16268);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 16269);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 16270);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8903);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йраешь.
pub(crate) fn n677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	s.is_empty()
}

/// Words ending -ораешь.
pub(crate) fn n678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3171);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8905);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16271);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 16272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3172);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16273);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3173);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8906);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 16275);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8907);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8908);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8909);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16277);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8910);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 16278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -блетаешь.
pub(crate) fn n9359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8945);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23158);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -длетаешь.
pub(crate) fn n9361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8947);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16314);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word перелетаешь.
pub(crate) fn n9362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8948);
	s.is_empty()
}

/// Words ending -злетаешь.
pub(crate) fn n9363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8949);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16315);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16316);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23159);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илетаешь.
pub(crate) fn n9364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8950);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 16317);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олетаешь.
pub(crate) fn n9365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8951);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 16318);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16319);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23160);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плетаешь.
pub(crate) fn n9366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8952);
	s.is_empty()
}

/// Words ending -слетаешь.
pub(crate) fn n9367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8953);
	s.is_empty()
}

/// Words ending -тлетаешь.
pub(crate) fn n9368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8954);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улетаешь.
pub(crate) fn n9369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8955);
	s.is_empty()
}

/// Words ending -ылетаешь.
pub(crate) fn n9370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8956);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16321);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23161);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -метаешь.
pub(crate) fn n3268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3204);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8957);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8958);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8959);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8960);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 8961);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8962);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8963);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 8964);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8965);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нетаешь.
pub(crate) fn n3269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3205);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 8966);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -петаешь.
pub(crate) fn n3270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3206);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8967);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретаешь.
pub(crate) fn n3271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3207);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -четаешь.
pub(crate) fn n3272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3208);
	s.is_empty()
}

/// Words ending -итаешь.
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3209);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3210);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8970);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8971);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8973);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3212);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8975);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8976);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8977);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8978);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23162);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23163);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 28203);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23164);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 23165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8979);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23166);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 23167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 16326);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16328);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16329);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16330);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16331);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8981);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8982);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16332);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтаешь.
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3213);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16333);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 16334);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16335);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16336);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отаешь.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 692);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3214);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8983);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8984);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23172);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23173);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23174);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23175);
									match s {
										[] => true,
										['в', s @ ..] => {
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
				['е', s @ ..] => {
					super::push(out, 8985);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3215);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3216);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16338);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16339);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16340);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23176);
									s.is_empty()
								}
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
			super::push(out, 3217);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8987);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8988);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8989);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8990);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8991);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3218);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8992);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3219);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8993);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23177);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23178);
									s.is_empty()
								}
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
			super::push(out, 3220);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8994);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3221);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8996);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртаешь.
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3222);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 8998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стаешь.
pub(crate) fn n697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3223);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3224);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8999);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16345);
							match s {
								[] => true,
								['р', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28205);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 28206);
											s.is_empty()
										}
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
					super::push(out, 9000);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 16346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9001);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 9002);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23180);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23181);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23182);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28207);
											s.is_empty()
										}
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
					super::push(out, 9003);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16348);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 28208);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31341);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 28209);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31342);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 23183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 16349);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 16350);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 16351);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23184);
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 23185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 16352);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23186);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 16353);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16354);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23187);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23188);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 23189);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 28210);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 16355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							super::push(out, 16356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 9005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16357);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 16358);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16359);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23192);
									s.is_empty()
								}
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
			super::push(out, 3225);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3226);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9007);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9008);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3227);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3228);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9010);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 16361);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23193);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 23194);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23195);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 28211);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 23196);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 23197);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 23198);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23199);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28212);
											s.is_empty()
										}
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
					super::push(out, 9011);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9012);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3229);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9013);
					s.is_empty()
				}
				['л', s @ ..] => super::t7::n9429(s, out),
				['р', s @ ..] => super::t7::n9430(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t7::n3294(s, out),
		['р', s @ ..] => super::t7::n3295(s, out),
		['с', s @ ..] => {
			super::push(out, 3232);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t7::n3297(s, out),
		['у', s @ ..] => super::t7::n3298(s, out),
		['ы', s @ ..] => super::t7::n3299(s, out),
		['ю', s @ ..] => super::t7::n3300(s, out),
		['я', s @ ..] => super::t7::n3301(s, out),
		_ => false,
	}
}

/// Words ending -ттаешь.
pub(crate) fn n698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 695);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утаешь.
pub(crate) fn n699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 696);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9028);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9030);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 9031);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16376);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16377);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3241);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9032);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23213);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23214);
									s.is_empty()
								}
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
			super::push(out, 3242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9033);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9034);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9035);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9036);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтаешь.
pub(crate) fn n700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 697);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3243);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9037);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 9038);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтаешь.
pub(crate) fn n701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 698);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3244);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16381);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16382);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 16383);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16384);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23215);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 16385);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штаешь.
pub(crate) fn n702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 699);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3245);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9039);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытаешь.
pub(crate) fn n703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 700);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3246);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3247);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9040);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 9041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9042);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9043);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9044);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9045);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятаешь.
pub(crate) fn n704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	s.is_empty()
}

/// Words ending -апхаешь.
pub(crate) fn n3338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3274);
	s.is_empty()
}

/// Words ending -рхаешь.
pub(crate) fn n712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 709);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3275);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16404);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3276);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16406);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16407);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3277);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 9078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3278);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -схаешь.
pub(crate) fn n713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 710);
	s.is_empty()
}

/// Words ending -ухаешь.
pub(crate) fn n714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 711);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3279);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3280);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9081);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9082);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16409);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 16410);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9083);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 9084);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9085);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9086);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16413);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 16414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9087);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9088);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9089);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3282);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9090);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9091);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3283);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3284);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9093);
					match s {
						[] => true,
						['а', 'л', 'б', 'а', s @ ..] => {
							super::push(out, 16418);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23225);
									s.is_empty()
								}
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
			super::push(out, 3285);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9094);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16419);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 16420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9095);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9096);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9097);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3286);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16422);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3287);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9100);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9101);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9102);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16424);
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 16425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3288);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9104);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9105);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9106);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чхаешь.
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 712);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3289);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыхаешь.
pub(crate) fn n716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 713);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3290);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3291);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9109);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9110);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9111);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16428);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23226);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23227);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 16429);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9112);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16432);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 16433);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9113);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 9114);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9115);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3292);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16435);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16436);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16437);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 16438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3293);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9117);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23230);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23231);
									s.is_empty()
								}
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
			super::push(out, 3294);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9118);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9119);
					match s {
						[] => true,
						['р', 'т', 'а', s @ ..] => {
							super::push(out, 16439);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9120);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16440);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 16441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3295);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16442);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16443);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3296);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16445);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 16446);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9122);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9123);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9124);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9125);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9126);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							super::push(out, 16448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9127);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9128);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9129);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 16450);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23234);
									s.is_empty()
								}
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
			super::push(out, 3297);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9130);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16451);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 16452);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28216);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 28217);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эхаешь.
pub(crate) fn n717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 714);
	s.is_empty()
}

/// Words ending -юхаешь.
pub(crate) fn n718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3298);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9131);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16453);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 16454);
							match s {
								[] => true,
								['п', 'и', s @ ..] => {
									super::push(out, 23235);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28219);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3299);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16455);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3300);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9132);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9133);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9134);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3301);
			match s {
				[] => true,
				['т', 'о', s @ ..] => {
					super::push(out, 9135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яхаешь.
pub(crate) fn n719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 716);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3302);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16458);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16459);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23236);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 16460);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16461);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 23237);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичаешь.
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 727);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9165);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9166);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3325);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9168);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 9169);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9170);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16479);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28224);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 28225);
											s.is_empty()
										}
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
					super::push(out, 9171);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16480);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23250);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 23251);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9172);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9173);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9174);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9175);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 9176);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t7::n3390(s, out),
		_ => false,
	}
}

/// Words ending -очаешь.
pub(crate) fn n734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 731);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3331);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16577);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 16578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23438);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23439);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23440);
									s.is_empty()
								}
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
			super::push(out, 3332);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -епчаешь.
pub(crate) fn n3397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3333);
	match s {
		[] => true,
		['р', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16579);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16580);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16581);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчаешь.
pub(crate) fn n736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 733);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3334);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16582);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3335);
			match s {
				[] => true,
				['г', 'о', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16584);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 16585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -учаешь.
pub(crate) fn n737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 734);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3337);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 3338);
			match s {
				[] => true,
				['и', 'о', s @ ..] => {
					super::push(out, 9200);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3339);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3340);
			match s {
				[] => true,
				['о', 'д', 'а', s @ ..] => {
					super::push(out, 9201);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9202);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23441);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 23442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3341);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9203);
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							super::push(out, 16590);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 23443);
									s.is_empty()
								}
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
			super::push(out, 3342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9204);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9205);
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					super::push(out, 9206);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3343);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3344);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3345);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 3346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьчаешь.
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 735);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3347);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16591);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 16592);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16593);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 16594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючаешь.
pub(crate) fn n739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 736);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ячаешь.
pub(crate) fn n740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 737);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слушаешь.
pub(crate) fn n9680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9245);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16610);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16611);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 16612);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нушаешь.
pub(crate) fn n3437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3373);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9246);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рушаешь.
pub(crate) fn n3439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3375);
	s.is_empty()
}

/// Words ending -сушаешь.
pub(crate) fn n3440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3376);
	s.is_empty()
}

/// Words ending -тушаешь.
pub(crate) fn n3441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3377);
	s.is_empty()
}

/// Words ending -чшаешь.
pub(crate) fn n751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 748);
	match s {
		[] => true,
		['у', 'л', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9247);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9248);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышаешь.
pub(crate) fn n752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 749);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьшаешь.
pub(crate) fn n753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 750);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 3379);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16614);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 16615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9249);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -невеешь.
pub(crate) fn n3545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3474);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 9350);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9351);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9352);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16664);
					match s {
						[] => true,
						['л', 'п', '\u{1}', s @ ..] => {
							super::push(out, 23477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 9353);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16665);
					match s {
						[] => true,
						['р', 'о', 'к', 'о', s @ ..] => {
							super::push(out, 23478);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28422);
									s.is_empty()
								}
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
			super::push(out, 9354);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 16666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еревеешь.
pub(crate) fn n9796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9355);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 16667);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 16668);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цевеешь.
pub(crate) fn n3547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3476);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9357);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16670);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 16671);
					match s {
						[] => true,
						['л', 'г', 'а', s @ ..] => {
							super::push(out, 23481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шевеешь.
pub(crate) fn n3548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3477);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9358);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					super::push(out, 16672);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -звеешь.
pub(crate) fn n826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 815);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3478);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3479);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16673);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивеешь.
pub(crate) fn n827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 816);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3480);
			match s {
				[] => true,
				['р', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 9359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3481);
			match s {
				[] => true,
				['у', 'л', 'е', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16675);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16676);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3482);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3483);
			match s {
				[] => true,
				['п', 'о', 'с', '\u{1}', s @ ..] => {
					super::push(out, 9361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3484);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3485);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16678);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9364);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9365);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9366);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3487);
			match s {
				[] => true,
				['о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16682);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3488);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16684);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16685);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23483);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 23484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16686);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16687);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'а', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16689);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16690);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лвеешь.
pub(crate) fn n828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 817);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овеешь.
pub(crate) fn n829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 818);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 3490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9369);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9370);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 16692);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 16693);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23486);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23487);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3491);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16694);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3492);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9372);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'о', 'р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16697);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16698);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16700);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16701);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3493);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9373);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9374);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16704);
							match s {
								[] => true,
								['т', 'с', 'е', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28423);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28424);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3494);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9375);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 16705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9376);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23489);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 9377);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16706);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9378);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9379);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16708);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28425);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28426);
											s.is_empty()
										}
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
					super::push(out, 9380);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3496);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3497);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9382);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23491);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23492);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9383);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9384);
					match s {
						[] => true,
						['д', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23494);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23495);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23496);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28427);
											s.is_empty()
										}
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
					super::push(out, 9385);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9386);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23497);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23498);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28428);
											s.is_empty()
										}
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
					super::push(out, 9387);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3498);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9388);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9389);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 9390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16712);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16713);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3499);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16715);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3500);
			match s {
				[] => true,
				['н', s @ ..] => super::t7::n9843(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -свеешь.
pub(crate) fn n830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 819);
	s.is_empty()
}

/// Words ending -твеешь.
pub(crate) fn n831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3501);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16719);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16720);
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
				['и', s @ ..] => {
					super::push(out, 9391);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16721);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23500);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 23501);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28431);
											s.is_empty()
										}
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
					super::push(out, 9392);
					match s {
						[] => true,
						['е', 'ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23502);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывеешь.
pub(crate) fn n832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3502);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явеешь.
pub(crate) fn n833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 822);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3503);
			match s {
				[] => true,
				['е', 'п', 'е', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 9393);
					s.is_empty()
				}
				['х', 'у', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 9394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3504);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хилеешь.
pub(crate) fn n3639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3553);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9460);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9461);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -млеешь.
pub(crate) fn n866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 850);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3554);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3555);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олеешь.
pub(crate) fn n867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 851);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9463);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9464);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9465);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9466);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3557);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9467);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3558);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 3559);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9468);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9469);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плеешь.
pub(crate) fn n868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 852);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9470);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слеешь.
pub(crate) fn n869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 853);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3561);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3562);
			match s {
				[] => true,
				['р', 'з', 'в', 'о', s @ ..] => {
					super::push(out, 9472);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3563);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9473);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлеешь.
pub(crate) fn n870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 854);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3564);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3565);
			match s {
				[] => true,
				['ч', 'е', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16763);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3566);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16765);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3567);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3568);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3569);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлеешь.
pub(crate) fn n871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 855);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 3570);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3571);
			s.is_empty()
		}
		['я', 'р', 'д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9476);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9477);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылеешь.
pub(crate) fn n873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 857);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 3572);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9478);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23520);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -денеешь.
pub(crate) fn n3694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3603);
	match s {
		[] => true,
		['е', 'л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16800);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16801);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 16802);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9508);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'т', 'с', '\u{1}', s @ ..] => {
			super::push(out, 9509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ленеешь.
pub(crate) fn n3695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3604);
	match s {
		[] => true,
		['е', 'з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16805);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'е', 'т', 'с', '\u{1}', s @ ..] => {
			super::push(out, 9510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -менеешь.
pub(crate) fn n3696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9511);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23529);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23530);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23532);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9512);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16807);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28433);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28434);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -епенеешь.
pub(crate) fn n3697(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 9513);
			s.is_empty()
		}
		['ц', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16808);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренеешь.
pub(crate) fn n3698(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9514);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 16810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9515);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 9516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенеешь.
pub(crate) fn n3699(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9517);
			s.is_empty()
		}
		['с', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16811);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16812);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16813);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торфенеешь.
pub(crate) fn n3700(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9518);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -коченеешь.
pub(crate) fn n3701(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9520);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9521);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жнеешь.
pub(crate) fn n885(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3606);
			match s {
				[] => true,
				['л', 'в', '\u{1}', s @ ..] => {
					super::push(out, 9523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3607);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знеешь.
pub(crate) fn n886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 869);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3608);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 9525);
					match s {
						[] => true,
						['р', 'е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23534);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23535);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28435);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3609);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3610);
			match s {
				[] => true,
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16814);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3611);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9526);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инеешь.
pub(crate) fn n887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 870);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9527);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3613);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9529);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стройнеешь.
pub(crate) fn n888(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3614);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тускнеешь.
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3616);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3617);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9530);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -полнеешь.
pub(crate) fn n890(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3618);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3619);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мнеешь.
pub(crate) fn n891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 871);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9531);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9533);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9534);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -длиннеешь.
pub(crate) fn n892(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3621);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онеешь.
pub(crate) fn n893(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3623);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'о', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9536);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3624);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -крупнеешь.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3625);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнеешь.
pub(crate) fn n895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 872);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3627);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9538);
					s.is_empty()
				}
				['д', 'а', s @ ..] => {
					super::push(out, 9539);
					s.is_empty()
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16816);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16817);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3628);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16819);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16821);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3629);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9540);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3630);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16824);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23536);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23537);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28436);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 16826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снеешь.
pub(crate) fn n896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 873);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3631);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16827);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16828);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3632);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9541);
					s.is_empty()
				}
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16830);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16831);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 16832);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16833);
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
					super::push(out, 9542);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3633);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9544);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9545);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9546);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9547);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнеешь.
pub(crate) fn n897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 874);
	match s {
		[] => true,
		['о', 'л', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9548);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'у', 'р', 'г', '\u{1}', s @ ..] => {
			super::push(out, 3634);
			s.is_empty()
		}
		['у', 'м', '\u{1}', s @ ..] => {
			super::push(out, 3635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унеешь.
pub(crate) fn n898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 875);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3636);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16836);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3637);
			match s {
				[] => true,
				['у', 'ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16838);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16839);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3638);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 9550);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3639);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9551);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3640);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чнеешь.
pub(crate) fn n899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 876);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3641);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16843);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 16844);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23541);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23542);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 23543);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 28437);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9552);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3642);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16845);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9554);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16847);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16848);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 16849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шнеешь.
pub(crate) fn n900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 877);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 3643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щнеешь.
pub(crate) fn n901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 878);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьнеешь.
pub(crate) fn n902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 879);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юнеешь.
pub(crate) fn n903(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3646);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -янеешь.
pub(crate) fn n904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 880);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3648);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', 'л', 'о', s @ ..] => {
					super::push(out, 9555);
					s.is_empty()
				}
				['о', 'в', 'а', s @ ..] => {
					super::push(out, 9556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'к', 'е', 'т', 'с', 'о', s @ ..] => {
			super::push(out, 3649);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3650);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9557);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16850);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23544);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23545);
									s.is_empty()
								}
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
			super::push(out, 3651);
			match s {
				[] => true,
				['г', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16851);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16852);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 3652);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 3653);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16854);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16855);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дуреешь.
pub(crate) fn n3801(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9594);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 9595);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыреешь.
pub(crate) fn n924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 898);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9596);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9597);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9598);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 9599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оробогатеешь.
pub(crate) fn n33384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31493);
	s.is_empty()
}

/// Words ending -ротеешь.
pub(crate) fn n3829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3715);
	match s {
		[] => true,
		['и', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16891);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16892);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птеешь.
pub(crate) fn n936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 910);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртеешь.
pub(crate) fn n937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 911);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стеешь.
pub(crate) fn n938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 912);
	match s {
		[] => true,
		['л', 'о', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9635);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9637);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16893);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16894);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16896);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утеешь.
pub(crate) fn n939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 913);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3718);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытеешь.
pub(crate) fn n940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 914);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ютеешь.
pub(crate) fn n941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 915);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9639);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9640);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятеешь.
pub(crate) fn n942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 916);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3721);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9641);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -амолкнешь.
pub(crate) fn n17937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16997);
	s.is_empty()
}

/// Words ending -умолкнешь.
pub(crate) fn n17938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16998);
	s.is_empty()
}

/// Words ending -толкнешь.
pub(crate) fn n10332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9815);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 16999);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23579);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мкнешь.
pub(crate) fn n1099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1072);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3948);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9816);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 9817);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 9818);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9819);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 9820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3949);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3950);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3951);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3952);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окнешь.
pub(crate) fn n1100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1073);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3953);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3954);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3955);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9821);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17000);
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
					super::push(out, 9822);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9823);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9824);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9825);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9826);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9827);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркнешь.
pub(crate) fn n1101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1074);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3956);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 9828);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 17003);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23580);
									s.is_empty()
								}
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
			super::push(out, 3957);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9829);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 17004);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17005);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17006);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9831);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17008);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17009);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23582);
									s.is_empty()
								}
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
			super::push(out, 3958);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3959);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9833);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17011);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 23583);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28440);
											s.is_empty()
										}
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
					super::push(out, 9834);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17012);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 9835);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 9836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3960);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3961);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9837);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17013);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 17014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9838);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 9839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 3962);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9840);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 9841);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9842);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скнешь.
pub(crate) fn n1102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1075);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3963);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9843);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3964);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17018);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ткнешь.
pub(crate) fn n1103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1076);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3966);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 3967);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укнешь.
pub(crate) fn n1104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1077);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3968);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3969);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9846);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чкнешь.
pub(crate) fn n1105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1078);
	s.is_empty()
}

/// Words ending -шкнешь.
pub(crate) fn n1106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1079);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкнешь.
pub(crate) fn n1107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1080);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3972);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3973);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 3974);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3975);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9848);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9849);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 9850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3976);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькнешь.
pub(crate) fn n1108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1081);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3977);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9852);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9853);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3978);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юкнешь.
pub(crate) fn n1109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1082);
	match s {
		[] => true,
		['т', 'а', s @ ..] => {
			super::push(out, 3979);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -якнешь.
pub(crate) fn n1110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1083);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3980);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3981);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3982);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3983);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3984);
			match s {
				[] => true,
				['б', 'ы', s @ ..] => {
					super::push(out, 9857);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9859);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9860);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9861);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыпорхнешь.
pub(crate) fn n10500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9980);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урхнешь.
pub(crate) fn n4231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4095);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 9981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ухнешь.
pub(crate) fn n1165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1136);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4096);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9982);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9983);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 9984);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9985);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9986);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9987);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9989);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9990);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4099);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9992);
					match s {
						[] => true,
						['с', 'и', s @ ..] => {
							super::push(out, 17094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9993);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9994);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 4101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чхнешь.
pub(crate) fn n1166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1137);
	s.is_empty()
}

/// Words ending -ыхнешь.
pub(crate) fn n1167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1138);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4102);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4103);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4104);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 4105);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9997);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9998);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9999);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10000);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4106);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17098);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17099);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4107);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10002);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17101);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17102);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эхнешь.
pub(crate) fn n1168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1139);
	s.is_empty()
}

/// Words ending -юхнешь.
pub(crate) fn n1169(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4108);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 4109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яхнешь.
pub(crate) fn n1170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1140);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17103);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10004);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10005);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17105);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17106);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твуешь.
pub(crate) fn n1260(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4216);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4217);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10077);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23612);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 23613);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t7::n4360(s, out),
		_ => false,
	}
}

/// Words ending -еседуешь.
pub(crate) fn n10660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10132);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23814);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28674);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ждуешь.
pub(crate) fn n1275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1239);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4241);
			match s {
				[] => true,
				['р', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17241);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17242);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здуешь.
pub(crate) fn n1276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1240);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4242);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4243);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4244);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 10133);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4245);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17244);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идуешь.
pub(crate) fn n1277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1241);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4246);
			match s {
				[] => true,
				['а', 'з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17246);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17247);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23815);
									s.is_empty()
								}
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
			super::push(out, 4247);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдуешь.
pub(crate) fn n1278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1242);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4248);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17248);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17249);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндуешь.
pub(crate) fn n1279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1243);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4249);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10136);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 10137);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23816);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23817);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23818);
									s.is_empty()
								}
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
			super::push(out, 4250);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10138);
					match s {
						[] => true,
						['о', 'к', 'е', 'р', '\u{1}', s @ ..] => {
							super::push(out, 17252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10139);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 17253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10140);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одуешь.
pub(crate) fn n1280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1244);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10141);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10142);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23819);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23820);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 23821);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23822);
									s.is_empty()
								}
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
			super::push(out, 4252);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4253);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4254);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10144);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10145);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17256);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17257);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 23823);
									s.is_empty()
								}
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
			super::push(out, 4255);
			match s {
				[] => true,
				['и', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17258);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17259);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10147);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23824);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 23825);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 23826);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 28676);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдуешь.
pub(crate) fn n1281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1245);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4256);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4257);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 10148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'к', 'с', 'а', s @ ..] => {
			super::push(out, 4258);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 10149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4259);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17261);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17262);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удуешь.
pub(crate) fn n1284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1248);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 4261);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17264);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28677);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28678);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 31670);
													s.is_empty()
												}
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
							super::push(out, 17265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдуешь.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1249);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядуешь.
pub(crate) fn n1286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1250);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4263);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17266);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17268);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ализуешь.
pub(crate) fn n10700(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17281);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 17282);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17283);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28679);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28680);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31671);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', 'т', 'а', 'м', 'е', s @ ..] => {
					super::push(out, 23829);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17284);
			s.is_empty()
		}
		['м', 'р', 'о', s @ ..] => {
			match s {
				[] => true,
				['н', 'е', s @ ..] => {
					super::push(out, 23830);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', 'а', s @ ..] => {
					super::push(out, 23831);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28683);
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
				['и', s @ ..] => {
					super::push(out, 23832);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23833);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28684);
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
					super::push(out, 23834);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23835);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23836);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 28685);
							s.is_empty()
						}
						['н', 'е', 'ц', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31672);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32946);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 31673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23837);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28686);
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
				['е', s @ ..] => {
					super::push(out, 23838);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23839);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17285);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мобилизуешь.
pub(crate) fn n18247(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23841);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 23842);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ростерилизуешь.
pub(crate) fn n25224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23843);
	s.is_empty()
}

/// Words ending -тилизуешь.
pub(crate) fn n18250(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 23844);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 23845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллизуешь.
pub(crate) fn n10703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10162);
	match s {
		[] => true,
		['а', 'т', 'с', 'и', 'р', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23846);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23847);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23848);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23849);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 23850);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олизуешь.
pub(crate) fn n10704(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 17288);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мизуешь.
pub(crate) fn n4432(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10163);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10164);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10165);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17290);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -низуешь.
pub(crate) fn n4433(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17291);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17292);
					match s {
						[] => true,
						['р', 'о', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28691);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28692);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17293);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17294);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17295);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10166);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17296);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ризуешь.
pub(crate) fn n4434(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10167);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17298);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17299);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17300);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							super::push(out, 23854);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 23855);
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
					super::push(out, 17301);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 23856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17302);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17303);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17304);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['к', 'а', 'р', 'а', 'х', 'о', s @ ..] => {
							super::push(out, 23857);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17306);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17307);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17309);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17310);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10168);
			s.is_empty()
		}
		['я', 'л', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17311);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17312);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тизуешь.
pub(crate) fn n4435(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10169);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10170);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10171);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мзуешь.
pub(crate) fn n1305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1269);
	s.is_empty()
}

/// Words ending -нзуешь.
pub(crate) fn n1306(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4285);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -узуешь.
pub(crate) fn n1307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1270);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4287);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 10173);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьзуешь.
pub(crate) fn n1308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1271);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4288);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17314);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17315);
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28695);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28696);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 31674);
													s.is_empty()
												}
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
							super::push(out, 17316);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзуешь.
pub(crate) fn n1309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1272);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аникуешь.
pub(crate) fn n10749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10199);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23871);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23872);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23873);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пикуешь.
pub(crate) fn n4460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4309);
	match s {
		[] => true,
		['ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17329);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рикуешь.
pub(crate) fn n4461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4310);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10200);
			match s {
				[] => true,
				['а', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23874);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23875);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10201);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тикуешь.
pub(crate) fn n4462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4311);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23877);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23879);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23880);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 23881);
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
					super::push(out, 17331);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17332);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фикуешь.
pub(crate) fn n4463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4312);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10202);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чикуешь.
pub(crate) fn n4464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4313);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 10203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щикуешь.
pub(crate) fn n4466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4315);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 10204);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word хиккуешь.
pub(crate) fn n1317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1280);
	s.is_empty()
}

/// Words ending -лкуешь.
pub(crate) fn n1318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1281);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4316);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 10205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4317);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 10206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4318);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17333);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17334);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17335);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 23882);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 28697);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17336);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нкуешь.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1282);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4319);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10207);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 10208);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10209);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4320);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 10211);
					s.is_empty()
				}
				['р', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 10212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10213);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17338);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10214);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 10215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17339);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 10216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17341);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окуешь.
pub(crate) fn n1320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1283);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4322);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 4323);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4324);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10217);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23884);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4325);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10219);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10220);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пкуешь.
pub(crate) fn n1321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1284);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ркуешь.
pub(crate) fn n1322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1285);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4327);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17343);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4328);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10222);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 10223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4329);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17345);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17346);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скуешь.
pub(crate) fn n1323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1286);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4331);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4332);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17348);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17349);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10226);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 10227);
					match s {
						[] => true,
						['н', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23887);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23888);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 28698);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4333);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 10228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17350);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ткуешь.
pub(crate) fn n1324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1287);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4334);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4335);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4336);
			match s {
				[] => true,
				['е', 'в', 'з', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17352);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17353);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4337);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 10230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укуешь.
pub(crate) fn n1325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1288);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10231);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10232);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10233);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10234);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4338);
			match s {
				[] => true,
				['ш', '\u{1}', s @ ..] => {
					super::push(out, 10235);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чкуешь.
pub(crate) fn n1326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1289);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4339);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17358);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17359);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4340);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4341);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шкуешь.
pub(crate) fn n1327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1290);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4342);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4343);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 4344);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыкуешь.
pub(crate) fn n1328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1291);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4345);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4346);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4347);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 10238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4348);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 10239);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 10240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькуешь.
pub(crate) fn n1329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1292);
	s.is_empty()
}

/// Words ending -юкуешь.
pub(crate) fn n1330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1293);
	match s {
		[] => true,
		['т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10241);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10242);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -якуешь.
pub(crate) fn n1331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1294);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 4349);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ируешь.
pub(crate) fn n1375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1338);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4426);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10331);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10332);
					match s {
						[] => true,
						['о', 'л', 'о', s @ ..] => {
							super::push(out, 17400);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10333);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17401);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17402);
							match s {
								[] => true,
								['к', 'с', 'н', 'а', 'р', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28703);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 28704);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28705);
											s.is_empty()
										}
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
					super::push(out, 10334);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17403);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17404);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23903);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28706);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 28707);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 28708);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 23904);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31675);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 31676);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 31677);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 31678);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10335);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10336);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17406);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17407);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 23906);
									match s {
										[] => true,
										['с', 'а', 'м', 'о', s @ ..] => {
											super::push(out, 28709);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31679);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10337);
					match s {
						[] => true,
						['к', 'н', 'и', 'о', s @ ..] => {
							super::push(out, 17408);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23907);
									s.is_empty()
								}
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
			super::push(out, 4427);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10338);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17409);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23908);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23909);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 23910);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28710);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 31680);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23911);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23912);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 23913);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23914);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 28711);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17410);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23915);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 23916);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28712);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 28713);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31681);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t7::n10905(s, out),
				['о', s @ ..] => {
					super::push(out, 10340);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t7::n10907(s, out),
				['ь', s @ ..] => super::t7::n10908(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t7::n4591(s, out),
		['д', s @ ..] => super::t7::n4592(s, out),
		['ж', s @ ..] => super::t7::n4593(s, out),
		['з', s @ ..] => super::t7::n4594(s, out),
		['и', s @ ..] => super::t7::n4595(s, out),
		['к', s @ ..] => super::t8::n4596(s, out),
		['л', s @ ..] => super::t8::n4597(s, out),
		['м', s @ ..] => super::t8::n4598(s, out),
		['н', s @ ..] => super::t8::n4599(s, out),
		['о', s @ ..] => super::t8::n4600(s, out),
		['п', s @ ..] => super::t8::n4601(s, out),
		['р', s @ ..] => super::t8::n4602(s, out),
		['с', s @ ..] => super::t8::n4603(s, out),
		['т', s @ ..] => super::t8::n4604(s, out),
		['у', s @ ..] => super::t8::n4605(s, out),
		['ф', s @ ..] => super::t8::n4606(s, out),
		['ц', s @ ..] => super::t8::n4607(s, out),
		['ч', 'ь', 'л', 'у', 'м', 'а', s @ ..] => super::t8::n4608(s, out),
		['ш', s @ ..] => super::t8::n4609(s, out),
		['ь', s @ ..] => super::t8::n4610(s, out),
		['ю', 'ь', 'в', 'р', 'е', 'т', 'н', 'и', 'о', s @ ..] => super::t8::n4611(s, out),
		_ => false,
	}
}

/// Words ending -штруешь.
pub(crate) fn n4618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4455);
	match s {
		[] => true,
		['у', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17732);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17733);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17734);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17735);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтруешь.
pub(crate) fn n4619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4456);
	match s {
		[] => true,
		['л', 'и', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17736);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17737);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уруешь.
pub(crate) fn n1378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1341);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 4457);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10505);
					s.is_empty()
				}
				['н', 'е', 'ц', 'о', s @ ..] => {
					super::push(out, 10506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4458);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10507);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4459);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17739);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17740);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17741);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4460);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10510);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10511);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10512);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10513);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фруешь.
pub(crate) fn n1379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1342);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4461);
			match s {
				[] => true,
				['ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17743);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17744);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17746);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17747);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17748);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 24294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юруешь.
pub(crate) fn n1380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1343);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 4462);
			match s {
				[] => true,
				['о', 'р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17750);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17751);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заинтересуешь.
pub(crate) fn n33814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31894);
	s.is_empty()
}

/// Words ending -осуешь.
pub(crate) fn n1388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1351);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10540);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 17767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17768);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24302);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 17769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гипсуешь.
pub(crate) fn n4646(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10541);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рсуешь.
pub(crate) fn n1390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1353);
	match s {
		[] => true,
		['о', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10543);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10544);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссуешь.
pub(crate) fn n1391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4479);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10545);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4480);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17771);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24305);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24306);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 24307);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24308);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24309);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29102);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17772);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24310);
									s.is_empty()
								}
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
			super::push(out, 4481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усуешь.
pub(crate) fn n1392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4482);
			s.is_empty()
		}
		['н', 'и', 'м', 'с', s @ ..] => {
			super::push(out, 4483);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4484);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юсуешь.
pub(crate) fn n1393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1356);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17773);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17775);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иктуешь.
pub(crate) fn n4671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4501);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10570);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17790);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10571);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 17792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтуешь.
pub(crate) fn n1398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1361);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4503);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4504);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нтуешь.
pub(crate) fn n1399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4505);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10574);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 10575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17795);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17796);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17797);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10577);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4506);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10579);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 10580);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17800);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17801);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24321);
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
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24322);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24323);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29105);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 17802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 10581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4507);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17803);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17805);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10583);
					match s {
						[] => true,
						['п', 'ш', 'а', s @ ..] => {
							super::push(out, 17807);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24324);
									s.is_empty()
								}
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
			super::push(out, 4508);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4509);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17808);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17809);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17810);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24325);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10586);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24326);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10587);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							super::push(out, 17811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отуешь.
pub(crate) fn n1400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1363);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 4510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртуешь.
pub(crate) fn n1401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1364);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4511);
			match s {
				[] => true,
				['в', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17812);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17813);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10588);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4512);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['н', 'о', 'к', 'а', s @ ..] => {
							super::push(out, 17816);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4513);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10589);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24328);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24329);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29106);
											s.is_empty()
										}
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
					super::push(out, 10590);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24330);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24331);
									s.is_empty()
								}
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
				['б', s @ ..] => {
					super::push(out, 10591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10592);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24332);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 24333);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29107);
											s.is_empty()
										}
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
					super::push(out, 10593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4514);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17819);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10595);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стуешь.
pub(crate) fn n1402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1365);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4515);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10596);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10598);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24334);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24335);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24336);
									s.is_empty()
								}
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
			super::push(out, 4516);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10599);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17824);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17825);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10600);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17826);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17827);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 24339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10601);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17828);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29108);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29109);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29110);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17829);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									super::push(out, 24340);
									s.is_empty()
								}
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
			super::push(out, 4517);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10602);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 10603);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17830);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4518);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4519);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10604);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 10605);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17832);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17833);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 10607);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттуешь.
pub(crate) fn n1403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1366);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утуешь.
pub(crate) fn n1404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1367);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10608);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4521);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17835);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 4522);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фтуешь.
pub(crate) fn n1405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1368);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4523);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10610);
					s.is_empty()
				}
				['р', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17837);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтуешь.
pub(crate) fn n1406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1369);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4524);
			match s {
				[] => true,
				['р', 'ф', '\u{1}', s @ ..] => {
					super::push(out, 10611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4525);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 10612);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17839);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24342);
									s.is_empty()
								}
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
			super::push(out, 4526);
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17840);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17841);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17842);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17844);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 10613);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтуешь.
pub(crate) fn n1407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1370);
	s.is_empty()
}

/// Words ending -ытуешь.
pub(crate) fn n1408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1371);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтуешь.
pub(crate) fn n1409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1372);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4528);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -этуешь.
pub(crate) fn n1410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1373);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4529);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4530);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 10616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютуешь.
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1374);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4531);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10617);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятуешь.
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1375);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 4532);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17846);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -агрохочешь.
pub(crate) fn n18943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17912);
	s.is_empty()
}

/// Words ending -огрохочешь.
pub(crate) fn n18944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17913);
	s.is_empty()
}

/// Words ending -хохочешь.
pub(crate) fn n11327(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17914);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17915);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17916);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -схочешь.
pub(crate) fn n4807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4615);
	match s {
		[] => true,
		['а', 'р', '\u{1}', s @ ..] => {
			super::push(out, 10723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -поуправляешь.
pub(crate) fn n33815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31895);
	s.is_empty()
}

/// Words ending -овляешь.
pub(crate) fn n4991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4781);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 10849);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17963);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'а', 'т', 'с', 'о', s @ ..] => {
			super::push(out, 10850);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10851);
			match s {
				[] => true,
				['о', 'н', s @ ..] => {
					super::push(out, 17965);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', 'г', 'а', s @ ..] => {
			super::push(out, 10852);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17966);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ствляешь.
pub(crate) fn n4992(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10853);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17968);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 24375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10854);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еляешь.
pub(crate) fn n1573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1530);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4783);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4784);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10855);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10856);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17969);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24376);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29116);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4785);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 4786);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 4787);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4788);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17970);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17971);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4789);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10857);
					match s {
						[] => true,
						['в', 'з', s @ ..] => {
							super::push(out, 17973);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24377);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29117);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иляешь.
pub(crate) fn n1574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1531);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10858);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10859);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10860);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10862);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10863);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 17974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4793);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 10864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4794);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 10865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10866);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10867);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кляешь.
pub(crate) fn n1575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1532);
	s.is_empty()
}

/// Words ending -мляешь.
pub(crate) fn n1576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1533);
	s.is_empty()
}

/// Words ending -оляешь.
pub(crate) fn n1577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1534);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4795);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17977);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
									super::push(out, 24378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4796);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10869);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10870);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 17980);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24379);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24380);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4797);
			s.is_empty()
		}
		['с', 'а', s @ ..] => {
			super::push(out, 4798);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 10872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4799);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пляешь.
pub(crate) fn n1578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1535);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10873);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 24382);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', 'о', s @ ..] => {
									super::push(out, 24383);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29119);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', 'о', s @ ..] => {
							super::push(out, 17981);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17982);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17983);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 24385);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29120);
											s.is_empty()
										}
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
					super::push(out, 10874);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 10875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рляешь.
pub(crate) fn n1579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1536);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 4801);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 10876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сляешь.
pub(crate) fn n1580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1537);
	s.is_empty()
}

/// Words ending -тляешь.
pub(crate) fn n1581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1538);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4802);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10877);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17985);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17986);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4803);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10879);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уляешь.
pub(crate) fn n1582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1539);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4804);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10880);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10881);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 10882);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10883);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10884);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10885);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4805);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4806);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4807);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10888);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фляешь.
pub(crate) fn n1583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1540);
	s.is_empty()
}

/// Words ending -хляешь.
pub(crate) fn n1584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1541);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4808);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10889);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17989);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24386);
									s.is_empty()
								}
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
			super::push(out, 4809);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шляешь.
pub(crate) fn n1585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1542);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4810);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17990);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17991);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17993);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17994);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 17995);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17996);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17997);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17998);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24387);
									s.is_empty()
								}
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
			super::push(out, 4811);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4812);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17999);
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24388);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29121);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31896);
													match s {
														[] => true,
														['п', s @ ..] => super::t8::n35010(s, out),
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
									super::push(out, 24389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 18000);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 24390);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 24391);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 24392);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 31897);
													match s {
														[] => true,
														['н', s @ ..] => super::t8::n35011(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 31898);
													match s {
														[] => true,
														['п', s @ ..] => super::t8::n35012(s, out),
														_ => false,
													}
												}
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
							super::push(out, 18001);
							s.is_empty()
						}
						['у', 'о', s @ ..] => {
							super::push(out, 18002);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 24393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18003);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыляешь.
pub(crate) fn n1586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1543);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4813);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10894);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24395);
									s.is_empty()
								}
								['о', 'д', s @ ..] => {
									super::push(out, 24396);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24397);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29122);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4814);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 4815);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4816);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4817);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 10896);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24398);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word сквернословишь.
pub(crate) fn n33819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31899);
	s.is_empty()
}

/// Words ending -асквернословишь.
pub(crate) fn n33820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31900);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 33029);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -осквернословишь.
pub(crate) fn n33821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31901);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 33030);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уровишь.
pub(crate) fn n5260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5029);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18161);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18162);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

