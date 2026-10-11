use flexion::Path;

/// Words ending -ечить.
pub(crate) fn n868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 857);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3540);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9458);
					match s {
						[] => true,
						['к', 'е', 'в', 'у', s @ ..] => {
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
						['л', 'е', 'ч', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
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
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16983);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 16984);
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
		['л', s @ ..] => {
			super::push(out, 3541);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9460);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9461);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16985);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 16986);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24360);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24361);
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
									super::push(out, 24362);
									match s {
										[] => true,
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
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9462);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16987);
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
					super::push(out, 9463);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16988);
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
					super::push(out, 9464);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16989);
							match s {
								[] => true,
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
					super::push(out, 9465);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16990);
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
							super::push(out, 16991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24363);
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
				['ы', s @ ..] => {
					super::push(out, 9466);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16992);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3542);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9467);
					match s {
						[] => true,
						['н', s @ ..] => {
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
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3543);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9468);
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
		['п', s @ ..] => {
			super::push(out, 3544);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9469);
					match s {
						[] => true,
						['е', 'б', 'о', 'е', 'н', 'з', 'и', 'ж', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3545);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9470);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24364);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31313);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9471);
					match s {
						[] => true,
						['в', 'и', 'т', 'о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24365);
									match s {
										[] => true,
										['з', s @ ..] => super::t14::n53514(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t14::n11389(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичить.
pub(crate) fn n869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 858);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3546);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9472);
					match s {
						[] => true,
						['ш', 'ь', 'л', 'о', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3547);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9473);
					match s {
						[] => true,
						['м', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9474);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9475);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16995);
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
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 9476);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16996);
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
						['у', s @ ..] => {
							super::push(out, 16997);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24367);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9477);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16998);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24368);
									match s {
										[] => true,
										['о', 'в', '\u{1}', s @ ..] => {
											super::push(out, 31315);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 24369);
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
				['з', s @ ..] => {
					super::push(out, 9478);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16999);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17000);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24370);
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
					super::push(out, 9479);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9480);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17001);
							match s {
								[] => true,
								['а', 'к', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9482);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17002);
							match s {
								[] => true,
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
					super::push(out, 9483);
					match s {
						[] => true,
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
			super::push(out, 3549);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9484);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24371);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24372);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24373);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24374);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9485);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24375);
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24376);
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
				['с', s @ ..] => {
					super::push(out, 9486);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17003);
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
		['п', s @ ..] => {
			super::push(out, 3551);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9487);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17004);
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
				['р', s @ ..] => {
					super::push(out, 9488);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17005);
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
				['с', s @ ..] => {
					super::push(out, 9489);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3552);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 9490);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17007);
							match s {
								[] => true,
								['в', 'г', 'н', 'и', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3553);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9491);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17008);
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
		['щ', s @ ..] => super::t14::n3748(s, out),
		_ => false,
	}
}

/// Words ending -кчить.
pub(crate) fn n870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 859);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3555);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9493);
					match s {
						[] => true,
						['е', 'щ', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олчить.
pub(crate) fn n3750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3556);
	s.is_empty()
}

/// Words ending -нчить.
pub(crate) fn n872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 861);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3557);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9495);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17011);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 31316);
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
				['п', s @ ..] => {
					super::push(out, 9496);
					match s {
						[] => true,
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
			super::push(out, 3558);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17012);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3559);
			match s {
				[] => true,
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17014);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17015);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24377);
									match s {
										[] => true,
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
							super::push(out, 17016);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24378);
									match s {
										[] => true,
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
							super::push(out, 17017);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24379);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
							super::push(out, 17018);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17019);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24380);
									match s {
										[] => true,
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
							super::push(out, 17020);
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
						['ы', s @ ..] => {
							super::push(out, 17021);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24381);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очить.
pub(crate) fn n873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 862);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9499);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9500);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17022);
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
				['о', s @ ..] => {
					super::push(out, 9501);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17023);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31317);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31318);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3561);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9503);
					match s {
						[] => true,
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
			super::push(out, 3562);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9504);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17025);
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
				['я', s @ ..] => {
					super::push(out, 9505);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17026);
							match s {
								[] => true,
								['о', 'п', 'у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31319);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 31320);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 31321);
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
		['к', s @ ..] => {
			super::push(out, 3563);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9506);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17027);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 24382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17028);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17029);
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
							super::push(out, 17030);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24383);
									match s {
										[] => true,
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
							super::push(out, 17031);
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
						['д', s @ ..] => {
							super::push(out, 17032);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17033);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24384);
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
							super::push(out, 17034);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17035);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 24385);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 24386);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31322);
											match s {
												[] => true,
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
									super::push(out, 24387);
									match s {
										[] => true,
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
							super::push(out, 17036);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17037);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17038);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24388);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31323);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36452);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n76911(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t14::n3758(s, out),
		['м', s @ ..] => super::t14::n3759(s, out),
		['н', s @ ..] => {
			super::push(out, 3566);
			match s {
				[] => true,
				['л', s @ ..] => super::t14::n11447(s, out),
				['с', s @ ..] => super::t14::n11448(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t14::n3761(s, out),
		['р', s @ ..] => super::t14::n3762(s, out),
		['с', s @ ..] => super::t14::n3763(s, out),
		['т', s @ ..] => super::t14::n3764(s, out),
		_ => false,
	}
}

/// Words ending -пчить.
pub(crate) fn n874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3571);
			match s {
				[] => true,
				['р', 'а', 'х', '\u{1}', s @ ..] => {
					super::push(out, 9549);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчить.
pub(crate) fn n875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 864);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3572);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 9550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17094);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17095);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24437);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 24438);
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
							super::push(out, 17096);
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
		['е', s @ ..] => {
			super::push(out, 3573);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17097);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17098);
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
			super::push(out, 3574);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17099);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17100);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 9553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17101);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17102);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24439);
									match s {
										[] => true,
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
							super::push(out, 17103);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24440);
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
							super::push(out, 17104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17105);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24441);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3575);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9554);
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

