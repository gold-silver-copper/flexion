use flexion::Path;

/// Words ending -сленный.
pub(crate) fn n8600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8209);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14895);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27159);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14896);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14897);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21850);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27161);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							super::push(out, 27162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 21851);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27163);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27164);
							match s {
								[] => true,
								['р', 'е', 'п', 'е', s @ ..] => {
									super::push(out, 30425);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 32142);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 33087);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 32143);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 33088);
													s.is_empty()
												}
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
							super::push(out, 27165);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27166);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 30427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27167);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30428);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32144);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 30429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27168);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27169);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30431);
									s.is_empty()
								}
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
			super::push(out, 14898);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27170);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27171);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30432);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 30433);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 32145);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32146);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30434);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32147);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30435);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30436);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32148);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 32149);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32150);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33089);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30437);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30438);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тленный.
pub(crate) fn n8601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8210);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14899);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14900);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 21852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14901);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21853);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30439);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30440);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32152);
											s.is_empty()
										}
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
					super::push(out, 21854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14902);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21855);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14903);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уленный.
pub(crate) fn n8602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8211);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14904);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14905);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 14906);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 14907);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21858);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14908);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фленный.
pub(crate) fn n8603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8212);
	s.is_empty()
}

/// Words ending -хленный.
pub(crate) fn n8604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8213);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14909);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 14910);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21860);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -членный.
pub(crate) fn n8605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8214);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14911);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 14912);
			match s {
				[] => true,
				['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 21861);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шленный.
pub(crate) fn n8606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8215);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14913);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14914);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27174);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30441);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27175);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27176);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32153);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 32154);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32155);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33090);
													match s {
														[] => true,
														['р', s @ ..] => super::t10::n35138(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30443);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30444);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30445);
									s.is_empty()
								}
								['о', 'л', 'з', '\u{1}', s @ ..] => {
									super::push(out, 30446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27178);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32156);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32157);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыленный.
pub(crate) fn n8607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8216);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14915);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14916);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14917);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яленный.
pub(crate) fn n8608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8217);
	s.is_empty()
}

/// Words ending -одновременный.
pub(crate) fn n34738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33091);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33477);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -азновременный.
pub(crate) fn n34739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33092);
	s.is_empty()
}

/// Words ending -диновременный.
pub(crate) fn n34740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33093);
	s.is_empty()
}

/// Words ending -беременный.
pub(crate) fn n28538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27189);
	s.is_empty()
}

/// Words ending -переменный.
pub(crate) fn n28539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27190);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30460);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 30461);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32167);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пременный.
pub(crate) fn n22978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21874);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27191);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30462);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32168);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 33094);
									s.is_empty()
								}
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
			super::push(out, 27192);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30463);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зменный.
pub(crate) fn n8614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8223);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14927);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21875);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 21876);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27193);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27194);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27195);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30465);
									s.is_empty()
								}
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
			super::push(out, 14928);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21878);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21879);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21881);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 21882);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -именный.
pub(crate) fn n8615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8224);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йменный.
pub(crate) fn n8616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8225);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14930);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14931);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оменный.
pub(crate) fn n8617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8226);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14932);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21885);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21886);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 27202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14934);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21887);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 27203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рменный.
pub(crate) fn n8618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8227);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14935);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 21889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14937);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27205);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 27206);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27207);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30467);
									s.is_empty()
								}
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
			super::push(out, 14938);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27208);
							match s {
								[] => true,
								['р', 'о', 'р', s @ ..] => {
									super::push(out, 30468);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32169);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27209);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30469);
									match s {
										[] => true,
										['б', '\u{1}', s @ ..] => {
											super::push(out, 32170);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27210);
							match s {
								[] => true,
								['а', 'л', 'п', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32171);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 33095);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 32172);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сменный.
pub(crate) fn n8619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8228);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14939);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21890);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14940);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 14941);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21891);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27212);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 30470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тменный.
pub(crate) fn n8620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8229);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уменный.
pub(crate) fn n8621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8230);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 14943);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чменный.
pub(crate) fn n8622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8231);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 14945);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыменный.
pub(crate) fn n8623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8232);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 14946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьменный.
pub(crate) fn n8624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8233);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14947);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14948);
			match s {
				[] => true,
				['и', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27213);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27214);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27215);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word распространенный.
pub(crate) fn n34743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33096);
	s.is_empty()
}

/// Words ending -ераспространенный.
pub(crate) fn n34744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33097);
	s.is_empty()
}

/// Words ending -ораспространенный.
pub(crate) fn n34745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33098);
	s.is_empty()
}

/// Words ending -ененный.
pub(crate) fn n8630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8239);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 14963);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14964);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21912);
					match s {
						[] => true,
						['з', 'д', s @ ..] => {
							super::push(out, 27234);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 21913);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27235);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27236);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30484);
									s.is_empty()
								}
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
			super::push(out, 14965);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21914);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30485);
									match s {
										[] => true,
										['о', 'е', s @ ..] => {
											super::push(out, 32174);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33099);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 30486);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32175);
											s.is_empty()
										}
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
					super::push(out, 21915);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30487);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32176);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30488);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32177);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14966);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21916);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14967);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21917);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 21918);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14968);
			match s {
				[] => true,
				['а', 'з', 'у', s @ ..] => {
					super::push(out, 21919);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 14969);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21920);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							super::push(out, 27240);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27241);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27242);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зненный.
pub(crate) fn n8632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8241);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14970);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21924);
					match s {
						[] => true,
						['д', 'е', s @ ..] => {
							super::push(out, 27243);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 30491);
									s.is_empty()
								}
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
			super::push(out, 14971);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21925);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 21926);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14972);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 21927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14973);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21928);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21929);
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 27246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14974);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21930);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21931);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30492);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21932);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иненный.
pub(crate) fn n8633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8242);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14975);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21933);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21934);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14976);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21935);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27250);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14977);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14978);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21938);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21939);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 21940);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 27253);
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							super::push(out, 27254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14980);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 14981);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21941);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 27255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21942);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21943);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21944);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21945);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21946);
					match s {
						[] => true,
						['о', 'п', 'о', s @ ..] => {
							super::push(out, 27257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 21947);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21948);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21949);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27259);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27260);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 27261);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27262);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30494);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21950);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21951);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лненный.
pub(crate) fn n8634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8243);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14983);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27264);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30496);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30497);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32178);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33100);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32179);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 33101);
													s.is_empty()
												}
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
							super::push(out, 27265);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27266);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27267);
							match s {
								[] => true,
								['и', 'е', s @ ..] => {
									super::push(out, 30498);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27268);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мненный.
pub(crate) fn n8635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8244);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14984);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14985);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21953);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21954);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21955);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оненный.
pub(crate) fn n8636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8245);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14986);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14987);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21957);
					match s {
						[] => true,
						['з', 'у', 'е', s @ ..] => {
							super::push(out, 27271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14988);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21958);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27272);
							match s {
								[] => true,
								['р', 'п', 'о', s @ ..] => {
									super::push(out, 30500);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32181);
											match s {
												[] => true,
												['е', 'л', 'о', 'к', '\u{1}', s @ ..] => {
													super::push(out, 33102);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21959);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14989);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14990);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21961);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21962);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27273);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27274);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 27275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21963);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рненный.
pub(crate) fn n8638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8247);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21964);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 21965);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 21966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сненный.
pub(crate) fn n8639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8248);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14992);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21967);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27278);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30502);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32182);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27279);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27280);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30503);
									s.is_empty()
								}
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
			super::push(out, 14993);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21968);
					match s {
						[] => true,
						['р', 'п', 'е', s @ ..] => {
							super::push(out, 27281);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21969);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 21970);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27282);
							match s {
								[] => true,
								['о', 'е', s @ ..] => {
									super::push(out, 30505);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21971);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27284);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30506);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тненный.
pub(crate) fn n8640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8249);
	match s {
		[] => true,
		['у', 'м', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', 'е', s @ ..] => {
					super::push(out, 21972);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очненный.
pub(crate) fn n15752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14994);
	s.is_empty()
}

/// Words ending -шненный.
pub(crate) fn n8642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8251);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яненный.
pub(crate) fn n8643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8252);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14996);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жноустроенный.
pub(crate) fn n34750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33103);
	s.is_empty()
}

/// Words ending -бренный.
pub(crate) fn n8656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8265);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15027);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15028);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 15029);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22017);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15030);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15031);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27315);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 30523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27316);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гренный.
pub(crate) fn n8658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8267);
	match s {
		[] => true,
		['а', 'б', s @ ..] => {
			super::push(out, 15033);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дренный.
pub(crate) fn n8659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8268);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15034);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15035);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27317);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27318);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27319);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30526);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32190);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 30527);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32191);
											s.is_empty()
										}
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
					super::push(out, 22023);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 22024);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15036);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15037);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22026);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27321);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 30528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 27322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22027);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еренный.
pub(crate) fn n8660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8269);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15038);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22028);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22029);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22030);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27325);
							s.is_empty()
						}
						['р', 'п', 'е', s @ ..] => {
							super::push(out, 27326);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22031);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22032);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27328);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 30530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22033);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27329);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27330);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 30531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27331);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 30532);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22034);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22035);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30533);
									s.is_empty()
								}
								['д', 'е', 'р', 'п', 'е', 'н', '\u{1}', s @ ..] => {
									super::push(out, 30534);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30535);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 32192);
											s.is_empty()
										}
										['л', 'з', '\u{1}', s @ ..] => {
											super::push(out, 32193);
											s.is_empty()
										}
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
					super::push(out, 22036);
					s.is_empty()
				}
				['з', 'и', s @ ..] => {
					super::push(out, 22037);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22038);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22039);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22040);
					s.is_empty()
				}
				['у', 'е', s @ ..] => {
					super::push(out, 22041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15040);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15041);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22043);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15042);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22045);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22046);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27334);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27335);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 30536);
									s.is_empty()
								}
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
			super::push(out, 15043);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 15044);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 15045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зренный.
