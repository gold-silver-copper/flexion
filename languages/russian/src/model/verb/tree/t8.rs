use flexion::Path;

/// Words ending -опить.
pub(crate) fn n800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 789);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3221);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8651);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8652);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8653);
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
		['д', s @ ..] => {
			super::push(out, 3223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8654);
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
		['ж', s @ ..] => {
			super::push(out, 3224);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8655);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8656);
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
					super::push(out, 8657);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8658);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23725);
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
		['з', s @ ..] => {
			super::push(out, 3225);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8659);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16029);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3226);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8660);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8661);
					match s {
						[] => true,
						['н', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8662);
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
					super::push(out, 8663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16030);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3227);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8664);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16032);
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
		['п', s @ ..] => {
			super::push(out, 3228);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3229);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8666);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16033);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23726);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 23727);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31027);
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
									super::push(out, 23728);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31028);
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
				['к', s @ ..] => {
					super::push(out, 8667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16034);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16035);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16036);
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
							super::push(out, 16037);
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
				['о', s @ ..] => {
					super::push(out, 8668);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16038);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23729);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23730);
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
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16040);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16041);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8671);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16042);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23732);
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
		['с', s @ ..] => super::t13::n3422(s, out),
		['т', s @ ..] => super::t13::n3423(s, out),
		['ш', s @ ..] => super::t13::n3424(s, out),
		_ => false,
	}
}

/// Words ending -ппить.
pub(crate) fn n801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 790);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3233);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8682);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпить.
pub(crate) fn n802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 791);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3234);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спить.
pub(crate) fn n803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 792);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3235);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3236);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тпить.
pub(crate) fn n804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 793);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упить.
pub(crate) fn n805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 794);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3239);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3240);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8686);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 16047);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23738);
									match s {
										[] => true,
										['с', s @ ..] => {
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
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 16048);
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
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8688);
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
				['ы', s @ ..] => {
					super::push(out, 8689);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8690);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8691);
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
					super::push(out, 8692);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 8694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16050);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16051);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16052);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23739);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23740);
									match s {
										[] => true,
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
							super::push(out, 16053);
							match s {
								[] => true,
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
					super::push(out, 8695);
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
				['е', s @ ..] => {
					super::push(out, 8696);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16054);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23741);
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
					super::push(out, 8697);
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
				['и', s @ ..] => {
					super::push(out, 8698);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8699);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16055);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 23742);
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
						['п', s @ ..] => {
							super::push(out, 16056);
							match s {
								[] => true,
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
					super::push(out, 8700);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16057);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23743);
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
				['с', s @ ..] => {
					super::push(out, 8701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8702);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16058);
							match s {
								[] => true,
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
					super::push(out, 8703);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16059);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8704);
					match s {
						[] => true,
						['р', 'т', 'н', 'о', 'к', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t13::n3436(s, out),
		['с', s @ ..] => super::t13::n3437(s, out),
		['т', s @ ..] => super::t13::n3438(s, out),
		_ => false,
	}
}

/// Words ending -ыпить.
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 795);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3247);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16078);
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
			super::push(out, 3248);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япить.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 796);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3249);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 8718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16079);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16080);
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

/// Words ending -арить.
pub(crate) fn n808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 797);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3250);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8719);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16081);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23760);
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
						['х', s @ ..] => {
							super::push(out, 16082);
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
		['в', s @ ..] => {
			super::push(out, 3251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8720);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8721);
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
					super::push(out, 8722);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8724);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16083);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23761);
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
							super::push(out, 16084);
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
					super::push(out, 8725);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16085);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 16086);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23762);
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
				['з', s @ ..] => {
					super::push(out, 8726);
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
					super::push(out, 8727);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8728);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 16087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23763);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23764);
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
					super::push(out, 8729);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16088);
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
						['м', s @ ..] => {
							super::push(out, 16089);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23765);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31031);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 16090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16091);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23766);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31032);
											match s {
												[] => true,
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
									super::push(out, 23767);
									match s {
										[] => true,
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
							super::push(out, 16092);
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
				['с', s @ ..] => {
					super::push(out, 8730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16093);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23768);
									match s {
										[] => true,
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
				['т', s @ ..] => {
					super::push(out, 8731);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16094);
							match s {
								[] => true,
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
					super::push(out, 8732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8733);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3252);
			match s {
				[] => true,
				['е', 'ч', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16096);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16097);
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
				['и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16098);
							s.is_empty()
						}
						['а', s @ ..] => super::t13::n22268(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t14::n3445(s, out),
		['ж', s @ ..] => super::t14::n3446(s, out),
		['з', s @ ..] => super::t14::n3447(s, out),
		['и', s @ ..] => super::t14::n3448(s, out),
		['к', s @ ..] => super::t14::n3449(s, out),
		['м', s @ ..] => super::t14::n3450(s, out),
		['н', s @ ..] => super::t14::n3451(s, out),
		['п', s @ ..] => super::t14::n3452(s, out),
		['с', s @ ..] => super::t14::n3453(s, out),
		['т', s @ ..] => super::t14::n3454(s, out),
		['у', s @ ..] => super::t14::n3455(s, out),
		['ф', s @ ..] => super::t14::n3456(s, out),
		['х', s @ ..] => super::t14::n3457(s, out),
		['ц', s @ ..] => super::t14::n3458(s, out),
		['ч', s @ ..] => super::t14::n3459(s, out),
		['ш', s @ ..] => super::t14::n3460(s, out),
		_ => false,
	}
}

/// Words ending -осеребрить.
pub(crate) fn n52778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31049);
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

/// Words ending -ысеребрить.
pub(crate) fn n52779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31050);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 36372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оребрить.
pub(crate) fn n22374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16159);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сребрить.
pub(crate) fn n22375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16160);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23808);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ибрить.
pub(crate) fn n3465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3273);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 8807);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16161);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обрить.
pub(crate) fn n3466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8808);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16163);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16164);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23810);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31051);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16165);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16166);
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
				['с', s @ ..] => {
					super::push(out, 16167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 16168);
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
		['п', s @ ..] => {
			super::push(out, 8810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8811);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сбрить.
pub(crate) fn n3467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3275);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тбрить.
pub(crate) fn n3468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3276);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8812);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убрить.
pub(crate) fn n3469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3277);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 8813);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16170);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16171);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23813);
							match s {
								[] => true,
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
					super::push(out, 16172);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23814);
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
					super::push(out, 16173);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23815);
							match s {
								[] => true,
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
					super::push(out, 16174);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23816);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23817);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31052);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16175);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23818);
							match s {
								[] => true,
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
					super::push(out, 16176);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23819);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбрить.
