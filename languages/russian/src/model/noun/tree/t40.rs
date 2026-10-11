use flexion::Path;

/// Words ending -мель.
pub(crate) fn n18936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18831);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 42876);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 42877);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71026);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96017);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 71027);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71028);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96018);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 71029);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96019);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114453);
									s.is_empty()
								}
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
			super::push(out, 42878);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42879);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 42880);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 71032);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96020);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 114454);
									s.is_empty()
								}
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
			super::push(out, 42881);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71033);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 71034);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96021);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114455);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126245);
											s.is_empty()
										}
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
					super::push(out, 71036);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 96022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71037);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96023);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114456);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 126246);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42882);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71038);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 96024);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114457);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 126247);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42883);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 42884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 42885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 42886);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71042);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нель.
pub(crate) fn n18937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18832);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42887);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71043);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 71044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42888);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71045);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 71047);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 71048);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 71049);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96029);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114458);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126248);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133334);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n138984(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42889);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 42890);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71050);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96030);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71051);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 71052);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42891);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71053);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96033);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71054);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96034);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114461);
									s.is_empty()
								}
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
			super::push(out, 42892);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71055);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96035);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 114462);
									s.is_empty()
								}
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
			super::push(out, 42893);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71056);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96036);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 96037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42894);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71057);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 42895);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71058);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оель.
pub(crate) fn n18938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18833);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 42896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пель.
pub(crate) fn n18939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42897);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 71059);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96040);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114464);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126249);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 133335);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71060);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71062);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 96043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42898);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71063);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 71064);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96044);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114466);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 96045);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114467);
									s.is_empty()
								}
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
			super::push(out, 42899);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71065);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96046);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 96047);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 96048);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126251);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 126252);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 133336);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71066);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71067);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 71068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71069);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96051);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71070);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96052);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114470);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126253);
											s.is_empty()
										}
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
					super::push(out, 71071);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71072);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96054);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114471);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126254);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42901);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71073);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 42902);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71075);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96056);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114472);
									s.is_empty()
								}
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
			super::push(out, 42903);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71077);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 42904);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71078);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96059);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114473);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рель.
pub(crate) fn n18940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18835);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42905);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71079);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96060);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 71080);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96061);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114475);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126256);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133337);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 126257);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133338);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42906);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 42907);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71081);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96062);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114476);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 126258);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42908);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71082);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 42909);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71083);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42910);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71085);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96066);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 71086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 42911);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71087);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96067);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114478);
									s.is_empty()
								}
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
			super::push(out, 42912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71088);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 42913);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 71089);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96069);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114479);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 126259);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133339);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42914);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71090);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сель.
pub(crate) fn n18941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18836);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 42915);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 42916);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42917);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 71094);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71095);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 42918);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 42919);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71097);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 71098);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96074);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71099);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 42920);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 42921);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42922);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96076);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 114481);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126260);
											match s {
												[] => true,
												['в', '\u{1}', s @ ..] => {
													super::push(out, 133340);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42923);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71102);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 71103);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96077);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 114482);
									s.is_empty()
								}
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
			super::push(out, 42924);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71104);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96078);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 96079);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 114484);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126261);
											s.is_empty()
										}
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
					super::push(out, 71105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 42925);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71106);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96081);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 114485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 71107);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96082);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114486);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126262);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42926);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71108);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96083);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114487);
									s.is_empty()
								}
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
			super::push(out, 42927);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71109);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96084);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114488);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 126263);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тель.
pub(crate) fn n18942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18837);
	match s {
		[] => true,
		['а', s @ ..] => super::t49::n43351(s, out),
		['е', s @ ..] => {
			super::push(out, 42929);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 71130);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96151);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 96152);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114602);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126431);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 96153);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114603);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126432);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 133499);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 114604);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126433);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 114605);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 126434);
											match s {
												[] => true,
												['о', 'д', '\u{1}', s @ ..] => {
													super::push(out, 133500);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71133);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t49::n43353(s, out),
		['й', s @ ..] => {
			super::push(out, 42931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71151);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96245);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 114765);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126608);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42932);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71152);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114766);
									s.is_empty()
								}
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
			super::push(out, 42933);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71153);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96247);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114767);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126609);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 96248);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114768);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 114769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42934);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 71155);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 71156);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 71157);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96249);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114770);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 126610);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133655);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71158);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 96250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 71159);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71160);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96251);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114771);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126611);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 96252);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114772);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 126612);
											s.is_empty()
										}
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
					super::push(out, 71161);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96253);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114773);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126613);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133656);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42935);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71162);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114774);
									s.is_empty()
								}
								['о', s @ ..] => super::t49::n116004(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 71163);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t49::n71912(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t49::n43359(s, out),
		['т', s @ ..] => super::t49::n43360(s, out),
		['х', s @ ..] => super::t49::n43361(s, out),
		['я', s @ ..] => super::t49::n43362(s, out),
		_ => false,
	}
}

/// Words ending -фель.
pub(crate) fn n18943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18838);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42940);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71177);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 71178);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71179);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96278);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 114786);
									s.is_empty()
								}
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
			super::push(out, 42941);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96279);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 96280);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 96281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 42942);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71181);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 71182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42943);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71183);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96282);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114787);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126622);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 96283);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114788);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 126623);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133662);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n139250(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42944);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71184);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96284);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 71185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42945);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71186);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 71187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 42946);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71188);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96285);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114790);
									s.is_empty()
								}
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
			super::push(out, 42947);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71189);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хель.
pub(crate) fn n18944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18839);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 42948);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71191);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42949);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71192);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 96289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 42950);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71193);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42951);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71194);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цель.
pub(crate) fn n18945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18840);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 42952);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71195);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42953);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71196);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42954);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71197);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42955);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71198);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96294);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114791);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126624);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42956);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 71199);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96295);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 114792);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126625);
											s.is_empty()
										}
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
					super::push(out, 71200);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96296);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71201);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96297);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 96298);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114795);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 126626);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 133663);
													s.is_empty()
												}
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
							super::push(out, 96299);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114796);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71202);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96300);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114797);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126627);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 133664);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42957);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71203);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96301);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 114798);
									s.is_empty()
								}
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
			super::push(out, 42958);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71204);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96302);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чель.