pub(crate) fn n8661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8270);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15046);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22048);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 27336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15047);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15048);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иренный.
pub(crate) fn n8662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8271);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 15049);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15050);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27337);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15051);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22052);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оренный.
pub(crate) fn n8663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8272);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15052);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22054);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27339);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22055);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27340);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30538);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30539);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32194);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27341);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22056);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27342);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27343);
							match s {
								[] => true,
								['л', 'в', 'о', 'д', 'у', 'е', s @ ..] => {
									super::push(out, 30541);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 32195);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27344);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 30542);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30543);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30544);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30545);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 30546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27345);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32196);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32197);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 33104);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n35140(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 33105);
													s.is_empty()
												}
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
							super::push(out, 27346);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30547);
									s.is_empty()
								}
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
			super::push(out, 15053);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 15054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22058);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22059);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15055);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22060);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27349);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30548);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32198);
											s.is_empty()
										}
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
					super::push(out, 22061);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27350);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30549);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 22062);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27351);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30551);
									s.is_empty()
								}
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
			super::push(out, 15056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22063);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22064);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15057);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22065);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22066);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15058);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22067);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22068);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22069);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15059);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22070);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22071);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 22072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15060);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22073);
					match s {
						[] => true,
						['з', 'е', s @ ..] => {
							super::push(out, 27358);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пренный.
pub(crate) fn n8664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8273);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 15061);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 22074);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 27359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тренный.
pub(crate) fn n8665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8274);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15062);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27360);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 27361);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 30553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 27362);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27363);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30555);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27364);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27365);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30557);
									s.is_empty()
								}
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
			super::push(out, 15063);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15064);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15065);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22076);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 27366);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30558);
									s.is_empty()
								}
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
			super::push(out, 15066);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22077);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 22078);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 27367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22079);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22080);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уренный.
pub(crate) fn n8666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8275);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15067);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22081);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22082);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15068);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 15069);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 15070);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22084);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15071);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22085);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27370);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 22086);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22087);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15072);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 22088);
					s.is_empty()
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27373);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15073);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15074);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22090);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22091);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 15075);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щренный.
pub(crate) fn n8667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8276);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15076);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыренный.
pub(crate) fn n8668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8277);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15078);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 15079);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 15080);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22092);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яренный.
pub(crate) fn n8670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8279);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 15081);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 15082);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22094);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 27376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овоприобретенный.
pub(crate) fn n34753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33106);
	s.is_empty()
}

/// Words ending -агоприобретенный.
pub(crate) fn n34754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33107);
	match s {
		[] => true,
		['л', 'б', '\u{1}', s @ ..] => {
			super::push(out, 33479);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -езамеченный.
pub(crate) fn n28762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27408);
	s.is_empty()
}

/// Words ending -печенный.
pub(crate) fn n15886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15128);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22138);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 22139);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22140);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 22141);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27409);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 30568);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33108);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33109);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33110);
											s.is_empty()
										}
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
					super::push(out, 27410);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30569);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32204);
									s.is_empty()
								}
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
			super::push(out, 22142);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27411);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реченный.
pub(crate) fn n15887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15129);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22143);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27412);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 27413);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22144);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22145);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22146);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27416);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30571);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32205);
									s.is_empty()
								}
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
			super::push(out, 22147);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22148);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27419);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сеченный.
pub(crate) fn n15888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15130);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22149);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 22150);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					super::push(out, 27420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22151);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 22152);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 22153);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 22154);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27421);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеченный.
pub(crate) fn n15889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15131);
	match s {
		[] => true,
		['е', 'р', 'б', s @ ..] => {
			super::push(out, 22155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иченный.
pub(crate) fn n8693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8302);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15132);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22156);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22157);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22158);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 22159);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22160);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15133);
			match s {
				[] => true,
				['а', 'р', 'г', s @ ..] => {
					match s {
						[] => true,
						['з', 'а', 'р', 'е', s @ ..] => {
							super::push(out, 27424);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'е', s @ ..] => {
							super::push(out, 27425);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', 'а', 'м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27426);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 27427);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лченный.
pub(crate) fn n8694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8303);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 15134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нченный.
pub(crate) fn n8695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8304);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15135);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22162);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 27428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15136);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27429);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 30577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27430);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30578);
									s.is_empty()
								}
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
			super::push(out, 15137);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22163);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27431);
							match s {
								[] => true,
								['з', 'е', 'н', '\u{1}', s @ ..] => {
									super::push(out, 30579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30580);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32206);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 30581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22164);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30582);
									s.is_empty()
								}
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
			super::push(out, 15138);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оченный.
pub(crate) fn n8696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8305);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15139);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22165);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22166);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15140);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 22167);
					match s {
						[] => true,
						['р', 'о', 'п', 'у', 'е', s @ ..] => {
							super::push(out, 27434);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30583);
									s.is_empty()
								}
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
			super::push(out, 15141);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 15142);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22168);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22169);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 27436);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 30584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22170);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27437);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 27438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30585);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30586);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 30587);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30588);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 32207);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30589);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30590);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32208);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 30591);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 32209);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 30592);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32210);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27439);
							match s {
								[] => true,
								['о', 'п', s @ ..] => {
									super::push(out, 30593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 27440);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 27441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22171);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30594);
									s.is_empty()
								}
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
			super::push(out, 15143);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22172);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22173);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27443);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 27444);
							match s {
								[] => true,
								['л', 'о', 'п', 'у', 'е', s @ ..] => {
									super::push(out, 30596);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32211);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22174);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15144);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22175);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 27447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22176);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27448);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30597);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30598);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 27449);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27450);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30600);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32213);
											s.is_empty()
										}
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
					super::push(out, 22177);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22178);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15145);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 22180);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15146);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22181);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 22182);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22183);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22184);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27454);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27455);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30601);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32214);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 30602);
									match s {
										[] => true,
										['о', s @ ..] => super::t10::n33819(s, out),
										['у', s @ ..] => {
											super::push(out, 32216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t10::n28811(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t10::n23304(s, out),
				['ы', s @ ..] => super::t10::n23305(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пченный.
pub(crate) fn n8697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8306);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 15147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рченный.
pub(crate) fn n8698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8307);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15148);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15149);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22188);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 22189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15150);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22190);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22191);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 22192);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27459);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27460);
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32217);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32218);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33112);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ученный.
pub(crate) fn n8699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8308);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15151);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 15152);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 15153);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 15154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22193);
					match s {
						[] => true,
						['м', 'а', s @ ..] => {
							super::push(out, 27461);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22194);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27462);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30604);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32219);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 30605);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32220);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15155);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 15156);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 15157);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22195);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22196);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15158);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22198);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22199);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22200);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15159);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 15160);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 22201);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15161);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22202);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22203);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22204);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 22205);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22206);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27468);
							match s {
								[] => true,
								['п', 'е', s @ ..] => {
									super::push(out, 30606);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32221);
											s.is_empty()
										}
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
					super::push(out, 22207);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27469);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27470);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27471);
							match s {
								[] => true,
								['а', 'р', 'е', s @ ..] => {
									super::push(out, 30607);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32222);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27472);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30608);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22208);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22209);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15162);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 15163);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 15164);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22210);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27475);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыченный.
pub(crate) fn n8700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8309);
	s.is_empty()
}

/// Words ending -ьченный.
pub(crate) fn n8701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8310);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15165);
			match s {
				[] => true,
				['е', 'м', 'з', 'и', 'о', s @ ..] => {
					super::push(out, 22211);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юченный.
pub(crate) fn n8702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8311);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15166);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27477);
							match s {
								[] => true,
								['з', 'е', s @ ..] => {
									super::push(out, 30610);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32223);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27478);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30611);
									s.is_empty()
								}
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
			super::push(out, 15167);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 15168);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22213);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яченный.
pub(crate) fn n8703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8312);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15169);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 15170);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 15171);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22214);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22215);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22216);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22217);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15172);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15173);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мноокрашенный.
pub(crate) fn n34760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33113);
	match s {
		[] => true,
		['е', 'т', '\u{1}', s @ ..] => {
			super::push(out, 33480);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -незанавешенный.
pub(crate) fn n32123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30621);
	s.is_empty()
}

/// Words ending -двешенный.
pub(crate) fn n23355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22234);
	s.is_empty()
}

/// Words ending -звешенный.
pub(crate) fn n23356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22235);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овешенный.
pub(crate) fn n23357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22236);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 27499);
			match s {
				[] => true,
				['в', 'а', 'р', 'у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32229);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 27500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывешенный.
pub(crate) fn n23358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22237);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мешенный.
pub(crate) fn n15941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15183);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22238);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 22239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пешенный.
pub(crate) fn n15942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15184);
	s.is_empty()
}

/// Words ending -решенный.
pub(crate) fn n15943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15185);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 22240);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 22241);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					super::push(out, 27502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 22242);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 22243);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 22244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тешенный.
pub(crate) fn n15944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15186);
	s.is_empty()
}