pub(crate) fn n3470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3278);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8814);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16177);
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

/// Words ending -ябрить.
pub(crate) fn n3471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3279);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 8816);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8817);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16178);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31053);
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
		_ => false,
	}
}

/// Words ending -врить.
pub(crate) fn n810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 799);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3280);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16179);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16180);
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

/// Words ending -грить.
pub(crate) fn n811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 800);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8818);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8819);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16181);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16182);
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
							super::push(out, 16183);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16184);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23820);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23821);
									match s {
										[] => true,
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
							super::push(out, 16185);
							match s {
								[] => true,
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
					super::push(out, 8820);
					match s {
						[] => true,
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
			super::push(out, 3282);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16186);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 16187);
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

/// Words ending -дрить.
pub(crate) fn n812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 801);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3283);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16188);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16189);
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
		['е', s @ ..] => {
			super::push(out, 3284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8823);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 8824);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16190);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23822);
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
							super::push(out, 16191);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8825);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3285);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8826);
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
		['н', s @ ..] => {
			super::push(out, 3286);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8827);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23824);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23825);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23826);
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
			super::push(out, 3287);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16193);
							s.is_empty()
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
							super::push(out, 16194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', s @ ..] => {
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
		['у', s @ ..] => {
			super::push(out, 3288);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16195);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16196);
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
							super::push(out, 16197);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23827);
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
							super::push(out, 16198);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 16199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 16200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 16201);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23828);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16202);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16203);
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
							super::push(out, 16204);
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

/// Words ending -ерить.
pub(crate) fn n813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 802);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3289);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8831);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 16205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3290);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8832);
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
		['в', s @ ..] => {
			super::push(out, 3291);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8833);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8834);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 16207);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8836);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16208);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 16209);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23830);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8839);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16212);
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
						['п', s @ ..] => {
							super::push(out, 16213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16214);
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
				['с', s @ ..] => {
					super::push(out, 8840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8841);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16215);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31054);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31055);
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
				['у', s @ ..] => {
					super::push(out, 8842);
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
				['ы', s @ ..] => {
					super::push(out, 8843);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16216);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23831);
									s.is_empty()
								}
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
			super::push(out, 3292);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8844);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16217);
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
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3293);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8845);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16218);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31056);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 31057);
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
					super::push(out, 8846);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16219);
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
			super::push(out, 3294);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8847);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 16220);
							match s {
								[] => true,
								['е', 'п', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3295);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8848);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23832);
									s.is_empty()
								}
								['б', s @ ..] => super::t14::n37327(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t14::n22463(s, out),
						_ => false,
					}
				}
				['ю', s @ ..] => super::t14::n10697(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t14::n3488(s, out),
		['л', s @ ..] => super::t14::n3489(s, out),
		['м', s @ ..] => super::t14::n3490(s, out),
		['н', s @ ..] => super::t14::n3491(s, out),
		['п', s @ ..] => super::t14::n3492(s, out),
		['с', s @ ..] => super::t14::n3493(s, out),
		['т', s @ ..] => super::t14::n3494(s, out),
		['ф', s @ ..] => super::t14::n3495(s, out),
		['х', s @ ..] => super::t14::n3496(s, out),
		['ч', s @ ..] => super::t14::n3497(s, out),
		['ш', s @ ..] => super::t14::n3498(s, out),
		['щ', s @ ..] => super::t14::n3499(s, out),
		['ь', s @ ..] => super::t14::n3500(s, out),
		_ => false,
	}
}

/// Words ending -зрить.
pub(crate) fn n814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 803);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3309);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3310);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3311);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирить.
pub(crate) fn n815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 804);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3313);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8903);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 16265);
							match s {
								[] => true,
								['и', 'р', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8904);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 16266);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
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
				['р', s @ ..] => {
					super::push(out, 8905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16267);
							match s {
								[] => true,
								['б', 'м', 'о', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3314);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8906);
					match s {
						[] => true,
						['а', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23863);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23864);
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
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3315);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8907);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8908);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8909);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16268);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23865);
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
					super::push(out, 8910);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8911);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 16270);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 23866);
									match s {
										[] => true,
										['т', 'у', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8912);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				['у', s @ ..] => {
					super::push(out, 8913);
					match s {
						[] => true,
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
			super::push(out, 3316);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8914);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16271);
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
		['ф', s @ ..] => {
			super::push(out, 3317);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8915);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 16272);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8916);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8917);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16273);
							match s {
								[] => true,
								['е', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31076);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31077);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31078);
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
					super::push(out, 8918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16274);
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
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8919);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -крить.
pub(crate) fn n816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 805);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3319);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16275);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16276);
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