pub(crate) fn n18946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42959);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42960);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71206);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96303);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114800);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126628);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 133665);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шель.
pub(crate) fn n18947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42961);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71207);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 71208);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71209);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96304);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114801);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126629);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133666);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42962);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96306);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 96307);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114802);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126630);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42963);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 42964);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71212);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114803);
									s.is_empty()
								}
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
			super::push(out, 42965);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42966);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 42967);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щель.
pub(crate) fn n18948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18843);
	s.is_empty()
}

/// Words ending -лиль.
pub(crate) fn n18957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18852);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42992);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71241);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 71242);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 71243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -миль.
pub(crate) fn n18958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18853);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42993);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71244);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96329);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 96330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 71246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 42994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ниль.
pub(crate) fn n18959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18854);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42995);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 42996);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 42997);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 71249);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96331);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114814);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126631);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оиль.
pub(crate) fn n18960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18855);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 42998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пиль.
pub(crate) fn n18961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18856);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 42999);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71251);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 96332);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114815);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 126632);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43000);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71252);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 71253);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 71254);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96335);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 114816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71255);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96336);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114817);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126633);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 133667);
													match s {
														[] => true,
														['г', s @ ..] => super::t49::n139251(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 71256);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 71257);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96338);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114818);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126634);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 133668);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n139252(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риль.
pub(crate) fn n18962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18857);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 43001);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 43002);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71258);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114819);
									s.is_empty()
								}
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
			super::push(out, 43003);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 43004);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71259);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -силь.
pub(crate) fn n18963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18858);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43005);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71261);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 96342);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114821);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 126635);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133669);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43006);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 71262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 43007);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71263);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96344);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тиль.
pub(crate) fn n18964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18859);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43008);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 71264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43009);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71265);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 96346);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114823);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126636);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 114824);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126637);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133670);
													match s {
														[] => true,
														['т', s @ ..] => super::t49::n139253(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71266);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96347);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 71267);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71268);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96349);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114826);
									s.is_empty()
								}
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
			super::push(out, 43011);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71269);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96350);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114827);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 96351);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114828);
									s.is_empty()
								}
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
			super::push(out, 43012);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71270);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96352);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114829);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126638);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133671);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43013);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 43014);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 71271);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 71272);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 71273);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 71274);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96353);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 114830);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126639);
											s.is_empty()
										}
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
					super::push(out, 71275);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96354);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71276);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96355);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 96356);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114832);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126640);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 133672);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n139254(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43015);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 43016);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -филь.
pub(crate) fn n18965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18860);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 43017);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71277);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 43018);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71278);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96358);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114833);
									s.is_empty()
								}
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
			super::push(out, 43019);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71279);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хиль.
pub(crate) fn n18966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18861);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43020);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96360);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 96361);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114834);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126641);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -циль.
pub(crate) fn n18967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18862);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43021);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71281);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word виндзейль.
pub(crate) fn n135051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133673);
	s.is_empty()
}

/// Words ending -ье-коктейль.
pub(crate) fn n135052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133674);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 140309);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 141786);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 142648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ильм-спектакль.
pub(crate) fn n135053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133675);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 137832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сундсвалль.
pub(crate) fn n135054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133676);
	s.is_empty()
}

/// Words ending -оволукомль.
pub(crate) fn n135055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133677);
	s.is_empty()
}

/// Words ending -оль-моголь.
pub(crate) fn n135056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133678);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 137833);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140310);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 141787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -триоль.
pub(crate) fn n72063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71314);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 96390);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114851);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 126649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фиоль.
pub(crate) fn n43483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43060);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 71315);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 96391);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 114852);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126650);
									s.is_empty()
								}
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
			super::push(out, 71316);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 96392);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 114853);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126651);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 133679);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йоль.
pub(crate) fn n18987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18882);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 43061);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коль.
pub(crate) fn n18988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18883);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43062);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43063);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71319);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 43064);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71321);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 71322);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96397);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114854);
									s.is_empty()
								}
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
			super::push(out, 43065);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 71323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43066);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -моль.
pub(crate) fn n18989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18884);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43067);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 43068);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71325);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 96398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43069);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71326);
					match s {
						[] => true,
						['н', 'а', 'о', s @ ..] => {
							super::push(out, 96399);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114855);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 126652);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133680);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43070);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71327);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 71328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ноль.
pub(crate) fn n18990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18885);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43071);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 43072);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71329);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 71330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поль.
pub(crate) fn n18991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18886);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43073);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 43074);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43075);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71333);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96401);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 114856);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126653);
											s.is_empty()
										}
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
					super::push(out, 71334);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96402);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 114857);
									s.is_empty()
								}
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
			super::push(out, 43076);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71335);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96403);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114858);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126654);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133681);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n139258(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43077);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71336);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96404);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114859);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126655);
											s.is_empty()
										}
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
					super::push(out, 71337);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96405);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126656);
											s.is_empty()
										}
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
					super::push(out, 71338);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 96406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71339);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96407);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 71340);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96408);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114862);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126657);
											s.is_empty()
										}
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
					super::push(out, 71341);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96409);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 96410);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 114864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 96411);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71342);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96412);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114866);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126658);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 133682);
													s.is_empty()
												}
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
							super::push(out, 96413);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96414);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 96415);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 96416);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114869);
									s.is_empty()
								}
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
			super::push(out, 43078);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71345);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43079);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71346);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96419);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71347);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 96420);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114871);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126659);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133683);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роль.
pub(crate) fn n18992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18887);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43080);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 43081);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71348);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 71349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 43082);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43083);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71351);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96422);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 114872);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126660);
											s.is_empty()
										}
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
					super::push(out, 71352);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96423);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114873);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 126661);
											s.is_empty()
										}
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
					super::push(out, 71353);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43084);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43085);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 43086);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71355);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 96425);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114874);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126662);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133684);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 114875);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126663);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 133685);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['а', s @ ..] => {
							super::push(out, 96426);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 96427);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 96428);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 96429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71356);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96430);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114876);
									s.is_empty()
								}
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
			super::push(out, 43087);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71357);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 71358);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96432);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114877);
									s.is_empty()
								}
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
			super::push(out, 43088);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71359);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96433);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 114878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соль.