/// Words ending -ишенный.
pub(crate) fn n8707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8316);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15188);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22246);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мшенный.
pub(crate) fn n8708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8317);
	s.is_empty()
}

/// Words ending -ошенный.
pub(crate) fn n8709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8318);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 15190);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22248);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22249);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22250);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27504);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30622);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22251);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15191);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15192);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15193);
			match s {
				[] => true,
				['а', 'з', s @ ..] => {
					super::push(out, 22254);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22255);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22256);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22257);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22258);
					match s {
						[] => true,
						['д', 'е', s @ ..] => {
							super::push(out, 27506);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30623);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22259);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22260);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22261);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15194);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22262);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27509);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30624);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 30625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27510);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30626);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22263);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 27511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22264);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27512);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22265);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27513);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 27514);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 27515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22266);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27516);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27517);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 30630);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32231);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27518);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22267);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27519);
							match s {
								[] => true,
								['п', 'ы', s @ ..] => {
									super::push(out, 30632);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32232);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15195);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пшенный.
pub(crate) fn n8710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8319);
	s.is_empty()
}

/// Words ending -ршенный.
pub(crate) fn n8711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8320);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15196);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27520);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27521);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30633);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32233);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27522);
							match s {
								[] => true,
								['с', 'е', s @ ..] => {
									super::push(out, 30634);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15197);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22269);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 22270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ушенный.
pub(crate) fn n8712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8321);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15198);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22271);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15199);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22273);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 22274);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22275);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22276);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22277);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27524);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27525);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30636);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32235);
											s.is_empty()
										}
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
					super::push(out, 22278);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15200);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15201);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15202);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15203);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22283);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 22284);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15204);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22285);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22286);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22287);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22288);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15205);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22289);
					match s {
						[] => true,
						['п', 'е', s @ ..] => {
							super::push(out, 27528);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чшенный.
pub(crate) fn n8713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8322);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 15206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ышенный.
pub(crate) fn n8714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8323);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15207);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22290);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьшенный.
pub(crate) fn n8715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8324);
	s.is_empty()
}

/// Words ending -юшенный.
pub(crate) fn n8716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8325);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рехзамещенный.
pub(crate) fn n34761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33114);
	s.is_empty()
}

/// Words ending -ищенный.
pub(crate) fn n8721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8330);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 15221);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22314);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27541);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27542);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32243);
											s.is_empty()
										}
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 32244);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15222);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22315);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22316);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27543);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22317);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22318);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 15223);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22319);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30649);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30650);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30651);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32246);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 33115);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 32247);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 33116);
													match s {
														[] => true,
														['м', 'м', 'и', '\u{1}', s @ ..] => super::t10::n35143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['х', s @ ..] => {
											super::push(out, 32248);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33117);
													match s {
														[] => true,
														['м', 'о', 'п', '\u{1}', s @ ..] => super::t10::n35144(s, out),
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
									super::push(out, 30652);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ощенный.
pub(crate) fn n8724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8333);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15225);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 15226);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 15227);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 15228);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22320);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 22321);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22322);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27546);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 30653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15229);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 22324);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15230);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15231);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22325);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22326);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22327);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27549);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30654);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27550);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22328);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ущенный.
pub(crate) fn n8726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8335);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 15234);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15235);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22331);
					match s {
						[] => true,
						['о', 'в', 'е', s @ ..] => {
							super::push(out, 27553);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30656);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15236);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22333);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22334);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27555);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30657);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32249);
											s.is_empty()
										}
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
					super::push(out, 22335);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22336);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 27556);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22337);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 15237);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22338);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыщенный.
pub(crate) fn n8727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8336);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15238);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15239);
			match s {
				[] => true,
				['а', 'н', 'е', 'н', s @ ..] => {
					super::push(out, 22340);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22341);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьщенный.
pub(crate) fn n8728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8337);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15240);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ющенный.
pub(crate) fn n8729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8338);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15241);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ященный.
pub(crate) fn n8730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8339);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15242);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27558);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27559);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['е', 'н', '\u{1}', s @ ..] => {
											super::push(out, 32250);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32251);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33118);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -о-телевизионный.
pub(crate) fn n34766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33119);
	s.is_empty()
}

/// Words ending -иссионный.
pub(crate) fn n23522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22391);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 30669);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 30670);
					s.is_empty()
				}
				['э', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32260);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 32261);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32262);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 32263);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тионный.
pub(crate) fn n8838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8436);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15342);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15343);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ционный.
pub(crate) fn n8839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8437);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15344);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22395);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27591);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 27592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 32265);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33120);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => super::t10::n33870(s, out),
										['ь', s @ ..] => super::t10::n33871(s, out),
										_ => false,
									}
								}
								['х', s @ ..] => super::t10::n32181(s, out),
								_ => false,
							}
						}
						['о', 'н', 'н', 'и', '-', s @ ..] => super::t10::n28955(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t10::n23528(s, out),
				['д', s @ ..] => super::t10::n23529(s, out),
				['з', s @ ..] => super::t10::n23530(s, out),
				['и', 'ц', s @ ..] => super::t10::n23531(s, out),
				['к', s @ ..] => super::t11::n23532(s, out),
				['л', s @ ..] => super::t11::n23533(s, out),
				['м', s @ ..] => super::t11::n23534(s, out),
				['н', s @ ..] => super::t11::n23535(s, out),
				['п', s @ ..] => super::t11::n23536(s, out),
				['р', s @ ..] => super::t11::n23537(s, out),
				['с', s @ ..] => super::t11::n23538(s, out),
				['т', s @ ..] => super::t11::n23539(s, out),
				['у', 'т', s @ ..] => super::t11::n23540(s, out),
				_ => false,
			}
		}
		['б', 'р', 'о', 'с', s @ ..] => super::t11::n16110(s, out),
		['и', s @ ..] => super::t11::n16111(s, out),
		['к', s @ ..] => super::t11::n16112(s, out),
		['н', s @ ..] => super::t11::n16113(s, out),
		['п', s @ ..] => super::t11::n16114(s, out),
		['р', s @ ..] => super::t11::n16115(s, out),
		['у', s @ ..] => super::t11::n16116(s, out),
		['ю', s @ ..] => super::t11::n16117(s, out),
		['я', s @ ..] => super::t11::n16118(s, out),
		_ => false,
	}
}

/// Words ending -кторный.
pub(crate) fn n9153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8732);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15642);
			match s {
				[] => true,
				['ф', 'ь', s @ ..] => {
					super::push(out, 22611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15643);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22612);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22613);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 22614);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 22615);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27757);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 30786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 27758);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 27759);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32338);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 32339);
											s.is_empty()
										}
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
					super::push(out, 22616);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 27760);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22617);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27761);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27762);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 30788);
									s.is_empty()
								}
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
			super::push(out, 15644);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22618);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15645);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27764);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 27765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ромоторный.
pub(crate) fn n29149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27766);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 30790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сомоторный.
pub(crate) fn n29150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27767);
	match s {
		[] => true,
		['к', 'а', 'т', '-', s @ ..] => {
			super::push(out, 30791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роторный.
pub(crate) fn n16438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15647);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 22619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пторный.
pub(crate) fn n9155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8734);
	match s {
		[] => true,
		['е', 'ц', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22620);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 22621);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сторный.
pub(crate) fn n9156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8735);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15648);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27768);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 27769);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15649);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уторный.
pub(crate) fn n9157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8736);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 15650);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15651);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 15652);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 15653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яторный.
pub(crate) fn n9158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8737);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22625);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27770);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', 'е', s @ ..] => {
							super::push(out, 27771);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 27772);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30794);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 32341);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 27773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -адопосадочный.
pub(crate) fn n34801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33152);
	s.is_empty()
}

/// Words ending -есопосадочный.
pub(crate) fn n34802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33153);
	s.is_empty()
}

/// The word электросварочный.
pub(crate) fn n34803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33154);
	s.is_empty()
}

/// Words ending -поточный.
pub(crate) fn n17145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16312);
	match s {
		[] => true,
		['о', 'в', s @ ..] => {
			super::push(out, 23093);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27988);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роточный.
pub(crate) fn n17146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16313);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 23094);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 23095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -соточный.
pub(crate) fn n17147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16314);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23096);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хоточный.
pub(crate) fn n17148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16315);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рточный.
pub(crate) fn n9719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9270);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16316);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16317);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сточный.
pub(crate) fn n9720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9271);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16318);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23098);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 27990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16319);
			match s {
				[] => true,
				['в', '-', 'о', s @ ..] => {
					super::push(out, 23099);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27991);
							s.is_empty()
						}
						['р', 'е', 'в', 'е', 'с', '\u{1}', s @ ..] => {
							super::push(out, 27992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16320);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23100);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уточный.
pub(crate) fn n9721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9272);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 16322);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16323);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 16324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыточный.
pub(crate) fn n9722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9273);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16325);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 16326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эточный.
pub(crate) fn n9723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9274);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 16327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьошировальный.
pub(crate) fn n34804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33155);
	s.is_empty()
}