/// Words ending -орить.
pub(crate) fn n817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 806);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3320);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 8921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16277);
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
				['с', s @ ..] => {
					super::push(out, 8922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16278);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 16279);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23867);
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
							super::push(out, 16280);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23868);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3321);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8923);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16281);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23869);
									match s {
										[] => true,
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
							super::push(out, 16282);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8924);
					match s {
						[] => true,
						['г', s @ ..] => super::t14::n22558(s, out),
						['р', s @ ..] => {
							super::push(out, 16284);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23882);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31088);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31089);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 31090);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t14::n10778(s, out),
				['т', s @ ..] => super::t14::n10779(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t14::n3514(s, out),
		['д', s @ ..] => super::t14::n3515(s, out),
		['ж', s @ ..] => super::t14::n3516(s, out),
		['з', s @ ..] => super::t14::n3517(s, out),
		['к', s @ ..] => super::t14::n3518(s, out),
		['м', s @ ..] => super::t14::n3519(s, out),
		['н', s @ ..] => super::t14::n3520(s, out),
		['п', s @ ..] => super::t14::n3521(s, out),
		['с', s @ ..] => super::t14::n3522(s, out),
		['т', s @ ..] => super::t14::n3523(s, out),
		['ф', s @ ..] => super::t14::n3524(s, out),
		['х', s @ ..] => super::t14::n3525(s, out),
		['ш', s @ ..] => {
			super::push(out, 3334);
			match s {
				[] => true,
				['а', s @ ..] => super::t14::n10834(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трить.
pub(crate) fn n818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 807);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3335);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16351);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 16352);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16353);
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
						['ы', s @ ..] => {
							super::push(out, 16354);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23934);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16355);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16356);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23935);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3336);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 8983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16357);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16358);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 23936);
									match s {
										[] => true,
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
							super::push(out, 16359);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23937);
									match s {
										[] => true,
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
							super::push(out, 16360);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23938);
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
							super::push(out, 16361);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23939);
									match s {
										[] => true,
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
							super::push(out, 16362);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 16363);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23940);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3337);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8984);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23941);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23942);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31116);
											match s {
												[] => true,
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
									super::push(out, 23943);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31117);
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
		['с', s @ ..] => {
			super::push(out, 3338);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8985);
					match s {
						[] => true,
						['п', s @ ..] => super::t14::n22663(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t14::n10840(s, out),
				['у', s @ ..] => super::t14::n10841(s, out),
				['ы', s @ ..] => super::t14::n10842(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урить.
pub(crate) fn n819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 808);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8989);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8990);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23962);
									s.is_empty()
								}
								_ => false,
							}
						}
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
					super::push(out, 8991);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16377);
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
					super::push(out, 8992);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16378);
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
				['м', s @ ..] => {
					super::push(out, 8993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16379);
							match s {
								[] => true,
								['л', 'а', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31120);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 16380);
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
					super::push(out, 8994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16381);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 16382);
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
					super::push(out, 8995);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16383);
							match s {
								[] => true,
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
					super::push(out, 8996);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16384);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3340);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8997);
					match s {
						[] => true,
						['л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23963);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23964);
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
									super::push(out, 23965);
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
			super::push(out, 3341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8998);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8999);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 16386);
							match s {
								[] => true,
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
					super::push(out, 9000);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16387);
							match s {
								[] => true,
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
					super::push(out, 9001);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16388);
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
					super::push(out, 9002);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16389);
							match s {
								[] => true,
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
					super::push(out, 9003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16390);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 16391);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23966);
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
							super::push(out, 16392);
							match s {
								[] => true,
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
					super::push(out, 9004);
					match s {
						[] => true,
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
			super::push(out, 3342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9005);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9006);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16393);
							match s {
								[] => true,
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
					super::push(out, 9007);
					match s {
						[] => true,
						['д', s @ ..] => super::t14::n22698(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9008);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16395);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t14::n10863(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t14::n3535(s, out),
		['к', s @ ..] => super::t14::n3536(s, out),
		['м', s @ ..] => super::t14::n3537(s, out),
		['н', s @ ..] => super::t14::n3538(s, out),
		['п', s @ ..] => super::t14::n3539(s, out),
		['т', s @ ..] => super::t14::n3540(s, out),
		['ш', s @ ..] => super::t14::n3541(s, out),
		['щ', s @ ..] => super::t14::n3542(s, out),
		_ => false,
	}
}

/// Words ending -фрить.
pub(crate) fn n820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 809);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3351);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9045);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16436);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16437);
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

/// Words ending -хрить.
pub(crate) fn n821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 810);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3352);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9046);
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
		['и', s @ ..] => {
			super::push(out, 3353);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9047);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16438);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16439);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 16440);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 16441);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23980);
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
			super::push(out, 3354);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9048);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9049);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3355);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щрить.