/// Words ending -тчить.
pub(crate) fn n876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 865);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3576);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9555);
					match s {
						[] => true,
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
			super::push(out, 3577);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9556);
					match s {
						[] => true,
						['у', 'з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учить.
pub(crate) fn n877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 866);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3578);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3579);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17106);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17107);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9559);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9560);
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
					super::push(out, 9561);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 17108);
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
				['з', s @ ..] => {
					super::push(out, 9562);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17109);
							match s {
								[] => true,
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
					super::push(out, 9563);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17110);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24442);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17111);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17112);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 17113);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 24443);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31341);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31342);
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
						['о', s @ ..] => {
							super::push(out, 17114);
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
				['с', s @ ..] => {
					super::push(out, 9565);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 17115);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24444);
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
				['т', s @ ..] => {
					super::push(out, 9566);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17116);
							match s {
								[] => true,
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
					super::push(out, 9567);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 17117);
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
				['ы', s @ ..] => {
					super::push(out, 9568);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17118);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3581);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9569);
					match s {
						[] => true,
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', 'а', 'л', 'б', 'о', '\u{1}', s @ ..] => {
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
		['г', s @ ..] => {
			super::push(out, 3582);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9570);
					match s {
						[] => true,
						['у', 'с', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3583);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9571);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17119);
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
					super::push(out, 9572);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17120);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3584);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9573);
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
		['ж', s @ ..] => {
			super::push(out, 3585);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9574);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9575);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9576);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9577);
					match s {
						[] => true,
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
			super::push(out, 3587);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9578);
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
		['к', s @ ..] => super::t14::n3782(s, out),
		['л', s @ ..] => super::t14::n3783(s, out),
		['м', s @ ..] => super::t14::n3784(s, out),
		['о', s @ ..] => {
			super::push(out, 3591);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9596);
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
		['п', s @ ..] => super::t14::n3786(s, out),
		['р', s @ ..] => super::t14::n3787(s, out),
		['с', s @ ..] => super::t14::n3788(s, out),
		['т', s @ ..] => super::t14::n3789(s, out),
		['щ', s @ ..] => super::t14::n3790(s, out),
		['ы', s @ ..] => super::t14::n3791(s, out),
		['я', s @ ..] => super::t14::n3792(s, out),
		_ => false,
	}
}

/// Words ending -ычить.
pub(crate) fn n878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 867);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3599);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9623);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3600);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17144);
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
				['о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'л', 'а', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9624);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 9625);
					match s {
						[] => true,
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
			super::push(out, 3601);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3602);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9626);
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

/// Words ending -ьчить.
pub(crate) fn n879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 868);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3603);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17145);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17146);
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
						['о', 'п', '\u{1}', s @ ..] => {
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
		['н', s @ ..] => {
			super::push(out, 3604);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 9627);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24452);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24453);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31343);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючить.
pub(crate) fn n880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 869);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3605);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9628);
					match s {
						[] => true,
						['и', 'п', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3606);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17147);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17148);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24454);
									match s {
										[] => true,
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
							super::push(out, 17149);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24455);
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
					super::push(out, 9630);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17150);
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
										['о', s @ ..] => {
											super::push(out, 31344);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 36458);
													s.is_empty()
												}
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
							super::push(out, 17151);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17152);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24456);
									s.is_empty()
								}
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
			super::push(out, 3607);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9631);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24457);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24458);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31345);
											match s {
												[] => true,
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
									super::push(out, 24459);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31346);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 31347);
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
									super::push(out, 24460);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31348);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3608);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17153);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17154);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['к', s @ ..] => {
					super::push(out, 9633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17155);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9634);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24462);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3609);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 9635);
					match s {
						[] => true,
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
			super::push(out, 3610);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9636);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17158);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17159);
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

/// Words ending -ячить.
pub(crate) fn n881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 870);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3611);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17160);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 17161);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17162);
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
							super::push(out, 17163);
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
		['д', s @ ..] => {
			super::push(out, 3612);
			match s {
				[] => true,
				['з', 'и', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17164);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17165);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24463);
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
							super::push(out, 17166);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24464);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3613);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9638);
					match s {
						[] => true,
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17167);
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
		['м', s @ ..] => {
			super::push(out, 3614);
			match s {
				[] => true,
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9639);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24465);
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
		['н', s @ ..] => {
			super::push(out, 3615);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9640);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17168);
							match s {
								[] => true,
								['р', 'е', 'с', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9641);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24466);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31349);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
									super::push(out, 24467);
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
					super::push(out, 9642);
					match s {
						[] => true,
						['о', 'к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9643);
					match s {
						[] => true,
						['р', 'е', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3616);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9644);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17169);
							match s {
								[] => true,
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
					super::push(out, 9645);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17170);
							match s {
								[] => true,
								['и', 'п', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9646);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24468);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 24469);
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
						['к', s @ ..] => {
							super::push(out, 17172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24470);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 24471);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t14::n38725(s, out),
								['с', s @ ..] => super::t14::n38726(s, out),
								_ => false,
							}
						}
						['м', s @ ..] => super::t14::n23866(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t14::n11597(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t14::n3812(s, out),
		['у', s @ ..] => super::t14::n3813(s, out),
		['ь', s @ ..] => super::t14::n3814(s, out),
		_ => false,
	}
}