pub(crate) fn n18993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18888);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43089);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 43090);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71360);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96434);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114879);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 126664);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133686);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 43091);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71361);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96435);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 71362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43092);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71363);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96436);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114881);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126665);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 96437);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114882);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126666);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 133687);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43093);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71364);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96438);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114883);
									s.is_empty()
								}
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
			super::push(out, 43094);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71365);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96439);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114884);
									s.is_empty()
								}
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
			super::push(out, 43095);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71366);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43096);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71367);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96441);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71368);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43097);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71369);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -толь.
pub(crate) fn n18994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18889);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43098);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43099);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96444);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 96445);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114886);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126667);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уоль.
pub(crate) fn n18995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18890);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 43100);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 43101);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71371);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фоль.
pub(crate) fn n18996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18891);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43102);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71372);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96447);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71373);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96448);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114888);
									s.is_empty()
								}
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
			super::push(out, 43103);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71374);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -холь.
pub(crate) fn n18997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18892);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 43104);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 71375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43105);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71376);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 71377);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 96452);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114889);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126668);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьоль.
pub(crate) fn n18998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18893);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 43106);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71378);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пусторосль.
pub(crate) fn n135066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133688);
	s.is_empty()
}

/// Words ending -иссык-куль.
pub(crate) fn n135067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133689);
	s.is_empty()
}

/// Words ending -труль.
pub(crate) fn n43568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43145);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 71412);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 96481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 114906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суль.
pub(crate) fn n19023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18918);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43146);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 43147);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71413);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -финьшампань.
pub(crate) fn n139259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137835);
	s.is_empty()
}

/// Words ending -рт-пристань.
pub(crate) fn n139260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137836);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140311);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 141788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тайцзицюань.
pub(crate) fn n139261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137837);
	s.is_empty()
}

/// The word погребень.
pub(crate) fn n135073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133695);
	s.is_empty()
}

/// The word форштевень.
pub(crate) fn n135074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133696);
	s.is_empty()
}

/// Words ending -ивень.
pub(crate) fn n43680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43257);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 71512);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 71513);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 96548);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 114940);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 126694);
									s.is_empty()
								}
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
			super::push(out, 71514);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 96549);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 114941);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 126695);
									s.is_empty()
								}
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
			super::push(out, 71515);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 96550);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 114942);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 126696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овень.
pub(crate) fn n43681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43258);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 71516);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 96551);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 114943);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 126697);
									s.is_empty()
								}
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
			super::push(out, 71517);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 96552);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 114944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 71518);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 96553);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твень.
pub(crate) fn n43683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43260);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 71520);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 96555);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 114945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -день.
pub(crate) fn n19084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18979);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43261);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 43262);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71521);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 96556);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 96557);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 114947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 96558);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 114948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 71522);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96559);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114949);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 126698);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133697);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n139262(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 43263);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71523);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96560);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 114950);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126699);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133698);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 114951);
									s.is_empty()
								}
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
			super::push(out, 43264);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71524);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96561);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114952);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 126700);
											s.is_empty()
										}
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
					super::push(out, 71525);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 43265);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71526);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96563);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114953);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126701);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133699);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 114954);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126702);
											s.is_empty()
										}
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
					super::push(out, 71527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43266);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 71528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 43267);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71529);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96565);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114955);
									s.is_empty()
								}
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
			super::push(out, 43268);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71530);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 96566);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114956);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126703);
											s.is_empty()
										}
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
					super::push(out, 71531);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96567);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114957);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 126704);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 133700);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71532);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96568);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 96569);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114958);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 114959);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126705);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133701);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n139263(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 71533);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43269);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71534);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96571);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114960);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126706);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133702);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n139264(s, out),
														_ => false,
													}
												}
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
							super::push(out, 96572);
							match s {
								[] => true,
								['о', s @ ..] => super::t49::n116190(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t49::n72285(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t49::n43693(s, out),
		['ы', s @ ..] => super::t49::n43694(s, out),
		['я', s @ ..] => super::t49::n43695(s, out),
		_ => false,
	}
}

/// Words ending -жень.
pub(crate) fn n19085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18980);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43273);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 71544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43274);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96580);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 96581);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114964);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126708);
											s.is_empty()
										}
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
					super::push(out, 71546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71547);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96583);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71548);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96584);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114966);
									s.is_empty()
								}
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
			super::push(out, 43275);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43276);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71550);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96585);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114967);
									s.is_empty()
								}
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
			super::push(out, 43277);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зень.
pub(crate) fn n19086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18981);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43278);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71552);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96586);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71553);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96587);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 96588);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114969);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126709);
											s.is_empty()
										}
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
					super::push(out, 71554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71555);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71556);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 43280);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71557);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96590);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114970);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126710);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 114971);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126711);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43281);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71558);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лень.
pub(crate) fn n19087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18982);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43282);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 43283);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71559);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 96592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 43284);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71560);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96593);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 114972);
									s.is_empty()
								}
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
			super::push(out, 43285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71561);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 71562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 71563);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96595);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114973);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126712);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 96596);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114974);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126713);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133704);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n139265(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 126714);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133705);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71564);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 71565);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96597);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 114975);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 126715);
											s.is_empty()
										}
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
					super::push(out, 71566);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 71567);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43287);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71568);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 43288);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мень.
pub(crate) fn n19088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18983);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43289);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 43290);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96600);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 96601);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 114976);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126716);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 133706);
													match s {
														[] => true,
														['т', s @ ..] => super::t49::n139266(s, out),
														_ => false,
													}
												}
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
							super::push(out, 96602);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114977);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126717);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 96603);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114978);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126718);
											s.is_empty()
										}
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
					super::push(out, 71571);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96604);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43291);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71573);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96606);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96607);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 96608);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114981);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126719);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43292);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71575);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43293);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71576);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 96610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 43294);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71577);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43295);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71578);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96612);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 114982);
									s.is_empty()
								}
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
			super::push(out, 43296);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71579);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 43297);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 71580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 43298);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71581);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96614);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 96615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 43299);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пень.