pub(crate) fn n822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 811);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ырить.
pub(crate) fn n823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 812);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9051);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9052);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16443);
							match s {
								[] => true,
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
		['д', s @ ..] => {
			super::push(out, 3358);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9053);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16444);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23981);
									match s {
										[] => true,
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
							super::push(out, 16445);
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
		['з', s @ ..] => {
			super::push(out, 3359);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9054);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9055);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23982);
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
								['с', s @ ..] => {
									super::push(out, 23983);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 31136);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36397);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3360);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16446);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
		['п', s @ ..] => {
			super::push(out, 3361);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16447);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16448);
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
						['т', s @ ..] => {
							super::push(out, 16449);
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
		['т', s @ ..] => {
			super::push(out, 3362);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9056);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9057);
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
					super::push(out, 9058);
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
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23984);
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
				['у', 'б', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16450);
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
		['ф', 'у', 'ф', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрить.
pub(crate) fn n824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 813);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3363);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9059);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23985);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'п', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3365);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9061);
					match s {
						[] => true,
						['м', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3366);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9062);
					match s {
						[] => true,
						['н', 'а', 'м', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3367);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9063);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16451);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16452);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярить.
pub(crate) fn n825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 814);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3368);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3369);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9064);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16453);
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
		['л', s @ ..] => {
			super::push(out, 3370);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9065);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16454);
							match s {
								[] => true,
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
					super::push(out, 9066);
					match s {
						[] => true,
						['и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23987);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23988);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9067);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16455);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31137);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31138);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 16456);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23989);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3371);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9068);
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
		['у', s @ ..] => {
			super::push(out, 3372);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 9069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16457);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23990);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 23991);
									match s {
										[] => true,
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
							super::push(out, 16458);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23992);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23993);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 23994);
									match s {
										[] => true,
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
							super::push(out, 16459);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23995);
									match s {
										[] => true,
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
							super::push(out, 16460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3373);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9070);
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
		['ь', s @ ..] => {
			super::push(out, 3374);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9071);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16461);
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

/// Words ending -заколбасить.
pub(crate) fn n66402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36398);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -наколбасить.
pub(crate) fn n66403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36399);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -васить.
pub(crate) fn n3569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3376);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9078);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16467);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16468);
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
				['о', s @ ..] => {
					super::push(out, 16469);
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
					super::push(out, 16470);
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
					super::push(out, 16471);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23998);
							match s {
								[] => true,
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
					super::push(out, 16472);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гасить.
pub(crate) fn n3570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3377);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9079);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9080);
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
			super::push(out, 9081);
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

/// Words ending -вкасить.
pub(crate) fn n10947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9082);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16473);
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

/// Words ending -ласить.
pub(crate) fn n3572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3379);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9083);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16474);
					match s {
						[] => true,
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
			super::push(out, 9084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16475);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 16476);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 24000);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31141);
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

/// Words ending -масить.
pub(crate) fn n3573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3380);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9085);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16478);
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
		_ => false,
	}
}

/// Words ending -пасить.
pub(crate) fn n3574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3381);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9086);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16479);
					match s {
						[] => true,
						['е', 'б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 24001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9087);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -расить.
pub(crate) fn n3575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3382);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9088);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16481);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16482);
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
				['и', s @ ..] => {
					super::push(out, 16483);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24002);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31142);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 16484);
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
					super::push(out, 16485);
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
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16486);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24003);
							match s {
								[] => true,
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
					super::push(out, 16487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 16488);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31143);
									s.is_empty()
								}
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
			super::push(out, 9089);
			match s {
				[] => true,
				['д', 'и', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24005);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24006);
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
							super::push(out, 24007);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24008);
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

/// Words ending -уасить.
pub(crate) fn n3576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3383);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 9090);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16489);
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

/// Words ending -есить.
pub(crate) fn n827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 816);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3384);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9091);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9092);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 16490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16491);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									super::push(out, 24009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 16492);
							match s {
								[] => true,
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
					super::push(out, 9094);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16493);
							match s {
								[] => true,
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
					super::push(out, 9095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9096);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16494);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9097);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9098);
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
								['а', 'з', '\u{1}', s @ ..] => {
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9099);
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
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9100);
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
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9101);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16495);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3386);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9102);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16496);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24010);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31144);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 16497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24011);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24012);
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
		['л', s @ ..] => {
			super::push(out, 3387);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9103);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16499);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 24013);
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
				['б', s @ ..] => {
					super::push(out, 9104);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16500);
							match s {
								[] => true,
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
					super::push(out, 9105);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16501);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24014);
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
				['о', s @ ..] => super::t14::n10978(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t14::n3581(s, out),
		['р', s @ ..] => {
			super::push(out, 3389);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9112);
					match s {
						[] => true,
						['с', s @ ..] => super::t14::n22877(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -исить.
pub(crate) fn n828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 817);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йсить.
pub(crate) fn n829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 818);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3391);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ксить.
pub(crate) fn n830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 819);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3392);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16506);
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
		['е', s @ ..] => {
			super::push(out, 3393);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9115);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24021);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24022);
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
					super::push(out, 9116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3394);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 9117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16508);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16509);
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
			super::push(out, 3395);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9118);
					match s {
						[] => true,
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
			super::push(out, 3396);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16510);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мсить.