/// Words ending -иальный.
pub(crate) fn n9818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9364);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16395);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 16396);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23147);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23148);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28009);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 23149);
					match s {
						[] => true,
						['л', 'л', 'а', s @ ..] => {
							super::push(out, 28010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16397);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23150);
					match s {
						[] => true,
						['м', 'е', s @ ..] => {
							super::push(out, 28011);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16398);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16399);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23152);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', 'о', 'к', 'и', s @ ..] => {
							super::push(out, 28012);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 30889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'и', s @ ..] => {
							super::push(out, 28013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16400);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16401);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					super::push(out, 23153);
					s.is_empty()
				}
				['е', 'т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28014);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'т', s @ ..] => {
					super::push(out, 23154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28016);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28017);
							match s {
								[] => true,
								['р', 'р', 'е', 'т', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 32373);
											s.is_empty()
										}
										['ж', s @ ..] => {
											super::push(out, 32374);
											s.is_empty()
										}
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
					super::push(out, 23155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16402);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 16403);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 16404);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23157);
					match s {
						[] => true,
						['п', 'с', 'о', s @ ..] => {
							super::push(out, 28018);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23158);
					match s {
						[] => true,
						['ф', 'о', '-', 'о', 'н', 'н', 'е', 'з', 'а', 'к', '\u{1}', s @ ..] => {
							super::push(out, 28019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23159);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28020);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 28021);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23160);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерикальный.
pub(crate) fn n29421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28023);
	s.is_empty()
}

/// Words ending -ирикальный.
pub(crate) fn n29422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28024);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30891);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртикальный.
pub(crate) fn n24347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23163);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фокальный.
pub(crate) fn n17247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16406);
	s.is_empty()
}

/// Words ending -ззеркальный.
pub(crate) fn n17248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16407);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23164);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скальный.
pub(crate) fn n17249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16408);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23165);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23166);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 28028);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мальный.
pub(crate) fn n9820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9366);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16409);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16410);
			match s {
				[] => true,
				['о', 'ф', s @ ..] => {
					super::push(out, 23167);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28029);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 30893);
									s.is_empty()
								}
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
			super::push(out, 16411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нальный.
pub(crate) fn n9821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9367);
	match s {
		[] => true,
		['а', 'х', s @ ..] => {
			super::push(out, 16412);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16413);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['г', 'а', 'в', s @ ..] => {
					super::push(out, 23168);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 23169);
					s.is_empty()
				}
				['м', 'о', s @ ..] => {
					super::push(out, 23170);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23171);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28031);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 23172);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 23173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16414);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23174);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28033);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 28034);
							s.is_empty()
						}
						['с', 'с', 'е', 'ф', 'о', s @ ..] => {
							super::push(out, 28035);
							match s {
								[] => true,
								['р', 'п', 'о', 'к', s @ ..] => {
									super::push(out, 30895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 28036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16415);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16416);
			match s {
				[] => true,
				['м', 'м', 'о', 'к', '-', s @ ..] => {
					super::push(out, 23176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пальный.
pub(crate) fn n9822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9368);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16417);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16418);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23177);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23178);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28037);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 28038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23179);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ральный.
pub(crate) fn n9823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9369);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16419);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 16420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23180);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16421);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23182);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 23183);
					match s {
						[] => true,
						['а', 'с', s @ ..] => {
							super::push(out, 28039);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30897);
									s.is_empty()
								}
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
			super::push(out, 16422);
			match s {
				[] => true,
				['б', 'и', 'л', 'о', s @ ..] => {
					super::push(out, 23184);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28040);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23185);
					s.is_empty()
				}
				['н', 'е', s @ ..] => {
					super::push(out, 23186);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23187);
					match s {
						[] => true,
						['а', 'л', 'р', s @ ..] => {
							super::push(out, 28041);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 30899);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32375);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16423);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23188);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23189);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16424);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23190);
					s.is_empty()
				}
				['т', 'к', s @ ..] => {
					super::push(out, 23191);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							super::push(out, 28044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16425);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23192);
					match s {
						[] => true,
						['р', 'у', 'о', s @ ..] => {
							super::push(out, 28045);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'и', s @ ..] => {
					super::push(out, 23193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сальный.
pub(crate) fn n9824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9370);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16427);
			match s {
				[] => true,
				['ч', 'о', s @ ..] => {
					super::push(out, 23194);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16428);
			s.is_empty()
		}
		['с', 'о', s @ ..] => {
			super::push(out, 16429);
			match s {
				[] => true,
				['л', 'г', '\u{1}', s @ ..] => {
					super::push(out, 23195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тальный.
pub(crate) fn n9825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9371);
	match s {
		[] => true,
		['а', 'н', s @ ..] => {
			super::push(out, 16430);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23196);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16431);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16432);
			match s {
				[] => true,
				['б', 'р', 'о', 'о', s @ ..] => {
					super::push(out, 23197);
					match s {
						[] => true,
						['к', 'з', s @ ..] => {
							super::push(out, 28048);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23198);
					s.is_empty()
				}
				['п', 'с', s @ ..] => {
					super::push(out, 23199);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 23200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16433);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16434);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['м', 'у', 'р', s @ ..] => {
							super::push(out, 28049);
							match s {
								[] => true,
								['т', 'с', 'н', 'и', 'о', s @ ..] => {
									super::push(out, 30903);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32376);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28050);
							match s {
								[] => true,
								['и', 'т', 'н', 'о', 'к', 'е', s @ ..] => {
									super::push(out, 30904);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32377);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16435);
			match s {
				[] => true,
				['м', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 23201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16436);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16437);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28051);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28052);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30905);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23202);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28053);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16438);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23203);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23204);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уальный.
pub(crate) fn n9826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9372);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16439);
			match s {
				[] => true,
				['и', 'ц', s @ ..] => {
					super::push(out, 23205);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'и', 'в', 'о', s @ ..] => {
			super::push(out, 16440);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23206);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28057);
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
				['к', 'е', 'с', 'о', 'м', s @ ..] => {
					super::push(out, 23207);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'ц', 'о', 'р', 'п', '-', s @ ..] => {
					super::push(out, 23208);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фальный.
pub(crate) fn n9827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9373);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16441);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23209);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кхальный.
pub(crate) fn n9828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9374);
	s.is_empty()
}

/// Words ending -чальный.
pub(crate) fn n9829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9375);
	match s {
		[] => true,
		['а', 'н', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23210);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16442);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16443);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -некорабельный.
pub(crate) fn n34805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33156);
	s.is_empty()
}

/// Words ending -инедельный.
pub(crate) fn n29468(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 30912);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32380);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 32381);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 30913);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32382);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 32383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онедельный.
pub(crate) fn n29469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28070);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 30914);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 30915);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 32385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 30916);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30917);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 30918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хнедельный.
pub(crate) fn n29470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28071);
	match s {
		[] => true,
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					super::push(out, 32387);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 32388);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33159);
							match s {
								[] => true,
								['е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 33495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'в', 'д', '\u{1}', s @ ..] => {
			super::push(out, 30919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -редельный.
pub(crate) fn n24413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23224);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28072);
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['о', 'с', s @ ..] => {
					super::push(out, 30920);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 30921);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word трехраздельный.
pub(crate) fn n24415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23226);
	s.is_empty()
}

/// Words ending -ндельный.
pub(crate) fn n17304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16460);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23227);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одельный.
pub(crate) fn n17305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16461);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23228);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28073);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23229);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23230);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 28075);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сдельный.
pub(crate) fn n17306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16462);
	s.is_empty()
}

/// Words ending -тдельный.
pub(crate) fn n17307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16463);
	s.is_empty()
}

/// Words ending -удельный.
pub(crate) fn n17308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16464);
	s.is_empty()
}

/// Words ending -зельный.
pub(crate) fn n9834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9380);
	s.is_empty()
}

/// Words ending -кельный.
pub(crate) fn n9835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9381);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16465);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23231);
					s.is_empty()
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28076);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лельный.
pub(crate) fn n9836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9382);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 16467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мельный.
pub(crate) fn n9837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9383);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16468);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23232);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16469);
			match s {
				[] => true,
				['з', 'о', 'н', 'ч', 'о', 'л', 'е', 'щ', '\u{1}', s @ ..] => {
					super::push(out, 23234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 16470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23235);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нельный.
pub(crate) fn n9838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9384);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16471);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16472);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16473);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23238);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16474);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23240);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пельный.
pub(crate) fn n9839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16475);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23241);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28080);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23242);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 28081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16476);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23243);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23244);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28082);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16477);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 16478);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16479);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рельный.
pub(crate) fn n9840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9386);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 16480);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сельный.
pub(crate) fn n9841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9387);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', 'у', s @ ..] => {
					super::push(out, 23245);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16481);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16482);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23248);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'п', 'а', s @ ..] => {
					super::push(out, 23249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16483);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23250);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23251);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тельный.