pub(crate) fn n19089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18984);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43300);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 43301);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96617);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 96618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71584);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 71585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43302);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71586);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 43303);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71588);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43304);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 71589);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71590);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43305);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71591);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43306);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71593);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71594);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 96625);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96626);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 96627);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114988);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126720);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133707);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n139267(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рень.
pub(crate) fn n19090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18985);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43307);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71596);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 96628);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114989);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 126721);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133708);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n139268(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43308);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71597);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71598);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43309);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71599);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 71600);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43310);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 71601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43311);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71602);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114991);
									s.is_empty()
								}
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
			super::push(out, 43312);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71603);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96634);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 114992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 96635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71604);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96636);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114993);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126722);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133709);
													match s {
														[] => true,
														['ж', s @ ..] => super::t49::n139269(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43313);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71606);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 71607);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 96638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 43314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сень.
pub(crate) fn n19091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18986);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43315);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 43316);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71608);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96639);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114994);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126723);
											s.is_empty()
										}
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
					super::push(out, 71609);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96640);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114995);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126724);
											s.is_empty()
										}
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
					super::push(out, 71610);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96641);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', '\u{1}', s @ ..] => {
							super::push(out, 96642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43317);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 43318);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тень.
pub(crate) fn n19092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18987);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43319);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 43320);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71611);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96643);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 114997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 71612);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96644);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71613);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96645);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114999);
									s.is_empty()
								}
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
			super::push(out, 43321);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71614);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 96646);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 71615);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96647);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115001);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126725);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 96648);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 115002);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 126726);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 96649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 71616);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 96650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43322);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71617);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96651);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 96652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 71618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 43323);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71619);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 96654);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115004);
									s.is_empty()
								}
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
			super::push(out, 43324);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71620);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 96655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71621);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96656);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 96657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71622);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96658);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 115005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 96659);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 115006);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126727);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 115007);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 126728);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 133710);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71623);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 96660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43325);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71624);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96661);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115008);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126729);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133711);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n139270(s, out),
														_ => false,
													}
												}
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
							super::push(out, 96662);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115009);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126730);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133712);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43326);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71625);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71626);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96664);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 115010);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126731);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 133713);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n139271(s, out),
														_ => false,
													}
												}
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
							super::push(out, 96665);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71627);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96666);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t49::n43750(s, out),
		['ы', s @ ..] => {
			super::push(out, 43328);
			match s {
				[] => true,
				['п', s @ ..] => super::t49::n72381(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t49::n43752(s, out),
		_ => false,
	}
}

/// Words ending -чень.
pub(crate) fn n19093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18988);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43330);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71634);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71635);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96673);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 71636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43331);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71637);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 71638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43332);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71639);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71640);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96675);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шень.
pub(crate) fn n19094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18989);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43333);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 71642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43334);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71643);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 96677);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115020);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126732);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 96678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71644);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 71645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43335);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71646);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71647);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 43336);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71648);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96682);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 115021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стоп-огонь.
pub(crate) fn n135092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133714);
	s.is_empty()
}

/// Words ending -ес-словарь.
pub(crate) fn n135093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133715);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137848);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 140314);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 141790);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 142649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дарь.
pub(crate) fn n19159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19054);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43444);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71746);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96742);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115044);
									s.is_empty()
								}
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
			super::push(out, 43445);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71747);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96743);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 96744);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71748);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96745);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115046);
									s.is_empty()
								}
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
			super::push(out, 43446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71749);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96746);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115047);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126741);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133716);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71750);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96747);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115048);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126742);
											s.is_empty()
										}
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
					super::push(out, 71751);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115049);
									s.is_empty()
								}
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
			super::push(out, 43447);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 71752);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 71753);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96749);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115050);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 126743);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43448);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 71755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96751);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 96752);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зарь.
pub(crate) fn n19160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19055);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43449);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 71757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 43450);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71758);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43451);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 71759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43452);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 71760);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -карь.
pub(crate) fn n19161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19056);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43453);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71761);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 71762);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 71763);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96755);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 96756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 43454);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43455);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 43456);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71766);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 96758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43457);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71767);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43458);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71769);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 71770);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96762);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115053);
									s.is_empty()
								}
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
			super::push(out, 43459);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71771);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 71772);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 43460);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 43461);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71774);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 43462);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71775);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 43463);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ларь.
pub(crate) fn n43888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43464);
	s.is_empty()
}

/// Words ending -еларь.
pub(crate) fn n43889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43465);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 71777);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 96766);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -марь.
pub(crate) fn n19163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19058);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43466);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 43467);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71778);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115054);
									s.is_empty()
								}
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
			super::push(out, 43468);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71779);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96768);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115055);
									s.is_empty()
								}
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
			super::push(out, 43469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 43470);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71781);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96769);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115056);
									s.is_empty()
								}
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
			super::push(out, 43471);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', '\u{1}', s @ ..] => {
			super::push(out, 43472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нарь.
pub(crate) fn n19164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19059);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 43473);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71783);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43474);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43475);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71785);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43476);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71786);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71787);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 71788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43477);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71789);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 43478);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 71790);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96776);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 115057);
									s.is_empty()
								}
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
			super::push(out, 43479);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71791);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -парь.
pub(crate) fn n19165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19060);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43480);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43481);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 71793);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 96778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43482);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71795);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71796);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 43483);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71797);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 96782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43484);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71798);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рарь.
pub(crate) fn n19166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19061);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43485);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 71799);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96783);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 115058);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126744);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сарь.
pub(crate) fn n19167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19062);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43487);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71800);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71801);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 71802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43488);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43489);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71804);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96785);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115059);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126745);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43490);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71805);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 43491);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тарь.
pub(crate) fn n19168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19063);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43492);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71806);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 96787);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 115060);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126746);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133717);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71807);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71808);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71809);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43493);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71810);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96789);
							match s {
								[] => true,
								['е', 'с', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 126747);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133718);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n139273(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 133719);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n139274(s, out),
														['с', s @ ..] => super::t49::n139275(s, out),
														['т', s @ ..] => super::t49::n139276(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 126748);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 133720);
													match s {
														[] => true,
														['г', s @ ..] => super::t49::n139277(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43494);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 43495);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71812);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96790);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 115061);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126749);
											s.is_empty()
										}
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
						['г', s @ ..] => {
							super::push(out, 96791);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 96792);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 71813);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 71814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43496);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71815);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71817);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 71818);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 43498);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71819);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115064);
									s.is_empty()
								}
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
			super::push(out, 43499);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71820);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 43500);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фарь.