pub(crate) fn n831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3397);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нсить.
pub(crate) fn n832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3398);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9120);
					match s {
						[] => true,
						['и', 'р', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3399);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9121);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24023);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24024);
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
		['э', s @ ..] => {
			super::push(out, 3400);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осить.
pub(crate) fn n833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 822);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3401);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9123);
					match s {
						[] => true,
						['у', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24025);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24026);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31153);
											match s {
												[] => true,
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
									super::push(out, 24027);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31154);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 31155);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36402);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n76866(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9124);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24028);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24029);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31156);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9125);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9126);
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
					super::push(out, 9127);
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
				['е', s @ ..] => {
					super::push(out, 9128);
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
					super::push(out, 9129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 16513);
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
						['н', s @ ..] => {
							super::push(out, 16514);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24030);
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
							super::push(out, 16515);
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
					super::push(out, 9130);
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
				['ы', s @ ..] => {
					super::push(out, 9131);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16516);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3403);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9132);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16517);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24031);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31157);
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
									super::push(out, 24032);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 31158);
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
						['г', s @ ..] => {
							super::push(out, 16518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24033);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24034);
									match s {
										[] => true,
										['н', s @ ..] => super::t14::n53081(s, out),
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => super::t14::n37737(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t14::n22912(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t14::n3597(s, out),
		['р', s @ ..] => super::t14::n3598(s, out),
		['с', s @ ..] => super::t14::n3599(s, out),
		_ => false,
	}
}

/// Words ending -псить.
pub(crate) fn n834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 823);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 3407);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16554);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсить.
pub(crate) fn n835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 824);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3408);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9152);
					match s {
						[] => true,
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
			super::push(out, 3409);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16556);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16557);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24078);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9154);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16558);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 24079);
									match s {
										[] => true,
										['а', 'р', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16559);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16560);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 24080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16561);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 31186);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 36409);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n76874(s, out),
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
									super::push(out, 24081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 16562);
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

/// Words ending -ссить.
pub(crate) fn n836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3410);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9156);
					match s {
						[] => true,
						['о', 'л', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24082);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24083);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9157);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16563);
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
		_ => false,
	}
}

/// Words ending -усить.
pub(crate) fn n837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 826);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3411);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9159);
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
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 9160);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16564);
							match s {
								[] => true,
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
					super::push(out, 9161);
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
				['д', s @ ..] => {
					super::push(out, 9162);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16565);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24084);
									match s {
										[] => true,
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
							super::push(out, 16566);
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
					super::push(out, 9163);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9164);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16567);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 16568);
							match s {
								[] => true,
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
					super::push(out, 9166);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9168);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16569);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3413);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16570);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16571);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16572);
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
				['и', s @ ..] => {
					super::push(out, 9170);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3414);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9171);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16574);
							match s {
								[] => true,
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
					super::push(out, 9172);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16575);
							match s {
								[] => true,
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
					super::push(out, 9173);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16576);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24085);
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
				['т', s @ ..] => {
					super::push(out, 9174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16577);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16578);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24086);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31187);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 24087);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => super::t14::n22997(s, out),
						['д', s @ ..] => super::t14::n22998(s, out),
						['е', s @ ..] => super::t14::n22999(s, out),
						['и', s @ ..] => super::t14::n23000(s, out),
						['о', s @ ..] => super::t14::n23001(s, out),
						['с', s @ ..] => super::t14::n23002(s, out),
						['у', s @ ..] => {
							super::push(out, 16585);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t14::n37839(s, out),
								_ => false,
							}
						}
						['ы', s @ ..] => super::t14::n23004(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t14::n3609(s, out),
		_ => false,
	}
}

/// Words ending -ысить.
pub(crate) fn n838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 827);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9178);
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
				['з', 'о', 'в', s @ ..] => {
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3417);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9179);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9180);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16590);
							match s {
								[] => true,
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
					super::push(out, 9181);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16591);
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
					super::push(out, 9182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16592);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16593);
							match s {
								[] => true,
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
					super::push(out, 9183);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16594);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16595);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24097);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эсить.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 828);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3418);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 9184);
					match s {
						[] => true,
						['с', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атить.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 829);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3419);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9185);
					match s {
						[] => true,
						['о', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24098);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24099);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9186);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16596);
							match s {
								[] => true,
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
					super::push(out, 9187);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16597);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24100);
									match s {
										[] => true,
										['в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36410);
													s.is_empty()
												}
												['б', s @ ..] => {
													super::push(out, 36411);
													match s {
														[] => true,
														['о', '\u{1}', s @ ..] => super::t14::n76875(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9188);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16598);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24101);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31191);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 24102);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16599);
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
							super::push(out, 16600);
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
							super::push(out, 16601);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9189);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9190);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16602);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 16603);
							match s {
								[] => true,
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
					super::push(out, 9191);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16604);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24104);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24105);
									match s {
										[] => true,
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
							super::push(out, 16605);
							match s {
								[] => true,
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
					super::push(out, 9192);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16606);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3422);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9193);
					match s {
						[] => true,
						['г', 'а', 'л', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3423);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9194);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16607);
							match s {
								[] => true,
								['л', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9195);
					s.is_empty()
				}
				['а', s @ ..] => super::t14::n11090(s, out),
				['б', s @ ..] => {
					super::push(out, 9197);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => super::t14::n23042(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t14::n11092(s, out),
				['д', s @ ..] => super::t14::n11093(s, out),
				['е', s @ ..] => super::t14::n11094(s, out),
				['и', s @ ..] => super::t14::n11095(s, out),
				['о', s @ ..] => super::t14::n11096(s, out),
				['с', s @ ..] => super::t14::n11097(s, out),
				['т', s @ ..] => super::t14::n11098(s, out),
				['у', s @ ..] => super::t14::n11099(s, out),
				['ы', s @ ..] => super::t14::n11100(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t14::n3619(s, out),
		['м', s @ ..] => super::t14::n3620(s, out),
		['н', s @ ..] => super::t14::n3621(s, out),
		['о', s @ ..] => super::t14::n3622(s, out),
		['п', s @ ..] => super::t14::n3623(s, out),
		['р', s @ ..] => super::t14::n3624(s, out),
		['х', s @ ..] => super::t14::n3625(s, out),
		_ => false,
	}
}

/// Words ending -ветить.
pub(crate) fn n3629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3435);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9232);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16653);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9233);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16654);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16655);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24136);
							match s {
								[] => true,
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
					super::push(out, 16656);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24137);
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
					super::push(out, 16657);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16658);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 24138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 24139);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 31217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 24140);
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
				['с', s @ ..] => {
					super::push(out, 16660);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24141);
							match s {
								[] => true,
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
					super::push(out, 16662);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24142);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9234);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24143);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31218);
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
		['ц', s @ ..] => {
			super::push(out, 9235);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16664);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16665);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24144);
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
					super::push(out, 16666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16667);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24145);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24146);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31219);
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
				['т', s @ ..] => {
					super::push(out, 16668);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24147);
							match s {
								[] => true,
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
					super::push(out, 16669);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -метить.
pub(crate) fn n3632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3438);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9240);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9241);
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
			super::push(out, 9242);
			match s {
				[] => true,
				['е', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 9243);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9244);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24149);
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
			super::push(out, 9245);
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
		['т', s @ ..] => {
			super::push(out, 9246);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9247);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нетить.