pub(crate) fn n9842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16484);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23252);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23253);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28088);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 30925);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32389);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 30926);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32390);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 28089);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 30927);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30928);
									match s {
										[] => true,
										['г', s @ ..] => {
											match s {
												[] => true,
												['а', 'н', 'о', s @ ..] => {
													super::push(out, 33160);
													match s {
														[] => true,
														['л', s @ ..] => super::t11::n35159(s, out),
														['р', s @ ..] => super::t11::n35160(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 33161);
													s.is_empty()
												}
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
							super::push(out, 28090);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28091);
							s.is_empty()
						}
						['о', s @ ..] => super::t11::n29492(s, out),
						['ы', s @ ..] => {
							super::push(out, 28093);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30934);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32396);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 30935);
									match s {
										[] => true,
										['а', s @ ..] => super::t11::n34007(s, out),
										_ => false,
									}
								}
								['м', s @ ..] => super::t11::n32465(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t11::n24444(s, out),
				['д', s @ ..] => super::t11::n24445(s, out),
				['ж', s @ ..] => super::t11::n24446(s, out),
				['з', s @ ..] => super::t11::n24447(s, out),
				['к', s @ ..] => super::t11::n24448(s, out),
				['л', s @ ..] => super::t11::n24449(s, out),
				['м', s @ ..] => super::t11::n24450(s, out),
				['н', s @ ..] => super::t11::n24451(s, out),
				['п', s @ ..] => super::t11::n24452(s, out),
				['р', s @ ..] => super::t11::n24453(s, out),
				['с', s @ ..] => super::t11::n24454(s, out),
				['т', s @ ..] => super::t11::n24455(s, out),
				['х', s @ ..] => super::t11::n24456(s, out),
				['ц', s @ ..] => super::t11::n24457(s, out),
				['ч', s @ ..] => super::t11::n24458(s, out),
				['щ', s @ ..] => super::t11::n24459(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t11::n17330(s, out),
		['и', s @ ..] => super::t11::n17331(s, out),
		['о', s @ ..] => super::t11::n17332(s, out),
		['р', s @ ..] => super::t11::n17333(s, out),
		['с', s @ ..] => super::t11::n17334(s, out),
		['я', s @ ..] => super::t11::n17335(s, out),
		_ => false,
	}
}

/// Words ending -фельный.
pub(crate) fn n9843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9389);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16491);
			s.is_empty()
		}
		['и', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23295);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 23296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16492);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23297);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16493);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 16494);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цельный.
pub(crate) fn n9844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9390);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16495);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16496);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23299);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 28172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16497);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чельный.
pub(crate) fn n9845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9391);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16499);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16500);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щельный.
pub(crate) fn n9846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9392);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 16502);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кольный.
pub(crate) fn n9873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9419);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 16562);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16563);
			match s {
				[] => true,
				['т', 'с', 'о', s @ ..] => {
					super::push(out, 23366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16564);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23367);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23368);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23369);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 23370);
					match s {
						[] => true,
						['о', s @ ..] => {
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
			super::push(out, 16565);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 16566);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мольный.
pub(crate) fn n9874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9420);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16567);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16568);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16569);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23373);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 23374);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анольный.
pub(crate) fn n17422(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23376);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23377);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енольный.
pub(crate) fn n17423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16570);
	s.is_empty()
}

/// Words ending -польный.
pub(crate) fn n9876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9422);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16571);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16572);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16573);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23380);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23381);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16574);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16575);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23384);
					match s {
						[] => true,
						['о', 'м', 'и', s @ ..] => {
							super::push(out, 28212);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23385);
					match s {
						[] => true,
						['т', 'е', 'м', 'е', s @ ..] => {
							super::push(out, 28214);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31016);
									s.is_empty()
								}
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
			super::push(out, 16576);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16577);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23386);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23387);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28216);
							s.is_empty()
						}
						['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 28217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23388);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 16578);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23389);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28219);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 31017);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 31018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23390);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 16579);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рольный.
pub(crate) fn n9877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9423);
	match s {
		[] => true,
		['и', 'т', 'с', 'н', s @ ..] => {
			super::push(out, 16580);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'н', 'о', 'к', 'д', s @ ..] => {
			super::push(out, 16581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сольный.
pub(crate) fn n9878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9424);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16582);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23393);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16583);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16584);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23394);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тольный.
pub(crate) fn n9879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9425);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16585);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23396);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23397);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23398);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28223);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уольный.
pub(crate) fn n9880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9426);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 16586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фольный.
pub(crate) fn n9881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9427);
	s.is_empty()
}

/// Words ending -хольный.
pub(crate) fn n9882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9428);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 16587);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 23400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -авшийся.
pub(crate) fn n10391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9915);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16999);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 17000);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23670);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23671);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28357);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28358);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 31052);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 31053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23672);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28360);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28361);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28362);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t11::n24880(s, out),
				['о', s @ ..] => super::t11::n24881(s, out),
				['р', s @ ..] => {
					super::push(out, 23675);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28388);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28389);
							match s {
								[] => true,
								['в', s @ ..] => super::t11::n32634(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t11::n24883(s, out),
				['ы', s @ ..] => super::t11::n24884(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t11::n17890(s, out),
		['д', s @ ..] => super::t11::n17891(s, out),
		['ж', s @ ..] => super::t11::n17892(s, out),
		['з', s @ ..] => super::t11::n17893(s, out),
		['к', s @ ..] => super::t11::n17894(s, out),
		['л', s @ ..] => super::t11::n17895(s, out),
		['м', s @ ..] => super::t11::n17896(s, out),
		['н', s @ ..] => super::t11::n17897(s, out),
		['п', s @ ..] => super::t11::n17898(s, out),
		['р', s @ ..] => super::t11::n17899(s, out),
		['с', s @ ..] => super::t11::n17900(s, out),
		['т', s @ ..] => super::t11::n17901(s, out),
		['х', s @ ..] => super::t11::n17902(s, out),
		['ч', s @ ..] => super::t11::n17903(s, out),
		['ш', s @ ..] => super::t11::n17904(s, out),
		['щ', s @ ..] => super::t11::n17905(s, out),
		_ => false,
	}
}

/// Words ending -ившийся.
pub(crate) fn n10393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9917);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17022);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					super::push(out, 23759);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 17023);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23760);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28476);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 28477);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28478);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 28479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23761);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23762);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23763);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31171);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31172);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32531);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31173);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32532);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 28481);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28482);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31174);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31175);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 32533);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 31176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28483);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23764);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 31178);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 31179);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 32534);
											s.is_empty()
										}
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
					super::push(out, 23765);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23766);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 23767);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 23768);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 23769);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 17024);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23770);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28486);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 28487);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31180);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31181);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32535);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 28488);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28489);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31182);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 32536);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33214);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31183);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32537);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28490);
							match s {
								[] => true,
								['с', 'ы', s @ ..] => {
									super::push(out, 31184);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32538);
											s.is_empty()
										}
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
					super::push(out, 23771);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 23772);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23773);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28491);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 28492);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28493);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28494);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23774);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23775);
					match s {
						[] => true,
						['л', 'с', s @ ..] => {
							super::push(out, 28495);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31186);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28496);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28497);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23776);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23777);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 23778);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 23779);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28498);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28499);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 31189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28500);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 28501);
							s.is_empty()
						}
						['ы', s @ ..] => super::t11::n29928(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t11::n17914(s, out),
		['е', s @ ..] => super::t11::n17915(s, out),
		['ж', s @ ..] => super::t11::n17916(s, out),
		['з', s @ ..] => super::t11::n17917(s, out),
		['л', s @ ..] => super::t11::n17918(s, out),
		['м', s @ ..] => super::t11::n17919(s, out),
		['н', s @ ..] => super::t11::n17920(s, out),
		['о', s @ ..] => super::t11::n17921(s, out),
		['п', s @ ..] => super::t11::n17922(s, out),
		['р', s @ ..] => super::t11::n17923(s, out),
		['с', s @ ..] => super::t11::n17924(s, out),
		['т', s @ ..] => super::t11::n17925(s, out),
		['ф', s @ ..] => super::t11::n17926(s, out),
		['ч', s @ ..] => super::t11::n17927(s, out),
		['ш', s @ ..] => super::t11::n17928(s, out),
		['щ', s @ ..] => super::t11::n17929(s, out),
		_ => false,
	}
}

/// Words ending -выдвинувшийся.
pub(crate) fn n34874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33224);
	s.is_empty()
}

/// Words ending -кнувшийся.
pub(crate) fn n25110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23888);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 28707);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31351);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 31352);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 28708);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 28709);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 28710);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 31354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 28711);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 28712);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 28713);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -апнувшийся.
pub(crate) fn n30153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28715);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 31356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнувшийся.
pub(crate) fn n25113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23891);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31357);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 32612);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 31358);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снувшийся.
pub(crate) fn n25114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23892);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28717);
			match s {
				[] => true,
				['л', 'п', 'ы', s @ ..] => {
					super::push(out, 31359);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28718);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 28719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тнувшийся.
pub(crate) fn n25115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23893);
	s.is_empty()
}

/// Words ending -унувшийся.
pub(crate) fn n25116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23894);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 28720);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 28721);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31362);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 31363);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хнувшийся.
pub(crate) fn n25117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23895);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31364);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 31365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', s @ ..] => {
			super::push(out, 28722);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 31366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 28723);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнувшийся.
pub(crate) fn n25118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23896);
	s.is_empty()
}

/// Words ending -ьнувшийся.
pub(crate) fn n25119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23897);
	s.is_empty()
}

/// Words ending -янувшийся.
pub(crate) fn n25120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23898);
	match s {
		[] => true,
		['т', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31368);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 31369);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывшийся.