pub(crate) fn n19169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19064);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 43501);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 43502);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71823);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 96798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -харь.
pub(crate) fn n19170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19065);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43503);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71824);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 71825);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 71826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43504);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 71827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71828);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43505);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71829);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 96802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43506);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71830);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 71831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 71832);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96804);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 115065);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126750);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133721);
													match s {
														[] => true,
														['р', s @ ..] => super::t49::n139278(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 71833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -царь.
pub(crate) fn n19171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19066);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 43507);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чарь.
pub(crate) fn n19172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19067);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 43508);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71835);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шарь.
pub(crate) fn n19173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19068);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43509);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алтырь.
pub(crate) fn n72657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71906);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 96849);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 115089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стырь.
pub(crate) fn n44008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43584);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 71907);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 96850);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 115090);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 126760);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 133726);
											s.is_empty()
										}
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
					super::push(out, 96851);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115091);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 126761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 96852);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115092);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 126762);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133727);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137855);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 71908);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 96853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цырь.
pub(crate) fn n19215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19110);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 43586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71909);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ом-летопись.
pub(crate) fn n139280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137856);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 140320);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 141795);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142653);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 143110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нлайн-сеть.
pub(crate) fn n135112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133734);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137860);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ранет-сеть.
pub(crate) fn n135113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133735);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137861);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 140323);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 141797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ернет-сеть.
pub(crate) fn n135114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133736);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137862);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 140324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмсеть.
pub(crate) fn n72730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71978);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 96900);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 115116);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 126776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осеть.
pub(crate) fn n44088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43664);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 71979);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 96901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 71980);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 96902);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115117);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 126777);
									s.is_empty()
								}
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
			super::push(out, 71981);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 96903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 71982);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 96904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 71983);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 96905);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 96906);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 115118);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 126778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 96907);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 115119);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126779);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 133737);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 137863);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тсеть.
pub(crate) fn n44089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43665);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 71984);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 96908);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115120);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 126780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цсеть.
pub(crate) fn n44090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43666);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 71985);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 96909);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -четь.
pub(crate) fn n19265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19160);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43667);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43668);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71987);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -круговерть.
pub(crate) fn n135116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133738);
	s.is_empty()
}

/// The word коловерть.
pub(crate) fn n135117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133739);
	s.is_empty()
}

/// Words ending -полусмерть.
pub(crate) fn n135118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133740);
	s.is_empty()
}

/// Words ending -метавласть.
pub(crate) fn n135119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133741);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -медсанчасть.
pub(crate) fn n135120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133742);
	s.is_empty()
}

/// Words ending -екресть.
pub(crate) fn n97981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96946);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 115135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 126791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сесть.
pub(crate) fn n44135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43711);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72039);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 96947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 115136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -честь.
pub(crate) fn n44137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43713);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 72040);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 72041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шесть.
pub(crate) fn n44138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72042);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щесть.
pub(crate) fn n44139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43715);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 72043);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 96948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -исть.
pub(crate) fn n19285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19180);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 43716);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72044);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96949);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 96950);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115137);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 126792);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43717);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43718);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72046);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96951);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115138);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 126793);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43719);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 72047);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96952);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 115139);
									s.is_empty()
								}
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
			super::push(out, 43720);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72048);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72049);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96954);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115140);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лсть.
pub(crate) fn n19286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19181);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 43721);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72050);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ость.
pub(crate) fn n19287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19182);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43722);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 43723);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72051);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96955);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 96956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 72052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t49::n44148(s, out),
		['г', s @ ..] => {
			super::push(out, 43725);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 72062);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96997);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115248);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126909);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133865);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72063);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 72064);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 43726);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72065);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 72066);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 96999);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115249);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 126910);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133866);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72067);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 72068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 43727);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72069);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97000);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 115250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 72070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72071);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 72072);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 72073);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 72074);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 72075);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 97001);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 115251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 72076);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 72077);
					match s {
						[] => true,
						['о', 'т', 'с', 'о', s @ ..] => {
							super::push(out, 97002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 72078);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 72079);
					match s {
						[] => true,
						['е', 'о', 'д', s @ ..] => {
							super::push(out, 97003);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 115252);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126911);
											s.is_empty()
										}
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
					super::push(out, 72080);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97004);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 97005);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115253);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 126912);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133867);
													s.is_empty()
												}
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
							super::push(out, 97006);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 115254);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126913);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133868);
													s.is_empty()
												}
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
							super::push(out, 97007);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 72081);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97008);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115256);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126914);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133869);
													s.is_empty()
												}
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
							super::push(out, 97009);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115257);
									match s {
										[] => true,
										['о', s @ ..] => super::t49::n128238(s, out),
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
					super::push(out, 72082);
					s.is_empty()
				}
				['с', s @ ..] => super::t49::n72835(s, out),
				['т', s @ ..] => super::t49::n72836(s, out),
				['у', s @ ..] => {
					super::push(out, 72085);
					s.is_empty()
				}
				['ы', s @ ..] => super::t49::n72838(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t49::n44153(s, out),
		['м', s @ ..] => super::t49::n44154(s, out),
		['н', s @ ..] => super::t49::n44155(s, out),
		['п', s @ ..] => super::t49::n44156(s, out),
		['р', s @ ..] => super::t49::n44157(s, out),
		['с', s @ ..] => super::t49::n44158(s, out),
		['т', s @ ..] => super::t49::n44159(s, out),
		['х', s @ ..] => super::t49::n44160(s, out),
		_ => false,
	}
}

/// Words ending -рсть.
pub(crate) fn n19288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19183);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43737);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72148);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 72149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43738);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усть.
pub(crate) fn n19289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19184);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 43739);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысть.
pub(crate) fn n19290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19185);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 43740);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72152);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юсть.
pub(crate) fn n19291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19186);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 43741);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72153);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 97195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115629);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясть.
pub(crate) fn n19292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19187);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 43742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -белоголовая.
pub(crate) fn n139941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138506);
	s.is_empty()
}