pub(crate) fn n3633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3439);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9248);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16674);
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
		['и', s @ ..] => {
			super::push(out, 9249);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16675);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24150);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24151);
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

/// Words ending -петить.
pub(crate) fn n3634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3440);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9250);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16676);
					match s {
						[] => true,
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
			super::push(out, 9251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретить.
pub(crate) fn n3635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3441);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9252);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 16678);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24152);
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
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9253);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16679);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16680);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16681);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16682);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9255);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9256);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16683);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сетить.
pub(crate) fn n3636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3442);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16684);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уетить.
pub(crate) fn n3637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3443);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 9258);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16686);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16687);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16688);
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

/// Words ending -фетить.
pub(crate) fn n3638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3444);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9259);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16689);
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

/// Words ending -шетить.
pub(crate) fn n3639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3445);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9260);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24153);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24154);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31220);
									match s {
										[] => true,
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
							super::push(out, 24155);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31221);
									match s {
										[] => true,
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
							super::push(out, 24156);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31222);
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
							super::push(out, 24157);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31223);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36421);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9261);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16690);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31224);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31225);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31226);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36422);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щетить.
pub(crate) fn n3640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3446);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 9262);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итить.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 832);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3447);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16692);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16693);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24158);
									match s {
										[] => true,
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
							super::push(out, 16694);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24159);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3448);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9264);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16695);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24160);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24161);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9265);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16696);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31227);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31228);
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3449);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9266);
					match s {
						[] => true,
						['н', 'о', 'м', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3450);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9267);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24162);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24163);
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
									super::push(out, 24164);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31229);
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
									super::push(out, 24165);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 24166);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31230);
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
									super::push(out, 24167);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3451);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9268);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9269);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9270);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16697);
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
						['о', s @ ..] => {
							super::push(out, 16698);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31231);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 31232);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 36423);
													match s {
														[] => true,
														['р', 'п', '\u{1}', s @ ..] => super::t14::n76886(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
		['щ', s @ ..] => {
			super::push(out, 3452);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтить.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 833);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3453);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9273);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16699);
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
		['е', s @ ..] => {
			super::push(out, 3454);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9274);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16700);
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
				['х', s @ ..] => {
					super::push(out, 9275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктить.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3455);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 9276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3456);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9277);
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

/// Words ending -лтить.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 835);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3457);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 9278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16702);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16703);
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
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 16704);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24168);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3458);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9279);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16705);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24169);
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
							super::push(out, 16706);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24170);
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
							super::push(out, 16707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтить.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 836);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3459);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9280);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16708);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31233);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31234);
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
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24171);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24172);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 24173);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31235);
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
									super::push(out, 24174);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24175);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31236);
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
		['е', s @ ..] => {
			super::push(out, 3460);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9281);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 16709);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3461);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16710);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16711);
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
							super::push(out, 16712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 16713);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 16714);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 16715);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 24176);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
							super::push(out, 16716);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16717);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 24177);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 24178);
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
							super::push(out, 16718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 16719);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 16720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 16721);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24179);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9283);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16722);
							match s {
								[] => true,
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
					super::push(out, 9284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16723);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16724);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24180);
									match s {
										[] => true,
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
							super::push(out, 16725);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24181);
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