pub(crate) fn n10396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9920);
	match s {
		[] => true,
		['м', 'ы', s @ ..] => {
			super::push(out, 17040);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явшийся.
pub(crate) fn n10397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9921);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17042);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23900);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23901);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 23902);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 23903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17043);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23904);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 23905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23906);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23907);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17044);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23909);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28728);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 31370);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32617);
											s.is_empty()
										}
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
					super::push(out, 23910);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23911);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 23912);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28729);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31371);
									s.is_empty()
								}
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
			super::push(out, 17045);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23913);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28730);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23914);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17046);
			match s {
				[] => true,
				['т', 'с', 'ы', s @ ..] => {
					super::push(out, 23916);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17047);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23917);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28733);
							match s {
								[] => true,
								['з', 'и', '\u{1}', s @ ..] => {
									super::push(out, 31373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23918);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ающийся.
pub(crate) fn n10423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9943);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17085);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 17086);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23945);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23946);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28738);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28739);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 31374);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 31375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23947);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28741);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 28742);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 28743);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 28744);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 28745);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 28746);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31377);
									s.is_empty()
								}
								['е', 'р', s @ ..] => {
									super::push(out, 31378);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32618);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33226);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 31379);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32619);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33227);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', 'р', s @ ..] => {
									super::push(out, 31380);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32620);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31381);
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									super::push(out, 31382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 28747);
							s.is_empty()
						}
						['л', s @ ..] => super::t11::n30189(s, out),
						['н', s @ ..] => {
							super::push(out, 28749);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28750);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28751);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31392);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31393);
									match s {
										[] => true,
										['п', s @ ..] => super::t11::n34248(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => super::t11::n30193(s, out),
						['ч', s @ ..] => super::t11::n30194(s, out),
						['ш', s @ ..] => super::t11::n30195(s, out),
						['щ', s @ ..] => super::t11::n30196(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t11::n25170(s, out),
				['у', s @ ..] => super::t11::n25171(s, out),
				['ы', s @ ..] => super::t11::n25172(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t11::n17984(s, out),
		['д', s @ ..] => super::t11::n17985(s, out),
		['ж', s @ ..] => super::t11::n17986(s, out),
		['з', s @ ..] => super::t11::n17987(s, out),
		['к', s @ ..] => super::t11::n17988(s, out),
		['л', s @ ..] => super::t11::n17989(s, out),
		['м', s @ ..] => super::t11::n17990(s, out),
		['н', s @ ..] => super::t11::n17991(s, out),
		['п', s @ ..] => super::t11::n17992(s, out),
		['р', s @ ..] => super::t11::n17993(s, out),
		['с', s @ ..] => super::t11::n17994(s, out),
		['т', s @ ..] => super::t11::n17995(s, out),
		['х', s @ ..] => super::t11::n17996(s, out),
		['ч', s @ ..] => super::t11::n17997(s, out),
		['ш', s @ ..] => super::t11::n17998(s, out),
		['щ', s @ ..] => super::t11::n17999(s, out),
		_ => false,
	}
}

/// Words ending -нирующийся.
pub(crate) fn n30271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28827);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31435);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пирующийся.
pub(crate) fn n30272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28828);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 31437);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рирующийся.
pub(crate) fn n30273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28829);
	s.is_empty()
}

/// Words ending -сирующийся.
pub(crate) fn n30274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28830);
	s.is_empty()
}

/// Words ending -тирующийся.
pub(crate) fn n30275(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31438);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33235);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 33236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 31439);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цирующийся.
pub(crate) fn n30276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28831);
	s.is_empty()
}

/// Words ending -трующийся.
pub(crate) fn n25254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24025);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 28832);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31440);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрующийся.
pub(crate) fn n25255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24026);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 28833);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 31441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сующийся.
pub(crate) fn n18016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17118);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24027);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 24028);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24029);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24030);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24031);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тующийся.
pub(crate) fn n18017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17119);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24032);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 24033);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хующийся.
pub(crate) fn n18018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17120);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чующийся.
pub(crate) fn n18019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17121);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24035);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шующийся.
pub(crate) fn n18020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17122);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 24036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьющийся.
pub(crate) fn n10429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9949);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17123);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 17124);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 17125);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юющийся.
pub(crate) fn n10430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9950);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17126);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яющийся.
pub(crate) fn n10431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9951);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17127);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 17128);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24038);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28837);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плящийся.
pub(crate) fn n18056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17154);
	s.is_empty()
}

/// Words ending -слящийся.
pub(crate) fn n18057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17155);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улящийся.
pub(crate) fn n18058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17156);
	s.is_empty()
}

/// Words ending -ылящийся.
pub(crate) fn n18059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17157);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 24072);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ялящийся.
pub(crate) fn n18060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17158);
	s.is_empty()
}

/// Words ending -мящийся.
pub(crate) fn n10439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9959);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24073);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17159);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нящийся.
pub(crate) fn n10440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9960);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24075);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24076);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17160);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 24077);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24079);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24080);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28860);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 17161);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17162);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 24082);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 24083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17163);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17164);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24085);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24086);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24087);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17165);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оящийся.
pub(crate) fn n10441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9961);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17166);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 17167);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17168);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24091);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24092);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пящийся.
pub(crate) fn n10442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9962);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17169);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17170);
			s.is_empty()
		}
		['у', 'т', s @ ..] => {
			super::push(out, 17171);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рящийся.
pub(crate) fn n10443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9963);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17172);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24096);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 17173);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24098);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24099);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 17174);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24100);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17175);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24102);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 24103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17176);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24105);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28869);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 31445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24106);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 24107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17177);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24109);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24110);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 24111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'з', 'у', 'п', '\u{1}', s @ ..] => {
			super::push(out, 17178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сящийся.
pub(crate) fn n10444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9964);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17179);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24112);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17180);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17181);
			match s {
				[] => true,
				['н', 'д', s @ ..] => {
					super::push(out, 24113);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 17182);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тящийся.
pub(crate) fn n10445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9965);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24115);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24116);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24117);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17183);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24118);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							super::push(out, 28875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17184);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24119);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 24120);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 28877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24121);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24122);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28879);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28880);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 24123);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17185);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					super::push(out, 24124);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17186);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17187);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 17188);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 17189);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28882);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уящийся.
pub(crate) fn n10446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9966);
	s.is_empty()
}

/// Words ending -одливавший.
pub(crate) fn n25805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24555);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 29043);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реливавший.
pub(crate) fn n18902(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24556);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 24557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зливавший.
pub(crate) fn n18903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17968);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24558);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иливавший.
pub(crate) fn n18904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17969);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 24560);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24561);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29045);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оливавший.
pub(crate) fn n18906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17971);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 24562);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 24563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29046);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 29047);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 29048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 24564);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24565);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29049);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24566);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пливавший.
pub(crate) fn n18907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17972);
	s.is_empty()
}

/// Words ending -асливавший.
pub(crate) fn n18908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17973);
	s.is_empty()
}

/// Words ending -тливавший.
pub(crate) fn n18909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17974);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уливавший.
pub(crate) fn n18910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17975);
	s.is_empty()
}

/// Words ending -ыливавший.
pub(crate) fn n18911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17976);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 24568);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 24569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яливавший.
pub(crate) fn n18912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17977);
	s.is_empty()
}

/// Words ending -нивавший.
pub(crate) fn n11392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10856);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17978);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 17979);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 17980);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17981);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 17982);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 17983);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17984);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пивавший.
pub(crate) fn n11393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10857);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 17985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ривавший.
pub(crate) fn n11394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10858);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17986);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24570);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 24571);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24572);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 24573);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 17987);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24574);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29052);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 29053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24575);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24576);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24577);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24578);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24579);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 24580);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 24581);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 17988);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17989);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17990);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24582);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24583);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 29056);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 29057);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 29058);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'у', s @ ..] => {
					super::push(out, 24584);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17991);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 17992);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 17993);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хивавший.
pub(crate) fn n11395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10859);
	s.is_empty()
}

/// Words ending -чивавший.
pub(crate) fn n11396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10860);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17994);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17995);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17996);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24586);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 29060);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24587);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24588);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24589);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 24590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шивавший.
pub(crate) fn n11397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10861);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17998);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 24591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 17999);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 18000);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 18001);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18002);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24595);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24596);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29063);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31496);
									s.is_empty()
								}
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
			super::push(out, 18003);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24597);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18004);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24598);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24599);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29065);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29066);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 29067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18005);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 18006);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18007);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 18008);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24602);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щивавший.
pub(crate) fn n11398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10862);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18009);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 18010);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24603);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29068);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31497);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твовавший.
pub(crate) fn n18951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18016);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 24609);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 24610);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29071);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 29072);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 29073);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31500);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32675);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 33241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 31501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 29074);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 29075);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31502);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32676);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 31503);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32677);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33242);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 32678);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 33243);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 31504);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32679);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 32680);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 32681);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32682);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33244);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 33245);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 33246);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 31505);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32683);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 32684);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33247);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 32685);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33248);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 33510);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 33249);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 32686);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 33250);
											s.is_empty()
										}
										['р', s @ ..] => super::t11::n34901(s, out),
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 32687);
									s.is_empty()
								}
								['с', s @ ..] => super::t11::n34306(s, out),
								['у', s @ ..] => super::t11::n34307(s, out),
								_ => false,
							}
						}
						['щ', s @ ..] => super::t11::n33055(s, out),
						_ => false,
					}
				}
				['й', s @ ..] => super::t11::n30526(s, out),
				['м', s @ ..] => {
					super::push(out, 29077);
					s.is_empty()
				}
				['н', s @ ..] => super::t11::n30528(s, out),
				['п', s @ ..] => super::t11::n30529(s, out),
				['р', s @ ..] => super::t11::n30530(s, out),
				['т', s @ ..] => super::t11::n30531(s, out),
				['ф', s @ ..] => super::t11::n30532(s, out),
				['ь', s @ ..] => super::t11::n30533(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едовавший.
pub(crate) fn n18959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18024);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 24615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 24616);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 24617);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24618);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29089);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 31518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ндовавший.
pub(crate) fn n18963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18028);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24622);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одовавший.
pub(crate) fn n18964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18029);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 24624);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24625);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 24626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдовавший.
pub(crate) fn n18965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18030);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удовавший.
pub(crate) fn n18966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18031);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 24628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядовавший.
pub(crate) fn n18967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18032);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 24629);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зовавший.
pub(crate) fn n11403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10866);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18033);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 18034);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24630);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18035);
			match s {
				[] => true,
				['л', 'л', s @ ..] => {
					super::push(out, 24631);
					match s {
						[] => true,
						['а', 'т', 'с', 'и', 'р', 'к', 'ы', s @ ..] => {
							super::push(out, 29094);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31519);
									s.is_empty()
								}
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
			super::push(out, 18036);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24632);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31520);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31521);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32715);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковавший.