/// Words ending -ешить.
pub(crate) fn n887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 876);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9661);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 9662);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9663);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9664);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3629);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9665);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24493);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24494);
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
				['к', s @ ..] => {
					super::push(out, 9666);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17189);
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
				['п', s @ ..] => {
					super::push(out, 9667);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17190);
							match s {
								[] => true,
								['р', 'т', 'н', 'о', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3630);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9668);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17191);
							match s {
								[] => true,
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
					super::push(out, 9669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17192);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17193);
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
		['о', s @ ..] => {
			super::push(out, 3631);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9670);
					match s {
						[] => true,
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
			super::push(out, 3632);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9671);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17194);
							match s {
								[] => true,
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
					super::push(out, 9672);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17195);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17196);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24495);
									match s {
										[] => true,
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
							super::push(out, 17197);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24496);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9674);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17198);
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
		['р', s @ ..] => {
			super::push(out, 3633);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 9675);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17199);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17200);
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
				['д', s @ ..] => {
					super::push(out, 9676);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17201);
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
					super::push(out, 9677);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17203);
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
					super::push(out, 9678);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17204);
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
					super::push(out, 9679);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 17206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24497);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24498);
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
						['п', s @ ..] => {
							super::push(out, 17207);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9680);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17208);
							match s {
								[] => true,
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
					super::push(out, 9681);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t14::n3830(s, out),
		_ => false,
	}
}

/// The word шамшить.
pub(crate) fn n23929(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -емшить.
pub(crate) fn n3837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3641);
	match s {
		[] => true,
		['р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омшить.
pub(crate) fn n3838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3642);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ошить.
pub(crate) fn n891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 880);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3643);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9694);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17213);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24500);
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
						['р', s @ ..] => {
							super::push(out, 17214);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24501);
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
				['е', s @ ..] => {
					super::push(out, 9695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17215);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 17216);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 24502);
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
		['д', s @ ..] => {
			super::push(out, 3644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9696);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9697);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17217);
							match s {
								[] => true,
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
					super::push(out, 9698);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17218);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31356);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31357);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36461);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3645);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9699);
					match s {
						[] => true,
						['к', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9700);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9701);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24503);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24504);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3647);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9702);
					match s {
						[] => true,
						['о', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24505);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24506);
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
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24507);
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
									super::push(out, 24508);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31358);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3648);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9703);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24509);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24510);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9704);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17219);
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
					super::push(out, 9705);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17220);
							match s {
								[] => true,
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
					super::push(out, 9706);
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
		['р', s @ ..] => super::t14::n3846(s, out),
		['т', s @ ..] => {
			super::push(out, 3651);
			match s {
				[] => true,
				['а', s @ ..] => super::t14::n11672(s, out),
				['о', s @ ..] => super::t14::n11673(s, out),
				['с', s @ ..] => super::t14::n11674(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ршить.
pub(crate) fn n892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 881);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3652);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9716);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17240);
							match s {
								[] => true,
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24526);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24527);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31366);
											match s {
												[] => true,
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
									super::push(out, 24528);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31367);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3653);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9717);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 9718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17241);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17242);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 17243);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24529);
									match s {
										[] => true,
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
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17244);
							match s {
								[] => true,
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
							super::push(out, 17245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17246);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24530);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17247);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24531);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ъ', 'з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3654);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9720);
					match s {
						[] => true,
						['е', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24532);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31368);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сшить.
pub(crate) fn n893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 882);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3655);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3656);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9721);
					match s {
						[] => true,
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
			super::push(out, 3657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тшить.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 883);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ушить.
pub(crate) fn n895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 884);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3659);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3660);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9722);
					match s {
						[] => true,
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
			super::push(out, 3661);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9723);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17248);
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
		['в', s @ ..] => {
			super::push(out, 3662);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9724);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17249);
							match s {
								[] => true,
								['о', 'г', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3663);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9725);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9726);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9727);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9728);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17250);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17251);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24533);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9729);
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
				['у', s @ ..] => {
					super::push(out, 9730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9731);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17253);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3664);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9732);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 17254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3665);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9733);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17255);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17256);
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
							super::push(out, 17257);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17258);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17259);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24534);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 24535);
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
									super::push(out, 24536);
									match s {
										[] => true,
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
							super::push(out, 17260);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24537);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9734);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24538);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24539);
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
								['о', s @ ..] => {
									super::push(out, 24540);
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
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24541);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31369);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3666);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t14::n3863(s, out),
		['п', s @ ..] => super::t14::n3864(s, out),
		['р', s @ ..] => super::t14::n3865(s, out),
		['с', s @ ..] => super::t14::n3866(s, out),
		['т', s @ ..] => super::t14::n3867(s, out),
		_ => false,
	}
}

/// Words ending -чшить.
pub(crate) fn n896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 885);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 3672);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9760);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышить.
pub(crate) fn n897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 886);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьшить.
pub(crate) fn n898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 887);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3675);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9761);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 17279);
							match s {
								[] => true,
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
					super::push(out, 9762);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17280);
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
		['н', s @ ..] => {
			super::push(out, 3676);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9763);
					match s {
						[] => true,
						['м', 'у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24551);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31374);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юшить.
pub(crate) fn n899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 888);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 3677);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9764);
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
			super::push(out, 3678);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9765);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17281);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17282);
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

/// Words ending -яшить.
pub(crate) fn n900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 889);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3679);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9766);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24552);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 24553);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3680);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3681);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 9767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word затаращить.
pub(crate) fn n66642(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word растаращить.
pub(crate) fn n66643(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word вытаращить.
pub(crate) fn n66644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36466);
	s.is_empty()
}