/// Words ending -он-столовая.
pub(crate) fn n139942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138507);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 140754);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142036);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 142773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тровая.
pub(crate) fn n73002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72248);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 97259);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115672);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 127478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 97260);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 115673);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 127479);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134486);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138508);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140755);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совая.
pub(crate) fn n44293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43869);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 72250);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 72251);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 97263);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127480);
									s.is_empty()
								}
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
			super::push(out, 72252);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 97264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 72253);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 97265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товая.
pub(crate) fn n44294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43870);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 72254);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 97266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 72255);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 97267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олстопузая.
pub(crate) fn n135892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134487);
	s.is_empty()
}

/// Words ending -олицейская.
pub(crate) fn n135893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134488);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 138509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -анская.
pub(crate) fn n73028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72274);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 97286);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 115687);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127487);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 134489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ринская.
pub(crate) fn n98327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97287);
	s.is_empty()
}

/// Words ending -рская.
pub(crate) fn n44313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43889);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72276);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 97288);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 115688);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127488);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 134490);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 138510);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72277);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97289);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 115689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115690);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 127489);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 134491);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138511);
													s.is_empty()
												}
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
							super::push(out, 115691);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127490);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 134492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 127491);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 134493);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 138512);
													match s {
														[] => true,
														['у', s @ ..] => super::t49::n142201(s, out),
														_ => false,
													}
												}
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
							super::push(out, 115692);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127492);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 134494);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 115693);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127493);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134495);
											s.is_empty()
										}
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
					super::push(out, 97290);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115694);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127494);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 134496);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 115695);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127495);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 115696);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 127496);
									s.is_empty()
								}
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
			super::push(out, 72278);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 97291);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 115697);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127497);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 134497);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72279);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 97292);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 115698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 97293);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115699);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 127498);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 134498);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 138513);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72280);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97294);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 115700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 97295);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сская.
pub(crate) fn n44314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43890);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 72281);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 97296);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тская.
pub(crate) fn n44315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43891);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 72282);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 97297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 72283);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьская.
pub(crate) fn n44316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43892);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 72284);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97299);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 115702);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цкая.
pub(crate) fn n19445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19340);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43893);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72285);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97300);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115703);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127500);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 134499);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43894);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 72286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькая.
pub(crate) fn n19446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19341);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 43895);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72287);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97301);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115704);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127501);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 97302);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 97303);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115706);
									s.is_empty()
								}
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
			super::push(out, 43896);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72288);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -глухонемая.
pub(crate) fn n135905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134500);
	s.is_empty()
}

/// Words ending -втоклавная.
pub(crate) fn n135906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134501);
	s.is_empty()
}

/// Words ending -авославная.
pub(crate) fn n135907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134502);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -невыездная.
pub(crate) fn n135908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134503);
	s.is_empty()
}

/// Words ending -выходная.
pub(crate) fn n116972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115725);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 127510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдная.
pub(crate) fn n44349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43925);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 72322);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 97331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядная.
pub(crate) fn n44350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43926);
	s.is_empty()
}

/// Words ending -еная.
pub(crate) fn n19460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19355);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 43927);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72323);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97332);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115726);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 127511);
											s.is_empty()
										}
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
					super::push(out, 72324);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 43928);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72325);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97334);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 115727);
									s.is_empty()
								}
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
			super::push(out, 43929);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 72326);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жная.
pub(crate) fn n19461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19356);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43930);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 43931);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72327);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97335);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 115728);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127512);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 134504);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72328);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97336);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 115729);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127513);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 134505);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72329);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97337);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115730);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 127514);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43932);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72331);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97338);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115731);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 127515);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43934);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 72332);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97339);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115732);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 127516);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134506);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n139950(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43935);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 72333);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97340);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115733);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 127517);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134507);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зная.
pub(crate) fn n19462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19357);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43936);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72334);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97341);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 97342);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115735);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127518);
											s.is_empty()
										}
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
					super::push(out, 72335);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 72336);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115736);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 115737);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 127519);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43937);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72337);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 97344);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115738);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 127520);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134508);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n139951(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 72338);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97345);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115739);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 127521);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134509);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иная.
pub(crate) fn n19463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19358);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 43938);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72339);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97346);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115740);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 127522);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 134510);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n139952(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йная.
pub(crate) fn n19464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19359);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43939);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 72340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43940);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 43941);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 72341);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97347);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115741);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127523);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134511);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n139953(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43942);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72342);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 72343);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97348);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кная.
pub(crate) fn n19465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19360);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 43943);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 72344);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97349);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 115743);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127524);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мная.
pub(crate) fn n19466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19361);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43944);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72345);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97350);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115744);
									s.is_empty()
								}
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
			super::push(out, 43945);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72346);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 72347);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нная.
pub(crate) fn n19467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43946);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72348);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97351);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 115745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 97352);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115746);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127525);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134512);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 134513);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 115747);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127526);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134514);
													match s {
														[] => true,
														['р', s @ ..] => super::t49::n139954(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 72349);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 97353);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115748);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127527);
											s.is_empty()
										}
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
					super::push(out, 72350);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97354);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115749);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 127528);
											s.is_empty()
										}
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
					super::push(out, 72351);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 97355);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 115750);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127529);
											s.is_empty()
										}
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
					super::push(out, 72352);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97356);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115751);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127530);
											s.is_empty()
										}
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
					super::push(out, 72353);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97357);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 72354);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97358);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 115753);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127531);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134515);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t49::n44371(s, out),
		['и', s @ ..] => {
			super::push(out, 43948);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72367);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97384);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 72368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43949);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72369);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 97385);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115783);
									s.is_empty()
								}
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
			super::push(out, 43950);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72370);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 97386);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115784);
									s.is_empty()
								}
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
			super::push(out, 43951);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72371);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97387);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 72372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пная.