pub(crate) fn n11404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10867);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18037);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24633);
					match s {
						[] => true,
						['б', 'ы', s @ ..] => {
							super::push(out, 29095);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31522);
									s.is_empty()
								}
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
			super::push(out, 18038);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 18039);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 24634);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 18040);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24635);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловавший.
pub(crate) fn n11405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10868);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24637);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24638);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 24639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18041);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 24640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18042);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24641);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18043);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 18044);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 18045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мовавший.
pub(crate) fn n11406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10869);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18046);
			match s {
				[] => true,
				['з', 'ы', s @ ..] => {
					super::push(out, 24643);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новавший.
pub(crate) fn n11407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10870);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18048);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 18049);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 18050);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 24644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18051);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 18052);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18053);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 18054);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18055);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 18056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -повавший.
pub(crate) fn n11408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10871);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 18057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровавший.
pub(crate) fn n11409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10872);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18058);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 24646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 18059);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18060);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18061);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24649);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n18998(s, out),
		['о', s @ ..] => {
			super::push(out, 18063);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24669);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24670);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 24671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18064);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24672);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 24673);
					match s {
						[] => true,
						['у', 'м', 'ы', s @ ..] => {
							super::push(out, 29168);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 24674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18065);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 18066);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 18067);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 24677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совавший.
pub(crate) fn n11410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10873);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18068);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 18069);
			match s {
				[] => true,
				['р', 'ы', s @ ..] => {
					super::push(out, 24678);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товавший.
pub(crate) fn n11411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10874);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18070);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24679);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24680);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18071);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24682);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24683);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24684);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18072);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 18073);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 18074);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18075);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24686);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18076);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 18077);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24687);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29173);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 31588);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32765);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 29174);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 18078);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 18079);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24688);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фовавший.
pub(crate) fn n11412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10875);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18080);
			match s {
				[] => true,
				['л', 'ш', 'ы', s @ ..] => {
					super::push(out, 24689);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 18081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховавший.
pub(crate) fn n11413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10876);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18082);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -анывавший.
pub(crate) fn n19062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18124);
	s.is_empty()
}

/// Words ending -енывавший.
pub(crate) fn n19063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18125);
	s.is_empty()
}

/// Words ending -знывавший.
pub(crate) fn n19064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18126);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унывавший.
pub(crate) fn n19065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18127);
	s.is_empty()
}

/// Words ending -пывавший.
pub(crate) fn n11424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10886);
	s.is_empty()
}

/// Words ending -рывавший.
pub(crate) fn n11425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10887);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18128);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24729);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 24730);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24731);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 24732);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 24733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 18129);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 18130);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 18131);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18132);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 18133);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18134);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 18135);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 24737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18136);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18137);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 18138);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 18139);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18140);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 18141);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сывавший.
pub(crate) fn n11426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10888);
	s.is_empty()
}

/// Words ending -тывавший.
pub(crate) fn n11427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10889);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18142);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 18143);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 18144);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 18145);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 18146);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 18147);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29188);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 29189);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24740);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24741);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29191);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29192);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31591);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32766);
											s.is_empty()
										}
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
					super::push(out, 24742);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29193);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29194);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24743);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24744);
					s.is_empty()
				}
				['ы', 'в', s @ ..] => {
					super::push(out, 24745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нькнувший.
pub(crate) fn n20353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19363);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -якнувший.
pub(crate) fn n12502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11905);
	s.is_empty()
}

/// Words ending -тывающий.
pub(crate) fn n12750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12145);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19641);
			match s {
				[] => true,
				['б', 'а', 'р', 'б', 'о', 'о', s @ ..] => {
					super::push(out, 25750);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29573);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 31681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
							super::push(out, 29574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 25751);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 31682);
									s.is_empty()
								}
								['о', 'е', s @ ..] => {
									super::push(out, 31683);
									match s {
										[] => true,
										['с', 'в', '\u{1}', s @ ..] => {
											super::push(out, 32777);
											s.is_empty()
										}
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
					super::push(out, 25752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19642);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 19643);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19644);
			match s {
				[] => true,
				['е', 'в', 'с', s @ ..] => {
					super::push(out, 25753);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29575);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31684);
									s.is_empty()
								}
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
			super::push(out, 19645);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25754);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29576);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 29577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25755);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 25756);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25757);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зкоэффективный.
pub(crate) fn n34961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33309);
	s.is_empty()
}

/// Words ending -окоэффективный.
pub(crate) fn n34962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33310);
	s.is_empty()
}

/// Words ending -экоэффективный.
pub(crate) fn n34963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33311);
	s.is_empty()
}

/// Words ending -езасвидетельствованный.
pub(crate) fn n34965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33313);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 33530);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вольствованный.
pub(crate) fn n34966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33314);
	s.is_empty()
}

/// Words ending -зованный.
pub(crate) fn n15214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14479);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21226);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 26587);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'б', 'о', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29979);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31858);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31859);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32843);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 29980);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31860);
									s.is_empty()
								}
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
			super::push(out, 21227);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 31861);
									s.is_empty()
								}
								['е', 'р', 'е', s @ ..] => {
									super::push(out, 31862);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32844);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', 'б', s @ ..] => {
									super::push(out, 31863);
									s.is_empty()
								}
								['к', 'и', s @ ..] => {
									super::push(out, 31864);
									s.is_empty()
								}
								['м', 'р', 'о', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32845);
											s.is_empty()
										}
										['ф', '-', s @ ..] => {
											super::push(out, 32846);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33315);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31865);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32847);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 32848);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 33316);
													s.is_empty()
												}
												['н', s @ ..] => {
													match s {
														[] => true,
														['е', s @ ..] => super::t11::n35197(s, out),
														['и', 'м', 'е', s @ ..] => super::t11::n35198(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32849);
											s.is_empty()
										}
										['т', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33317);
													s.is_empty()
												}
												['й', s @ ..] => {
													super::push(out, 33318);
													s.is_empty()
												}
												['н', 'е', 'ц', 'е', s @ ..] => {
													super::push(out, 33319);
													s.is_empty()
												}
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
							super::push(out, 29981);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31866);
									match s {
										[] => true,
										['и', 'ц', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33320);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 33321);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 29982);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							super::push(out, 29984);
							match s {
								[] => true,
								['р', 'о', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32850);
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33322);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 33323);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 32851);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 33324);
													s.is_empty()
												}
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
							super::push(out, 29985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29986);
							match s {
								[] => true,
								['т', 'ю', s @ ..] => {
									super::push(out, 31868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'т', s @ ..] => {
							super::push(out, 29987);
							match s {
								[] => true,
								['в', 'а', 'е', s @ ..] => super::t11::n33434(s, out),
								['к', s @ ..] => super::t11::n33435(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t11::n31465(s, out),
						['я', 'л', s @ ..] => super::t11::n31466(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => super::t11::n22305(s, out),
		_ => false,
	}
}

/// Words ending -кованный.
pub(crate) fn n15215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14480);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21229);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26589);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 29990);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26590);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 26591);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31875);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31876);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32853);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21230);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21231);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 26592);
					match s {
						[] => true,
						['б', 'у', 'п', 'о', 'е', s @ ..] => {
							super::push(out, 29991);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 26593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21232);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 21233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26594);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 29992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ц', s @ ..] => {
					super::push(out, 26595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21234);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21235);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21236);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29993);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 29994);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31879);
									s.is_empty()
								}
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
			super::push(out, 21237);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26596);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 26597);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лованный.
pub(crate) fn n15216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14481);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21238);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29995);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 29996);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									super::push(out, 31880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 26599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21239);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26600);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 26601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21240);
			match s {
				[] => true,
				['м', 'о', s @ ..] => {
					super::push(out, 26602);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 26603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 21241);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фмованный.
pub(crate) fn n22320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21243);
	s.is_empty()
}

/// Words ending -нованный.
pub(crate) fn n15218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14483);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 21244);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 21245);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21246);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29998);
							s.is_empty()
						}
						['и', 'е', 'н', s @ ..] => {
							super::push(out, 29999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31881);
									s.is_empty()
								}
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
			super::push(out, 21247);
			match s {
				[] => true,
				['р', 'а', 'м', 'а', s @ ..] => {
					super::push(out, 26608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21248);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21249);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21250);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пованный.
pub(crate) fn n15219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14484);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 21251);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30000);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21252);
			match s {
				[] => true,
				['а', 'т', 'ш', 'а', s @ ..] => {
					super::push(out, 26609);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рованный.
pub(crate) fn n15220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14485);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21253);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 26610);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 26611);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30002);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 31882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'з', s @ ..] => {
							super::push(out, 30003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21254);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26612);
					match s {
						[] => true,
						['л', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31883);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31884);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32854);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21255);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26613);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 30004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21256);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26614);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 26615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n22334(s, out),
		['о', s @ ..] => {
			super::push(out, 21258);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21259);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 26637);
					match s {
						[] => true,
						['л', 'и', 'ф', 'е', s @ ..] => {
							super::push(out, 30099);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31997);
									s.is_empty()
								}
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
			super::push(out, 21260);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26638);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 26639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 21261);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26640);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сованный.