/// The word понатащить.
pub(crate) fn n66645(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word повытащить.
pub(crate) fn n66646(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -бвощить.
pub(crate) fn n11761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9776);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -двощить.
pub(crate) fn n11762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9777);
	match s {
		[] => true,
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овощить.
pub(crate) fn n11763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9778);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывощить.
pub(crate) fn n11764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9779);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лощить.
pub(crate) fn n3883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3687);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9780);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9781);
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
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 9782);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17287);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17288);
					match s {
						[] => true,
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
			super::push(out, 9783);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17289);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тощить.
pub(crate) fn n3884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3688);
	s.is_empty()
}

/// Words ending -рщить.
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 894);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3689);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9784);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17290);
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
			super::push(out, 3690);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9785);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17291);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17292);
							match s {
								[] => true,
								['д', 'е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17293);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17294);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24560);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 24561);
									match s {
										[] => true,
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
							super::push(out, 17295);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24562);
									match s {
										[] => true,
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
							super::push(out, 17296);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24563);
									match s {
										[] => true,
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
							super::push(out, 17297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24564);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17298);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17299);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24565);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9787);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17300);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31380);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31381);
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
											super::push(out, 31382);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тщить.
pub(crate) fn n906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 895);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ущить.
pub(crate) fn n907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 896);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9788);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9789);
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
					super::push(out, 9790);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9791);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9792);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9793);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9794);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9796);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17302);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ющить.
pub(crate) fn n908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 897);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3693);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17303);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17304);
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
							super::push(out, 17305);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17306);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17307);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24566);
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 24567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17309);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ящить.
pub(crate) fn n909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 898);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3694);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9797);
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

/// Words ending -бчесть.
pub(crate) fn n3939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3726);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9851);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ечесть.
pub(crate) fn n3940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3727);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9852);
			match s {
				[] => true,
				['е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 17330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичесть.
pub(crate) fn n3941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3728);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9853);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24572);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31383);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очесть.
pub(crate) fn n3942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3729);
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
				['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -счесть.
pub(crate) fn n3943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3730);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9854);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тчесть.
pub(crate) fn n3944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3731);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9855);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учесть.
pub(crate) fn n3945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3732);
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

/// Words ending -ычесть.
pub(crate) fn n3946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3733);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъесть.
pub(crate) fn n928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 915);
	match s {
		[] => true,
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыесть.
pub(crate) fn n929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 916);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игануть.
pub(crate) fn n11937(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 17342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'т', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ргануть.
pub(crate) fn n11939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9873);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17343);
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

/// Words ending -дануть.
pub(crate) fn n3977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3747);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9874);
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 17344);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17345);
					match s {
						[] => true,
						['п', s @ ..] => {
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
				['о', 'в', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9875);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жануть.
pub(crate) fn n3978(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9876);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'и', 'р', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зануть.
pub(crate) fn n3979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3748);
	match s {
		[] => true,
		['р', 'е', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 9877);
			match s {
				[] => true,
				['р', 'г', '\u{1}', s @ ..] => {
					super::push(out, 17348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кануть.
pub(crate) fn n3980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3749);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9878);
			s.is_empty()
		}
		['а', 'к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', 'т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'а', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'л', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'у', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ч', s @ ..] => {
			match s {
				[] => true,
				['а', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', 'и', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word числануть.
pub(crate) fn n3981(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -мануть.
pub(crate) fn n3982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3750);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9879);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9880);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'ж', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9881);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['у', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word клинануть.
pub(crate) fn n3983(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -пануть.
pub(crate) fn n3984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3751);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', '\u{1}', s @ ..] => {
					super::push(out, 17351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9882);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 17352);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'щ', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['й', 'а', 'х', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'е', 'х', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рануть.
pub(crate) fn n3985(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 9883);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 17353);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 24575);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9884);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
				_ => false,
			}
		}
		['ы', 'в', 'ш', '\u{1}', s @ ..] => {
			super::push(out, 9885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сануть.
pub(crate) fn n3986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3752);
	match s {
		[] => true,
		['а', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9886);
			s.is_empty()
		}
		['и', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9887);
			match s {
				[] => true,
				['е', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'е', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 9888);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', 'л', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тануть.
pub(crate) fn n3987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3753);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			super::push(out, 9889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9890);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17355);
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
		['л', 'о', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 24577);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17356);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 24578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'п', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					super::push(out, 17357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17358);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 24579);
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
		['у', 'р', 'к', '\u{1}', s @ ..] => {
			super::push(out, 9891);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 9892);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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

/// Words ending -фануть.
pub(crate) fn n3988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3754);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9893);
			match s {
				[] => true,
				['р', 'т', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 17359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9894);
			match s {
				[] => true,
				['л', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9895);
			match s {
				[] => true,
				['л', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 9896);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17360);
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
		['у', s @ ..] => {
			super::push(out, 9897);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 17361);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хануть.
pub(crate) fn n3989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3755);
	match s {
		[] => true,
		['у', s @ ..] => {
			match s {
				[] => true,
				['р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word борщануть.
pub(crate) fn n3990(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -бнуть.
pub(crate) fn n948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 933);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3756);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9898);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24580);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24581);
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
		['е', s @ ..] => {
			super::push(out, 3757);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9899);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9900);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17362);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 24582);
									match s {
										[] => true,
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
							super::push(out, 17363);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24583);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31384);
											match s {
												[] => true,
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
									super::push(out, 24584);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24585);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24586);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 24587);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31385);
											match s {
												[] => true,
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
									super::push(out, 24588);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 24589);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31386);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9901);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9902);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 9903);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17364);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17365);
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
				['ы', s @ ..] => {
					super::push(out, 9904);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17366);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3758);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17367);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17368);
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
				['ш', s @ ..] => {
					super::push(out, 9906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17369);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24590);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3759);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9907);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17370);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31387);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31388);
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
		['у', s @ ..] => {
			super::push(out, 3760);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9908);
					match s {
						[] => true,
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
			super::push(out, 3761);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t14::n3997(s, out),
		_ => false,
	}
}