pub(crate) fn n19468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19363);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 43952);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43953);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 72374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 43954);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72375);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97388);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рная.
pub(crate) fn n19469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19364);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43955);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 72376);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 97389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 72377);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72378);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43956);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72379);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97392);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115787);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127556);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134536);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n139963(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 72380);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97393);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 115788);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 127557);
											s.is_empty()
										}
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
					super::push(out, 72381);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 97394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 97395);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 115790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 72382);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97396);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115791);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127558);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97397);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115792);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 127559);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 134537);
													s.is_empty()
												}
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
							super::push(out, 97398);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 115793);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127560);
											s.is_empty()
										}
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
					super::push(out, 72383);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97399);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 97400);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 115795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 72384);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43957);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72385);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 97402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 72386);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97403);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115796);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127561);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134538);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72387);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115797);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127562);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 134539);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 115798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 97404);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 115799);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 127563);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 134540);
													match s {
														[] => true,
														['у', s @ ..] => super::t49::n139964(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43958);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 72388);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 72389);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97406);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 115800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 72390);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сная.
pub(crate) fn n19470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19365);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43959);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72391);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 72392);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43960);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72393);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43961);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43962);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72396);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97411);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115801);
									s.is_empty()
								}
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
			super::push(out, 43963);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 72397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тная.
pub(crate) fn n19471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19366);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43964);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72398);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 72399);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97413);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115802);
									s.is_empty()
								}
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
			super::push(out, 43965);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43966);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72401);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97414);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 115803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 72402);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97415);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 115804);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 127564);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 134541);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43967);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72403);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97416);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115805);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 127565);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134542);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97417);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 115806);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 127566);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43968);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72405);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97418);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 115807);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127567);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 134543);
													s.is_empty()
												}
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
							super::push(out, 97419);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115808);
									s.is_empty()
								}
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
			super::push(out, 43969);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72406);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 97420);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 72407);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97421);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 97422);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72408);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97423);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115811);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 127568);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134544);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фная.
pub(crate) fn n19472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19367);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43970);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72409);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97424);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 115812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чная.
pub(crate) fn n19473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19368);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43971);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72410);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97425);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115813);
									s.is_empty()
								}
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
			super::push(out, 43972);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72411);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 97426);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 115814);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127569);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 134545);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72412);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97427);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115815);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127570);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 134546);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n139965(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 72413);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97428);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115816);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127571);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134547);
													s.is_empty()
												}
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
							super::push(out, 97429);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115817);
									s.is_empty()
								}
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
			super::push(out, 43973);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72414);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97430);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115818);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127572);
											s.is_empty()
										}
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
					super::push(out, 72415);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97431);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115819);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127573);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97432);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115820);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127574);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 97433);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 115821);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127575);
											s.is_empty()
										}
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
					super::push(out, 72416);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97434);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115822);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 127576);
											s.is_empty()
										}
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
					super::push(out, 72417);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97435);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115823);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127577);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43974);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72418);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 72419);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97436);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 115824);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 127578);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134548);
													s.is_empty()
												}
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
							super::push(out, 97437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 72420);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 72421);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72422);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 72423);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97439);
							match s {
								[] => true,
								['д', s @ ..] => super::t49::n117072(s, out),
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
			super::push(out, 43975);
			match s {
				[] => true,
				['л', s @ ..] => super::t49::n73178(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t49::n44400(s, out),
		_ => false,
	}
}

/// Words ending -шная.
pub(crate) fn n19474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19369);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43977);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43978);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72428);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97443);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щная.
pub(crate) fn n19475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19370);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 43979);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 72429);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97444);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115830);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 127582);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьная.
pub(crate) fn n19476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19371);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 43980);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72430);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97445);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115831);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127583);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 134551);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n139967(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 72431);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97446);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 115832);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127584);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 134552);
													s.is_empty()
												}
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
							super::push(out, 97447);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 115833);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 97448);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115834);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 127585);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 97449);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115835);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 127586);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 97450);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115836);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127587);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134553);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n139968(s, out),
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
									super::push(out, 115837);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127588);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 115838);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 127589);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134554);
													s.is_empty()
												}
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
							super::push(out, 97451);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115839);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127590);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134555);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72432);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яная.
pub(crate) fn n19477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19372);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 43981);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72433);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43982);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72434);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97454);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115841);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127591);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134556);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ор-ведущая.
pub(crate) fn n135962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134557);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 138534);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140767);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ма-тьмущая.
pub(crate) fn n135963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134558);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 138535);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 140768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твующая.
pub(crate) fn n98526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97482);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 115859);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 127600);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134559);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 138536);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140769);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142044);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дующая.
pub(crate) fn n73230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72476);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97483);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115860);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127601);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 134560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зующая.
pub(crate) fn n73231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72477);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97484);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 115861);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 127602);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134561);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рующая.
pub(crate) fn n73232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72478);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97485);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115862);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яющая.
pub(crate) fn n44448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44024);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 72479);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97486);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115863);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127603);
									match s {
										[] => true,
										['п', 'а', 'н', '\u{1}', s @ ..] => {
											super::push(out, 134562);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 127604);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 134563);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ящая.
pub(crate) fn n19507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19402);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 44025);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72480);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97487);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115864);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127605);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134564);
													match s {
														[] => true,
														['я', s @ ..] => super::t49::n139972(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 72481);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97488);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115865);
									s.is_empty()
								}
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
			super::push(out, 44026);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72482);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97489);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word гревиллея.
pub(crate) fn n135971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134566);
	s.is_empty()
}

/// Words ending -трела-змея.
pub(crate) fn n135972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134567);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138538);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кожгалантерея.
pub(crate) fn n135973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134568);
	s.is_empty()
}

/// Words ending -полименорея.
pub(crate) fn n135974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134569);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 138539);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word стеаторея.
pub(crate) fn n135975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134570);
	s.is_empty()
}

/// Words ending -адриплегия.
pub(crate) fn n135982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134577);
	s.is_empty()
}