pub(crate) fn n15221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14486);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21262);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26641);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 26642);
					match s {
						[] => true,
						['г', 'о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31998);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31999);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32000);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32001);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32989);
											s.is_empty()
										}
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
					super::push(out, 26643);
					s.is_empty()
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30100);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 30101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21263);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21264);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30102);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 30103);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30104);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32002);
									s.is_empty()
								}
								['р', 'п', 'е', s @ ..] => {
									super::push(out, 32003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 30105);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32004);
									s.is_empty()
								}
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
			super::push(out, 21265);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21266);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21267);
			match s {
				[] => true,
				['е', 'р', 'п', 'ы', s @ ..] => {
					super::push(out, 26645);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тованный.
pub(crate) fn n15222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14487);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21268);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 26646);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30107);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 26647);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21269);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 21270);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 21271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26648);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26649);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 30109);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32006);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32990);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 30110);
							match s {
								[] => true,
								['а', 'п', 'е', s @ ..] => {
									super::push(out, 32007);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32991);
											s.is_empty()
										}
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
					super::push(out, 26650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 21272);
			match s {
				[] => true,
				['е', 'ц', 'к', 'а', 'е', s @ ..] => {
					super::push(out, 26651);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 26652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21273);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21274);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32008);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30112);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 30113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 21275);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26653);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фованный.
pub(crate) fn n15223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14488);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 21276);
			match s {
				[] => true,
				['л', 'ш', 'ы', s @ ..] => {
					super::push(out, 26654);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хованный.
pub(crate) fn n15224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21277);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21278);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 26655);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цованный.
pub(crate) fn n15225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14490);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 21279);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 21280);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 21281);
			match s {
				[] => true,
				['л', 'а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30117);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 30118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ственный.
pub(crate) fn n15467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14717);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21533);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21534);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26843);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 26844);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 26845);
					match s {
						[] => true,
						['ч', 'о', s @ ..] => {
							super::push(out, 30198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21535);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26846);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 30199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32026);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 32027);
									match s {
										[] => true,
										['о', 'н', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33432);
													match s {
														[] => true,
														['и', s @ ..] => super::t11::n35256(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 33433);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['а', s @ ..] => {
									super::push(out, 32028);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 32029);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30200);
							match s {
								[] => true,
								['с', 'о', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32994);
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											super::push(out, 32995);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 30201);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26847);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30202);
							match s {
								[] => true,
								['з', 'и', 'о', 'р', 'п', s @ ..] => {
									match s {
										[] => true,
										['-', 'о', 'н', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33434);
													s.is_empty()
												}
												['ч', s @ ..] => {
													super::push(out, 33435);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 32996);
											s.is_empty()
										}
										['ж', s @ ..] => {
											super::push(out, 32997);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33436);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 32998);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33437);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 33438);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 32032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32033);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 32034);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32999);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'к', 'з', 'и', 'л', 'б', '\u{1}', s @ ..] => {
									super::push(out, 32035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 30204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21536);
			match s {
				[] => true,
				['д', s @ ..] => super::t11::n28181(s, out),
				['ж', s @ ..] => super::t11::n28182(s, out),
				['м', s @ ..] => super::t11::n28183(s, out),
				['т', s @ ..] => super::t11::n28184(s, out),
				['ч', s @ ..] => super::t11::n28185(s, out),
				['ш', s @ ..] => super::t11::n28186(s, out),
				['щ', s @ ..] => super::t11::n28187(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n22626(s, out),
		['й', s @ ..] => super::t11::n22627(s, out),
		['м', s @ ..] => super::t11::n22628(s, out),
		['н', s @ ..] => super::t11::n22629(s, out),
		['р', s @ ..] => super::t11::n22630(s, out),
		['с', s @ ..] => super::t11::n22631(s, out),
		['т', s @ ..] => super::t11::n22632(s, out),
		['ь', s @ ..] => super::t11::n22633(s, out),
		_ => false,
	}
}

/// Words ending -еревооруженный.
pub(crate) fn n35129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33467);
	s.is_empty()
}

/// Words ending -еренаправленный.
pub(crate) fn n35130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33468);
	s.is_empty()
}

/// Words ending -опреставленный.
pub(crate) fn n35131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33469);
	s.is_empty()
}

/// Words ending -копоставленный.
pub(crate) fn n35132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33470);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33603);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сопоставленный.
pub(crate) fn n35133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33471);
	s.is_empty()
}

/// Words ending -ивленный.
pub(crate) fn n15597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14844);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21738);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 21739);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27048);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21740);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21741);
			match s {
				[] => true,
				['к', 'с', 'и', s @ ..] => {
					super::push(out, 27051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21742);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27052);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 21743);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27053);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олвленный.
pub(crate) fn n22842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21744);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30365);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 30366);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овленный.
pub(crate) fn n15599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14846);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 21745);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21746);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27055);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27056);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30367);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 30368);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27057);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27058);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21747);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27059);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 30370);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33075);
											s.is_empty()
										}
										['с', 'о', 'в', 'е', s @ ..] => {
											super::push(out, 33076);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33472);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', 'е', s @ ..] => {
											super::push(out, 33077);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33473);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27060);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 27061);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27062);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21748);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27063);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21749);
			match s {
				[] => true,
				['о', 'г', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30374);
							match s {
								[] => true,
								['о', 'п', 'е', s @ ..] => {
									super::push(out, 32121);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33078);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 30375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твленный.
pub(crate) fn n15600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14847);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21750);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21751);
			match s {
				[] => true,
				['е', 'щ', s @ ..] => {
					super::push(out, 27065);
					match s {
						[] => true,
						['у', 'с', 'о', 'е', s @ ..] => {
							super::push(out, 30376);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явленный.
pub(crate) fn n15602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14849);
	match s {
		[] => true,
		['а', 'з', 'е', s @ ..] => {
			super::push(out, 21753);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21754);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27067);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27068);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 21755);
			match s {
				[] => true,
				['б', 'о', 'е', s @ ..] => {
					super::push(out, 27069);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27070);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 27071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 21756);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27072);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word осведомленный.
pub(crate) fn n35136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33474);
	s.is_empty()
}

/// Words ending -еосведомленный.
pub(crate) fn n35137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33475);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 33604);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ымленный.
pub(crate) fn n15629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14875);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21811);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьмленный.
pub(crate) fn n15630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14876);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 21812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ямленный.
pub(crate) fn n15631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14877);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 21813);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21814);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27121);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30404);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 30405);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнопромышленный.
pub(crate) fn n35138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33476);
	s.is_empty()
}

/// Words ending -агорастворенный.
pub(crate) fn n35140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33478);
	s.is_empty()
}

/// Words ending -ожесточенный.
pub(crate) fn n33819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32215);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33111);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -источенный.
pub(crate) fn n28811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27456);
	s.is_empty()
}

/// Words ending -тточенный.
pub(crate) fn n23304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22185);
	s.is_empty()
}

/// Words ending -ыточенный.
pub(crate) fn n23305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22186);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27457);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word иммунозащищенный.
pub(crate) fn n35143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33481);
	s.is_empty()
}

/// The word помехозащищенный.
pub(crate) fn n35144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33482);
	s.is_empty()
}

/// Words ending -отивационный.
pub(crate) fn n33870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32266);
	s.is_empty()
}

/// Words ending -ьтивационный.
pub(crate) fn n33871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32267);
	s.is_empty()
}

/// Words ending -хивационный.
pub(crate) fn n32181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30672);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 32268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --инновационный.
pub(crate) fn n28955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27593);
	s.is_empty()
}

/// Words ending -гационный.
pub(crate) fn n23528(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'г', 'а', s @ ..] => {
			super::push(out, 27594);
			s.is_empty()
		}
		['и', 'в', 'а', 'н', 'о', s @ ..] => {
			super::push(out, 27595);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30673);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дационный.
pub(crate) fn n23529(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27596);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 27597);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30674);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зационный.
pub(crate) fn n23530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22396);
	match s {
		[] => true,
		['а', 'г', 'е', 'д', 'о', s @ ..] => {
			super::push(out, 27598);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30675);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30676);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33121);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 33122);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 33123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33124);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33483);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 33605);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33606);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 33607);
													match s {
														[] => true,
														['е', s @ ..] => super::t11::n35322(s, out),
														_ => false,
													}
												}
												['е', s @ ..] => {
													super::push(out, 33608);
													match s {
														[] => true,
														['д', s @ ..] => super::t11::n35323(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 33609);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', 'и', 'ц', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 33484);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33485);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 33486);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 32269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'и', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32270);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30677);
					match s {
						[] => true,
						['о', 'и', s @ ..] => {
							super::push(out, 32272);
							s.is_empty()
						}
						['р', 'е', 'д', 'о', 'м', 'р', s @ ..] => {
							super::push(out, 32273);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30678);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32274);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30679);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32275);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33127);
									match s {
										[] => true,
										['и', 'р', 'п', 'е', s @ ..] => {
											super::push(out, 33487);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 33610);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 33128);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 33488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32276);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -циационный.
pub(crate) fn n23531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22397);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27599);
			s.is_empty()
		}
		_ => false,
	}
}