/// Words ending -внуть.
pub(crate) fn n949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 934);
	match s {
		[] => true,
		['а', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3763);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9912);
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
				['л', s @ ..] => {
					super::push(out, 9913);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17379);
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
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3764);
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

/// Words ending -гнуть.
pub(crate) fn n950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 935);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3765);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3766);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9914);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 17380);
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
				['н', s @ ..] => {
					super::push(out, 9916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17381);
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
				['ш', s @ ..] => {
					super::push(out, 9917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 17382);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24596);
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
							super::push(out, 17383);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
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
						['ы', s @ ..] => {
							super::push(out, 17384);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24597);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3767);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9918);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17385);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24598);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31389);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17386);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24599);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 31390);
											s.is_empty()
										}
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
					super::push(out, 9919);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17387);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9920);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17388);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24601);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9921);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17389);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24602);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 24603);
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
								['и', s @ ..] => {
									super::push(out, 24604);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24605);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31391);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 31392);
											match s {
												[] => true,
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
									super::push(out, 24606);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 24607);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 24608);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31393);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t14::n4004(s, out),
		['и', s @ ..] => super::t14::n4005(s, out),
		['л', s @ ..] => {
			super::push(out, 3770);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9931);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17397);
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
					super::push(out, 9932);
					match s {
						[] => true,
						['в', s @ ..] => super::t14::n24312(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t15::n4007(s, out),
		['р', s @ ..] => super::t15::n4008(s, out),
		['у', s @ ..] => super::t15::n4009(s, out),
		['ы', s @ ..] => super::t15::n4010(s, out),
		['ю', s @ ..] => super::t15::n4011(s, out),
		['я', s @ ..] => super::t15::n4012(s, out),
		_ => false,
	}
}

/// Words ending -днуть.
pub(crate) fn n951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3777);
			match s {
				[] => true,
				['л', 'б', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3778);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9956);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24649);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24650);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 24651);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3779);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9957);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17425);
							match s {
								[] => true,
								['и', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3780);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9958);
					match s {
						[] => true,
						['р', 'г', 'п', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3781);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3782);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9960);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17426);
							match s {
								[] => true,
								['р', 'о', 'ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9961);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3783);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 9962);
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
		['ы', s @ ..] => {
			super::push(out, 3784);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9963);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енуть.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 937);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3785);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жнуть.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 938);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 3786);
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

/// Words ending -знуть.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 939);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3787);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9964);
					match s {
						[] => true,
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
			super::push(out, 3788);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9965);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 9966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17429);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17430);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24652);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3789);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 17431);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 24653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17432);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24654);
									match s {
										[] => true,
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
							super::push(out, 17433);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24655);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31416);
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
						['к', s @ ..] => {
							super::push(out, 17434);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24656);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31417);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 17435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24657);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24658);
									match s {
										[] => true,
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
							super::push(out, 17436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3790);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3791);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17437);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 17438);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24659);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 24660);
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
						['д', s @ ..] => {
							super::push(out, 17439);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 17440);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24662);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24663);
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
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24664);
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24665);
									match s {
										[] => true,
										['в', s @ ..] => super::t15::n53813(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => super::t15::n24379(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t15::n4028(s, out),
		['ы', s @ ..] => super::t15::n4029(s, out),
		['ь', s @ ..] => super::t15::n4030(s, out),
		['я', s @ ..] => super::t15::n4031(s, out),
		_ => false,
	}
}

/// Words ending -инуть.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 940);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3796);
			match s {
				[] => true,
				['д', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
							super::push(out, 17451);
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
								['т', 'о', '\u{1}', s @ ..] => {
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
							super::push(out, 17452);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24683);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31427);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9976);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9977);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9978);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9979);
					match s {
						[] => true,
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
			super::push(out, 3798);
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
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3799);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9980);
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
								['ы', s @ ..] => {
									super::push(out, 24684);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31428);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9981);
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
						['в', '\u{1}', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9982);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17453);
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
		['л', s @ ..] => {
			super::push(out, 3800);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t15::n4037(s, out),
		['р', s @ ..] => super::t15::n4038(s, out),
		_ => false,
	}
}

/// Words ending -йнуть.
pub(crate) fn n956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 941);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3803);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9987);
					match s {
						[] => true,
						['п', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3804);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 9988);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кнуть.