/// Words ending -ория-регия.
pub(crate) fn n135983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134578);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 138543);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 140772);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -театопигия.
pub(crate) fn n135984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134579);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -коптеригия.
pub(crate) fn n135985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134580);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 140773);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лгия.
pub(crate) fn n19604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19499);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44145);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72582);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нгия.
pub(crate) fn n19605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44146);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72583);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97560);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 115907);
									s.is_empty()
								}
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
			super::push(out, 44147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -огия.
pub(crate) fn n19606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19501);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 44148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44149);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					super::push(out, 72585);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72586);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97562);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 115908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 97563);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115909);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127633);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 134581);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n139981(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'а', s @ ..] => super::t49::n98609(s, out),
						['е', s @ ..] => super::t49::n98610(s, out),
						['з', s @ ..] => super::t49::n98611(s, out),
						['и', s @ ..] => super::t49::n98612(s, out),
						['к', s @ ..] => super::t49::n98613(s, out),
						['л', s @ ..] => super::t49::n98614(s, out),
						['м', 'о', s @ ..] => super::t49::n98615(s, out),
						['н', s @ ..] => super::t49::n98616(s, out),
						['п', 'о', s @ ..] => super::t49::n98617(s, out),
						['р', s @ ..] => super::t49::n98618(s, out),
						['с', 'а', s @ ..] => super::t49::n98619(s, out),
						['т', s @ ..] => super::t49::n98620(s, out),
						['ф', s @ ..] => {
							super::push(out, 97574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргия.
pub(crate) fn n19607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19502);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44150);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 44151);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72587);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 72588);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97576);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115940);
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
							super::push(out, 97577);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 97578);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 97579);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 72589);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72590);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97581);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 115943);
									s.is_empty()
								}
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
			super::push(out, 44152);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 44153);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72591);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97582);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115944);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127657);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 97583);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 72592);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97584);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 115946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 97585);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115947);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127658);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134599);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'а', 'м', 'у', s @ ..] => {
					super::push(out, 72593);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97586);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 115948);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127659);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44154);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72594);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97587);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 115949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьгия.
pub(crate) fn n19608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19503);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44155);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72595);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 72596);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97588);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гипоспадия.
pub(crate) fn n136005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134600);
	s.is_empty()
}

/// Words ending -экзоконидия.
pub(crate) fn n136006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134601);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 138556);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зополиплоидия.
pub(crate) fn n136007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134602);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 138557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лополиплоидия.
pub(crate) fn n136008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134603);
	s.is_empty()
}

/// Words ending -тополиплоидия.
pub(crate) fn n136009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134604);
	s.is_empty()
}

/// Words ending -лхиплоидия.
pub(crate) fn n136010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134605);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138558);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 140781);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уплоидия.
pub(crate) fn n117208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115959);
	s.is_empty()
}

/// Words ending -фоидия.
pub(crate) fn n73365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72610);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 97595);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 115960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ридия.
pub(crate) fn n44591(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 72611);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 97596);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115961);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127663);
									s.is_empty()
								}
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
			super::push(out, 72612);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 97597);
					match s {
						[] => true,
						['с', 'о', 'к', s @ ..] => {
							super::push(out, 115962);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127664);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134606);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 138559);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сидия.
pub(crate) fn n44592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44167);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 72613);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 97598);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цидия.
pub(crate) fn n44593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44168);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 72614);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 72615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97599);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 115963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндия.
pub(crate) fn n19613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19508);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44169);
			match s {
				[] => true,
				['л', 'т', s @ ..] => {
					super::push(out, 72616);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97600);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 115964);
									s.is_empty()
								}
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
			super::push(out, 44170);
			match s {
				[] => true,
				['л', 'н', s @ ..] => {
					super::push(out, 72617);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97601);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 115965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одия.
pub(crate) fn n19614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19509);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72618);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 72619);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97602);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115966);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 127665);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 134607);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n139995(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'о', 'л', s @ ..] => {
			super::push(out, 44171);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72620);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97603);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 115967);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127666);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'п', s @ ..] => {
			super::push(out, 44173);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72622);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97604);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115968);
									s.is_empty()
								}
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
			super::push(out, 44174);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72623);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 97605);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115969);
									s.is_empty()
								}
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
			super::push(out, 44175);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72624);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдия.
pub(crate) fn n19615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19510);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44176);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72625);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 72626);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 72627);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 115970);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127667);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 115971);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127668);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 115972);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127669);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 97608);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 115973);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127670);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134608);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44177);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72628);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97609);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 97610);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115974);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											super::push(out, 127671);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удия.
pub(crate) fn n19616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19511);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 44178);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44179);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 97611);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115975);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 127672);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134609);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n139996(s, out),
														_ => false,
													}
												}
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
							super::push(out, 97612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдия.
pub(crate) fn n19617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19512);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юдия.
pub(crate) fn n19618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19513);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44181);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -встралазия.
pub(crate) fn n136015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оидоклазия.
pub(crate) fn n136016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134611);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138563);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ондроплазия.
pub(crate) fn n136017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134612);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 138564);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					super::push(out, 140784);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -номазия.
pub(crate) fn n98673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97622);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 115980);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 127676);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 127677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ромазия.
pub(crate) fn n98674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97623);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 115981);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -назия.
pub(crate) fn n44614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44188);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72635);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 97624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 72636);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 97625);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 115982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -разия.
pub(crate) fn n44615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44189);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 72637);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 72638);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 97627);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115983);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 127679);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 134613);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 138565);
													match s {
														[] => true,
														['д', s @ ..] => super::t49::n142230(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72639);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97628);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 97629);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 115984);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 127680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тазия.
pub(crate) fn n44616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44190);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 72640);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 97630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 72641);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97631);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 115985);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 127681);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 134614);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138566);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n142231(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['я', s @ ..] => {
											super::push(out, 134615);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 138567);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n142232(s, out),
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
									super::push(out, 127682);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 134616);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уазия.
pub(crate) fn n44617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44191);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 72643);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 97632);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 115986);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 127683);
									s.is_empty()
								}
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
			super::push(out, 72644);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 97633);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 115987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фазия.
pub(crate) fn n44618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44192);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 97634);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 97635);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 97636);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 115990);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 115991);
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
				['г', s @ ..] => {
					super::push(out, 97637);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115992);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 97638);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 97639);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 72645);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 97640);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хазия.
pub(crate) fn n44619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44193);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 72646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -взия.
pub(crate) fn n19621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19516);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44194);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72647);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