/// Words ending -отить.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 837);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3462);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9285);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24182);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24183);
									match s {
										[] => true,
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
							super::push(out, 16727);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9286);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 16728);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3463);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9287);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16729);
							match s {
								[] => true,
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
					super::push(out, 9288);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 16730);
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
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3464);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9289);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16731);
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
					super::push(out, 9290);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16732);
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
				['я', s @ ..] => {
					super::push(out, 9291);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24185);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24186);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3465);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9292);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 9293);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16734);
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
		['к', s @ ..] => {
			super::push(out, 3466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9294);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9295);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 16735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24187);
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
					super::push(out, 9296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24188);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 24189);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31237);
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
		['л', s @ ..] => {
			super::push(out, 3467);
			match s {
				[] => true,
				['г', s @ ..] => super::t14::n11196(s, out),
				['о', s @ ..] => super::t14::n11197(s, out),
				['п', s @ ..] => super::t14::n11198(s, out),
				['с', s @ ..] => super::t14::n11199(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t14::n3662(s, out),
		['н', s @ ..] => super::t14::n3663(s, out),
		['р', s @ ..] => super::t14::n3664(s, out),
		['с', s @ ..] => super::t14::n3665(s, out),
		['х', s @ ..] => super::t14::n3666(s, out),
		['ш', s @ ..] => super::t14::n3667(s, out),
		_ => false,
	}
}

/// Words ending -птить.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 838);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3474);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9311);
					match s {
						[] => true,
						['з', 'и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24227);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24228);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31260);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3475);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16767);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16768);
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
							super::push(out, 16769);
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
						['ы', s @ ..] => {
							super::push(out, 16770);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24229);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртить.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 839);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3476);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 9312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16771);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
		['е', s @ ..] => {
			super::push(out, 3477);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9313);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16772);
							match s {
								[] => true,
								['у', 'х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31261);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31262);
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
				['м', s @ ..] => {
					super::push(out, 9314);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16774);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16775);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24230);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', '\u{1}', s @ ..] => {
									super::push(out, 24231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 16776);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 16777);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24232);
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
							super::push(out, 16778);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 16779);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16780);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 24233);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31263);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 24234);
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
							super::push(out, 16781);
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
						['т', s @ ..] => {
							super::push(out, 16782);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 16783);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24235);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3478);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16784);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16785);
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
							super::push(out, 16786);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 16787);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31264);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16788);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 9317);
					match s {
						[] => true,
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
			super::push(out, 3479);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => super::t14::n23276(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стить.
pub(crate) fn n851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 840);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3480);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16791);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16792);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24237);
									match s {
										[] => true,
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
							super::push(out, 16793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24238);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24239);
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
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 16794);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16795);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16796);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 16797);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 16798);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24241);
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
							super::push(out, 16799);
							match s {
								[] => true,
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
					super::push(out, 9322);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16800);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24242);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 16801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24243);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24244);
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
				['п', s @ ..] => {
					super::push(out, 9323);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16802);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24245);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36429);
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
						['о', s @ ..] => {
							super::push(out, 16803);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24246);
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
						_ => false,
					}
				}
				['р', s @ ..] => super::t14::n11224(s, out),
				['ч', s @ ..] => {
					super::push(out, 9325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16811);
							s.is_empty()
						}
						['а', s @ ..] => super::t14::n23301(s, out),
						['и', s @ ..] => {
							super::push(out, 16813);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => super::t14::n38194(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => super::t14::n23303(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t14::n3675(s, out),
		['и', s @ ..] => super::t14::n3676(s, out),
		['к', s @ ..] => super::t14::n3677(s, out),
		['л', s @ ..] => super::t14::n3678(s, out),
		['м', s @ ..] => super::t14::n3679(s, out),
		['о', s @ ..] => super::t14::n3680(s, out),
		['р', s @ ..] => super::t14::n3681(s, out),
		['у', s @ ..] => super::t14::n3682(s, out),
		['ь', s @ ..] => super::t14::n3683(s, out),
		_ => false,
	}
}

/// Words ending -утить.
pub(crate) fn n852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3491);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9358);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9359);
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
					super::push(out, 9360);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 16877);
							match s {
								[] => true,
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
					super::push(out, 9361);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16878);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9362);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9363);
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
					super::push(out, 9364);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9365);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16879);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16880);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24297);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9366);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9367);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 16882);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24299);
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36447);
													s.is_empty()
												}
												['а', s @ ..] => {
													match s {
														[] => true,
														['з', s @ ..] => super::t14::n76905(s, out),
														['н', s @ ..] => super::t14::n76906(s, out),
														_ => false,
													}
												}
												['е', s @ ..] => {
													super::push(out, 36448);
													match s {
														[] => true,
														['р', 'е', 'п', '\u{1}', s @ ..] => super::t14::n76907(s, out),
														_ => false,
													}
												}
												['з', 'в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 36449);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n76908(s, out),
														_ => false,
													}
												}
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
							super::push(out, 16883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9368);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16884);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24301);
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
					super::push(out, 9369);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16886);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24302);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9370);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16888);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24304);
									match s {
										[] => true,
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
							super::push(out, 16889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9372);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3494);
			match s {
				[] => true,
				['а', s @ ..] => super::t14::n11273(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t14::n3689(s, out),
		['т', s @ ..] => super::t14::n3690(s, out),
		['ш', s @ ..] => super::t14::n3691(s, out),
		['щ', s @ ..] => super::t14::n3692(s, out),
		_ => false,
	}
}

/// Words ending -фтить.
pub(crate) fn n853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3499);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9384);
					match s {
						[] => true,
						['к', s @ ..] => {
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3500);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9385);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16902);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24310);
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 24311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 9386);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16903);
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
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3501);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9387);
					match s {
						[] => true,
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
			super::push(out, 3502);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9388);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтить.