pub(crate) fn n957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 942);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9989);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 9990);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17456);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24685);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9991);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17457);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 17458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 17459);
							match s {
								[] => true,
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
					super::push(out, 9992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17460);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17461);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 24686);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 17462);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17463);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24687);
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
				['ж', s @ ..] => {
					super::push(out, 9994);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17464);
							match s {
								[] => true,
								['е', 'х', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9995);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17465);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', 'и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24688);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31429);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 17466);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24689);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31430);
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
									super::push(out, 24690);
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
				['м', s @ ..] => {
					super::push(out, 9997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 17467);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24691);
									match s {
										[] => true,
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
						['о', s @ ..] => {
							super::push(out, 17468);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9998);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17469);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24692);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17470);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17471);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 24693);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3806);
			match s {
				[] => true,
				['я', s @ ..] => super::t15::n12118(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t15::n4043(s, out),
		['и', s @ ..] => super::t15::n4044(s, out),
		['й', s @ ..] => super::t15::n4045(s, out),
		['л', s @ ..] => super::t15::n4046(s, out),
		['м', s @ ..] => super::t15::n4047(s, out),
		['о', s @ ..] => super::t15::n4048(s, out),
		['р', s @ ..] => super::t15::n4049(s, out),
		['с', s @ ..] => super::t15::n4050(s, out),
		['т', s @ ..] => super::t15::n4051(s, out),
		['у', s @ ..] => super::t15::n4052(s, out),
		['ц', s @ ..] => {
			super::push(out, 3817);
			match s {
				[] => true,
				['ы', s @ ..] => super::t15::n12193(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t15::n4054(s, out),
		['ш', s @ ..] => super::t15::n4055(s, out),
		['ы', s @ ..] => super::t15::n4056(s, out),
		['ь', s @ ..] => super::t15::n4057(s, out),
		['э', s @ ..] => super::t15::n4058(s, out),
		['ю', s @ ..] => super::t15::n4059(s, out),
		['я', s @ ..] => super::t15::n4060(s, out),
		_ => false,
	}
}

/// Words ending -мнуть.
pub(crate) fn n958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 943);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3825);
			match s {
				[] => true,
				['р', 'д', s @ ..] => {
					match s {
						[] => true,
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
		['у', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онуть.
pub(crate) fn n959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 944);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3826);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 10104);
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
			super::push(out, 3827);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10105);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10106);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17615);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 17616);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24801);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 24802);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24803);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3828);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 10107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17617);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24804);
									s.is_empty()
								}
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
			super::push(out, 3829);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10108);
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
		['т', s @ ..] => {
			super::push(out, 3830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10109);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10110);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10111);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17619);
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
					super::push(out, 10112);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пнуть.
pub(crate) fn n960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 945);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 10114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10115);
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
						['с', s @ ..] => {
							super::push(out, 17621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 24805);
									match s {
										[] => true,
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
				['л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10116);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17622);
							match s {
								[] => true,
								['ц', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 17623);
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
				['т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 10117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 10118);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3832);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10120);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24806);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24807);
									match s {
										[] => true,
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
							super::push(out, 17625);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
					super::push(out, 10121);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17626);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 17627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24808);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31483);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36490);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n76933(s, out),
														_ => false,
													}
												}
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
							super::push(out, 17628);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 10122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 17629);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24811);
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
							super::push(out, 17630);
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
		['и', s @ ..] => super::t15::n4071(s, out),
		['й', s @ ..] => {
			super::push(out, 3834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10128);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24821);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t15::n4073(s, out),
		['р', s @ ..] => super::t15::n4074(s, out),
		['с', s @ ..] => {
			super::push(out, 3837);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10138);
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
		['у', s @ ..] => super::t15::n4076(s, out),
		['ы', s @ ..] => {
			super::push(out, 3839);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10141);
					match s {
						[] => true,
						['к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t15::n4078(s, out),
		['я', s @ ..] => super::t15::n4079(s, out),
		_ => false,
	}
}

/// Words ending -рнуть.
pub(crate) fn n961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 946);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3842);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10144);
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
		['е', s @ ..] => {
			super::push(out, 3843);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10145);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17663);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17664);
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
							super::push(out, 17665);
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
							super::push(out, 17666);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24839);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 24840);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31493);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17667);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17668);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24841);
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
				['г', s @ ..] => {
					super::push(out, 10147);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17669);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24842);
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
					super::push(out, 10148);
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
								['в', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17670);
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
						['ы', s @ ..] => {
							super::push(out, 17671);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24843);
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
				['л', s @ ..] => {
					super::push(out, 10149);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 17672);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24844);
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
				['п', s @ ..] => {
					super::push(out, 10150);
					s.is_empty()
				}
				['т', s @ ..] => super::t15::n12281(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t15::n4082(s, out),
		['о', s @ ..] => super::t15::n4083(s, out),
		['у', s @ ..] => super::t15::n4084(s, out),
		['ы', s @ ..] => super::t15::n4085(s, out),
		_ => false,
	}
}

/// Words ending -снуть.
pub(crate) fn n962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3848);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3849);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17684);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17685);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17686);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17687);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24857);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31509);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17688);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17689);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24858);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 24859);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 24860);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31510);
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
									super::push(out, 24861);
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
				['ж', s @ ..] => {
					super::push(out, 10171);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 10172);
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
				['п', s @ ..] => {
					super::push(out, 10173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17691);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24863);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3850);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10174);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17692);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24864);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31511);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31512);
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
							super::push(out, 17693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24865);
									s.is_empty()
								}
								['а', s @ ..] => super::t15::n39572(s, out),
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t15::n39575(s, out),
								['и', 'р', 'п', '\u{1}', s @ ..] => super::t15::n39576(s, out),
								['о', s @ ..] => super::t15::n39577(s, out),
								['с', s @ ..] => super::t15::n39578(s, out),
								['т', 'о', '\u{1}', s @ ..] => super::t15::n39579(s, out),
								['ы', s @ ..] => super::t15::n39580(s, out),
								_ => false,
							}
						}
						['х', s @ ..] => super::t15::n24810(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t15::n12306(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t15::n4089(s, out),
		['к', s @ ..] => super::t15::n4090(s, out),
		['о', s @ ..] => super::t15::n4091(s, out),
		['р', s @ ..] => super::t15::n4092(s, out),
		['у', s @ ..] => super::t15::n4093(s, out),
		['ы', s @ ..] => super::t15::n4094(s, out),
		['ю', s @ ..] => super::t15::n4095(s, out),
		['я', s @ ..] => super::t15::n4096(s, out),
		_ => false,
	}
}