pub(crate) fn n854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3503);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3504);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9389);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытить.
pub(crate) fn n855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 844);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 3505);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9390);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24312);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24313);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31294);
											match s {
												[] => true,
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
									super::push(out, 24314);
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
		['с', s @ ..] => {
			super::push(out, 3506);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9391);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9392);
					match s {
						[] => true,
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
				['е', s @ ..] => {
					super::push(out, 9393);
					match s {
						[] => true,
						['р', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ютить.
pub(crate) fn n856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3507);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3508);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9394);
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
		['р', s @ ..] => {
			super::push(out, 3509);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 3510);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9396);
					match s {
						[] => true,
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
			super::push(out, 3511);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9397);
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

/// Words ending -ятить.
pub(crate) fn n857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 846);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3512);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16904);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3513);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9399);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9400);
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
					super::push(out, 9401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9402);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16906);
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
					super::push(out, 9403);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24315);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24316);
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
									super::push(out, 24317);
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
									super::push(out, 24318);
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
								['ы', s @ ..] => {
									super::push(out, 24319);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31295);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9404);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16908);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16909);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24320);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9406);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16911);
							match s {
								[] => true,
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
					super::push(out, 9407);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word телеграфить.
pub(crate) fn n66566(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -бачить.
pub(crate) fn n3715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3521);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9414);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9415);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9416);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16928);
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
			super::push(out, 9417);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24326);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 9418);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16930);
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
		['ы', s @ ..] => {
			super::push(out, 9419);
			match s {
				[] => true,
				['д', 'р', 'о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31297);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31298);
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
					super::push(out, 16931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24328);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24329);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вачить.
pub(crate) fn n3716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3522);
	match s {
		[] => true,
		['е', 'л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16932);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гачить.
pub(crate) fn n3717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3523);
	match s {
		[] => true,
		['и', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16934);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16935);
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
				['с', s @ ..] => {
					super::push(out, 16936);
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
		['л', 'у', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16937);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -дачить.
pub(crate) fn n3718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9420);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9421);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24330);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 24331);
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
			super::push(out, 9422);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16940);
					match s {
						[] => true,
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
			super::push(out, 9423);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16941);
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
		['у', s @ ..] => {
			super::push(out, 9424);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24332);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24333);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31299);
									match s {
										[] => true,
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
							super::push(out, 24334);
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
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24335);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24336);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24337);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31300);
									match s {
										[] => true,
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
							super::push(out, 24338);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31301);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жачить.
pub(crate) fn n3719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3525);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9425);
			match s {
				[] => true,
				['и', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зачить.
pub(crate) fn n3720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3526);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16942);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16943);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24339);
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
					super::push(out, 16944);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24340);
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

/// Words ending -лачить.
pub(crate) fn n3721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3527);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9426);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16945);
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
		['в', s @ ..] => {
			super::push(out, 9427);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9428);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16947);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31302);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31303);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9429);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16948);
					match s {
						[] => true,
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
			super::push(out, 9430);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16949);
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
		['у', s @ ..] => {
			super::push(out, 9431);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24341);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24342);
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

/// Words ending -мачить.
pub(crate) fn n3722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3528);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9432);
			match s {
				[] => true,
				['д', 'й', 'а', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9433);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16951);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31304);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31305);
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
		['у', s @ ..] => {
			super::push(out, 9434);
			match s {
				[] => true,
				['ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 9435);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16952);
					match s {
						[] => true,
						['о', 'ф', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 9436);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16953);
					match s {
						[] => true,
						['е', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -начить.
pub(crate) fn n3723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3529);
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9437);
			match s {
				[] => true,
				['р', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24343);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24344);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9438);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16954);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16955);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16956);
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
		['и', s @ ..] => {
			super::push(out, 9439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16957);
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
		_ => false,
	}
}

/// Words ending -пачить.
pub(crate) fn n3724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3530);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9440);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24345);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рачить.
pub(crate) fn n3725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3531);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9441);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9442);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16958);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31306);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31307);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36450);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9443);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 16959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24347);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16960);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16961);
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
		['т', s @ ..] => {
			super::push(out, 9445);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16962);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31308);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31309);
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
									super::push(out, 31310);
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
		['у', s @ ..] => {
			super::push(out, 9446);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16963);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24348);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24349);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сачить.
pub(crate) fn n3726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3532);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9447);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9448);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16964);
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
			super::push(out, 9449);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тачить.
pub(crate) fn n3727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3533);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9450);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24350);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 24351);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31311);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9451);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16967);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31312);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 36451);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9452);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24353);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 24354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9453);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16968);
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
		_ => false,
	}
}

/// Words ending -хачить.
pub(crate) fn n3728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3534);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9454);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шачить.
pub(crate) fn n3729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3535);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16970);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16971);
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
			super::push(out, 9456);
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

/// Words ending -бчить.
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 854);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3536);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вчить.
pub(crate) fn n866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 855);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3537);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16972);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16973);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24355);
									match s {
										[] => true,
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
							super::push(out, 16974);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гчить.
pub(crate) fn n867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 856);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3538);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16975);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 16976);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24356);
									match s {
										[] => true,
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
							super::push(out, 16977);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24357);
									match s {
										[] => true,
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
							super::push(out, 16978);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24358);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3539);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16979);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 16980);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16981);
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
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16982);
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