/// Words ending -тнуть.
pub(crate) fn n963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3859);
			match s {
				[] => true,
				['в', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 10193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 10194);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17716);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24889);
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
				['у', 'л', 'п', 'с', 'к', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 10195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17717);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17718);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24890);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3860);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10196);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17719);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17720);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24891);
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
				['ж', 'е', 'р', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 10197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17721);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 17722);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 17723);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24892);
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
							super::push(out, 17724);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 17725);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24893);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31527);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17726);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17727);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 24894);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 24895);
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
							super::push(out, 17728);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17729);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24896);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3861);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10199);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 24897);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											super::push(out, 31528);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3862);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10200);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24898);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 24899);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24900);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 24901);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31529);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t15::n4101(s, out),
		['с', s @ ..] => super::t15::n4102(s, out),
		['у', s @ ..] => {
			super::push(out, 3865);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10214);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17746);
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
					super::push(out, 10215);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10216);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17747);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3866);
			match s {
				[] => true,
				['а', s @ ..] => super::t15::n12359(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t15::n4105(s, out),
		_ => false,
	}
}

/// Words ending -унуть.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3868);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10218);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10219);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10221);
					match s {
						[] => true,
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
			super::push(out, 3869);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3870);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10223);
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
					super::push(out, 10224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10225);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10226);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17748);
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
					super::push(out, 10227);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17749);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17750);
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
				['т', s @ ..] => {
					super::push(out, 10229);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17751);
							match s {
								[] => true,
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
					super::push(out, 10230);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17752);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24916);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31543);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фнуть.
pub(crate) fn n965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 950);
	match s {
		[] => true,
		['е', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3871);
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

/// Words ending -хнуть.
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10231);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17753);
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'р', 'у', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 10233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 10234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17754);
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
							super::push(out, 17755);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24917);
									match s {
										[] => true,
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
							super::push(out, 17756);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24918);
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
							super::push(out, 17757);
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 24919);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 24920);
									match s {
										[] => true,
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
							super::push(out, 17758);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17759);
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
							super::push(out, 17760);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17761);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24921);
									match s {
										[] => true,
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
							super::push(out, 17762);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24922);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17763);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17764);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24923);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31544);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17765);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24924);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 24925);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31545);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36510);
													s.is_empty()
												}
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
							super::push(out, 17766);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24926);
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
							super::push(out, 17767);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24927);
									match s {
										[] => true,
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
							super::push(out, 17768);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24928);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10236);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17769);
							match s {
								[] => true,
								['д', 'н', 'а', 'ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'а', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'б', 'е', 'ч', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => super::t15::n12383(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3873);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t15::n4113(s, out),
		['и', s @ ..] => super::t15::n4114(s, out),
		['о', s @ ..] => super::t15::n4115(s, out),
		['п', s @ ..] => super::t15::n4116(s, out),
		['р', s @ ..] => super::t15::n4117(s, out),
		['у', s @ ..] => super::t15::n4118(s, out),
		['ч', s @ ..] => {
			super::push(out, 3880);
			s.is_empty()
		}
		['ы', s @ ..] => super::t15::n4120(s, out),
		['э', s @ ..] => super::t15::n4121(s, out),
		['ю', s @ ..] => super::t15::n4122(s, out),
		['я', s @ ..] => super::t15::n4123(s, out),
		_ => false,
	}
}

/// Words ending -цнуть.
pub(crate) fn n967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 952);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3885);
			match s {
				[] => true,
				['л', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3886);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнуть.
pub(crate) fn n968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 953);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17846);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24994);
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
							super::push(out, 17847);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17848);
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
			super::push(out, 3888);
			match s {
				[] => true,
				['р', 'д', 'з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3889);
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

/// Words ending -шнуть.
pub(crate) fn n969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3890);
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

/// Words ending -ынуть.
pub(crate) fn n970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 955);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10269);
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
		['з', s @ ..] => {
			super::push(out, 3892);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 10270);
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
		['л', s @ ..] => {
			super::push(out, 3893);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 10271);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 10272);
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
								['о', s @ ..] => {
									super::push(out, 24995);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31569);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 17849);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17850);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 24996);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['т', s @ ..] => {
							super::push(out, 17851);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24997);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3894);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 10273);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17852);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17853);
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
							super::push(out, 17854);
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
								['п', s @ ..] => {
									super::push(out, 24998);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31570);
											match s {
												[] => true,
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьнуть.
pub(crate) fn n971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 956);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10274);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10275);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17855);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10276);
					match s {
						[] => true,
						['в', 'е', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
						['р', 'т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10277);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17856);
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 17857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25000);
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
				['о', s @ ..] => {
					super::push(out, 10278);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 25001);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31571);
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
					super::push(out, 10279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнуть.
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 957);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3896);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17858);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17859);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 17860);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17861);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17863);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25002);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17864);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17865);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 25003);
									match s {
										[] => true,
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
							super::push(out, 17866);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25004);
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
							super::push(out, 17867);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25005);
									match s {
										[] => true,
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
							super::push(out, 17868);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17869);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25007);
									match s {
										[] => true,
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
							super::push(out, 17870);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25008);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31572);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -януть.
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 958);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3897);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10282);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10283);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25009);
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
					super::push(out, 10284);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10285);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17871);
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
		['л', s @ ..] => {
			super::push(out, 3898);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17872);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17873);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17874);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25010);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31573);
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
							super::push(out, 17875);
							match s {
								[] => true,
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
							super::push(out, 17876);
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
							super::push(out, 17877);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25011);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10287);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17878);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31574);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 31575);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 36520);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t15::n76955(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 31576);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31577);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 31578);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 31579);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 36521);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n76956(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3899);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10288);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25012);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31580);
											match s {
												[] => true,
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
										['в', s @ ..] => {
											super::push(out, 31581);
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
								['у', s @ ..] => {
									super::push(out, 25013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t15::n4139(s, out),
		['т', s @ ..] => super::t15::n4140(s, out),
		_ => false,
	}
}

/// Words ending -блять.
pub(crate) fn n1051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1021);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4016);
			match s {
				[] => true,
				['л', 'с', s @ ..] => {
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
						_ => false,
					}
				}
				['р', 'г', s @ ..] => {
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4017);
			match s {
				[] => true,
				['р', 'т', 'о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17918);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25027);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 31586);
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
		['о', s @ ..] => {
			super::push(out, 4018);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10364);
					match s {
						[] => true,
						['о', 'п', 'с', s @ ..] => {
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
				['н', 'з', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10365);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10366);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17919);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25028);
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
								['б', s @ ..] => {
									super::push(out, 25029);
									s.is_empty()
								}
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31587);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4019);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10367);
					match s {
						[] => true,
						['щ', 'ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10368);
					match s {
						[] => true,
						['к', 'с', 'о', '\u{1}', s @ ..] => {
							super::push(out, 17920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4020);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['з', 'и', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'г', s @ ..] => {
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
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
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
		['ы', 'р', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ю', 'л', s @ ..] => {
			match s {
				[] => true,
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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

/// Words ending -влять.
pub(crate) fn n1052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1022);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4021);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10369);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17921);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 25030);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31588);
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
				['в', 'о', 'р', 'к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 10370);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
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
				['р', s @ ..] => {
					super::push(out, 10371);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25031);
									match s {
										[] => true,
										['з', s @ ..] => super::t15::n54281(s, out),
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t15::n66857(s, out),
												['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t15::n66858(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => super::t15::n39944(s, out),
								['о', s @ ..] => super::t15::n39945(s, out),
								['с', s @ ..] => super::t15::n39946(s, out),
								['т', 'о', s @ ..] => super::t15::n39947(s, out),
								['у', s @ ..] => super::t15::n39948(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t15::n25259(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t15::n12676(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t15::n4325(s, out),
		['з', s @ ..] => super::t15::n4326(s, out),
		['и', s @ ..] => super::t15::n4327(s, out),
		['л', 'о', 'м', s @ ..] => super::t15::n4328(s, out),
		['о', s @ ..] => super::t15::n4329(s, out),
		['т', s @ ..] => super::t15::n4330(s, out),
		['я', s @ ..] => super::t15::n4331(s, out),
		_ => false,
	}
}

/// Words ending -круглять.
pub(crate) fn n4332(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -длять.
pub(crate) fn n1054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1024);
	match s {
		[] => true,
		['е', 'м', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['п', 'о', '\u{1}', s @ ..] => {
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

/// Words ending -елять.
pub(crate) fn n1055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1025);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4027);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 4028);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10378);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
										_ => false,
									}
								}
								_ => false,
							}
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 17929);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25043);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31604);
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
		['ж', s @ ..] => {
			super::push(out, 4029);
			match s {
				[] => true,
				['к', 'и', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['я', 'т', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['м', s @ ..] => {
			super::push(out, 4030);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4031);
			match s {
				[] => true,
				['е', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4032);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 10380);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25044);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25045);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31605);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 31606);
											match s {
												[] => true,
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
									super::push(out, 25046);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31607);
											match s {
												[] => true,
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
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25047);
									match s {
										[] => true,
										['д', s @ ..] => super::t15::n54337(s, out),
										['п', s @ ..] => super::t15::n54338(s, out),
										['р', 'п', '\u{1}', s @ ..] => super::t15::n54339(s, out),
										_ => false,
									}
								}
								['с', s @ ..] => super::t15::n40003(s, out),
								['т', s @ ..] => super::t15::n40004(s, out),
								['ы', s @ ..] => super::t15::n40005(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t15::n4341(s, out),
		['ц', s @ ..] => super::t15::n4342(s, out),
		['щ', 'с', 'а', 'р', '\u{1}', s @ ..] => super::t15::n4343(s, out),
		_ => false,
	}
}

/// Words ending -илять.
pub(crate) fn n1056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1026);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4035);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10382);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10383);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10384);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10385);
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
			super::push(out, 4036);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10386);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10387);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 17930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4037);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 10388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4038);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 10389);
					match s {
						[] => true,
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
				['и', s @ ..] => {
					super::push(out, 10390);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17931);
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
					super::push(out, 10391);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17932);
							match s {
								[] => true,
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
					super::push(out, 10392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клять.
pub(crate) fn n1057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1027);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4039);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 10393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4040);
			match s {
				[] => true,
				['т', 'с', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млять.
pub(crate) fn n1058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1028);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', 'х', s @ ..] => {
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
				['р', 'с', 'о', s @ ..] => {
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
		['е', 'щ', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', 'е', 'в', 'у', s @ ..] => {
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
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'о', s @ ..] => {
			match s {
				[] => true,
				['к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ф', 'о', s @ ..] => {
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
		['у', s @ ..] => {
			match s {
				[] => true,
				['з', 'а', 'р', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'д', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['я', 'р', 'п', 'с', s @ ..] => {
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

