use flexion::Path;

/// Words ending -разить.
pub(crate) fn n3008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2818);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7783);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7784);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 23161);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30806);
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7785);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23162);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 23163);
							match s {
								[] => true,
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 23164);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30807);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36308);
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
								['р', s @ ..] => {
									super::push(out, 30808);
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
							super::push(out, 23165);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30809);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 36309);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39764);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 39765);
													match s {
														[] => true,
														['н', s @ ..] => super::t13::n83306(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 39766);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n83307(s, out),
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
									super::push(out, 30810);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 30811);
									match s {
										[] => true,
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
							super::push(out, 23166);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30812);
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
								['г', s @ ..] => {
									super::push(out, 30813);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 36310);
											match s {
												[] => true,
												['л', 'б', s @ ..] => super::t13::n76782(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => super::t13::n52045(s, out),
								['о', s @ ..] => super::t13::n52046(s, out),
								['с', s @ ..] => super::t13::n52047(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t13::n35877(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t13::n9445(s, out),
		['о', s @ ..] => super::t13::n9446(s, out),
		['с', s @ ..] => super::t13::n9447(s, out),
		['т', s @ ..] => super::t13::n9448(s, out),
		['ы', s @ ..] => super::t13::n9449(s, out),
		_ => false,
	}
}

/// Words ending -бзить.
pub(crate) fn n704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2820);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7792);
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
		['ы', s @ ..] => {
			super::push(out, 2821);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7793);
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

/// Words ending -езить.
pub(crate) fn n705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2822);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7794);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23170);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23171);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23172);
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
		['л', s @ ..] => {
			super::push(out, 2823);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7795);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 15165);
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
				['с', s @ ..] => {
					super::push(out, 7796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15166);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15167);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23173);
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
		['р', s @ ..] => {
			super::push(out, 2824);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15168);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15169);
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
					super::push(out, 7798);
					match s {
						[] => true,
						['с', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изить.
pub(crate) fn n706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 695);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2825);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7799);
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
		['л', s @ ..] => {
			super::push(out, 2826);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15170);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15171);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15172);
							match s {
								[] => true,
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
					super::push(out, 7800);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15173);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2827);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15174);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 15175);
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
		['н', s @ ..] => {
			super::push(out, 2828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7801);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2829);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7802);
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

/// Words ending -ейзить.
pub(crate) fn n3020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2830);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7803);
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

/// Words ending -мзить.
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 697);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 2831);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7804);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15176);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нзить.
pub(crate) fn n709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 698);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -озить.
pub(crate) fn n710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 699);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7805);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7806);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 15179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23175);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23176);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30818);
											match s {
												[] => true,
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
									super::push(out, 23177);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7807);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7808);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7809);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15181);
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
					super::push(out, 7810);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 15182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7811);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15183);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									super::push(out, 23178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 15184);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 15185);
							match s {
								[] => true,
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
					super::push(out, 7812);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15186);
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
					super::push(out, 7813);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23179);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23180);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30819);
											match s {
												[] => true,
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
									super::push(out, 23181);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30820);
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
					super::push(out, 7814);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23182);
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 15189);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 15190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7816);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 15192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7818);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23184);
									s.is_empty()
								}
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
			super::push(out, 2834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7819);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15195);
							match s {
								[] => true,
								['а', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15196);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15197);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15198);
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
			super::push(out, 2835);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15199);
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2836);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15200);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15201);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 23185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15202);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => super::t13::n35918(s, out),
								_ => false,
							}
						}
						['ъ', s @ ..] => super::t13::n20828(s, out),
						_ => false,
					}
				}
				['ю', s @ ..] => super::t13::n9490(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t13::n3027(s, out),
		['н', s @ ..] => super::t13::n3028(s, out),
		['р', s @ ..] => super::t13::n3029(s, out),
		['ф', s @ ..] => super::t13::n3030(s, out),
		['х', s @ ..] => super::t13::n3031(s, out),
		_ => false,
	}
}

/// Words ending -рзить.
pub(crate) fn n711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 700);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2842);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7833);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15215);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23203);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30828);
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
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 15216);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15217);
							match s {
								[] => true,
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
					super::push(out, 7834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15218);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15219);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15220);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15221);
							match s {
								[] => true,
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
					super::push(out, 7835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15222);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15223);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узить.
pub(crate) fn n712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2843);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2844);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7836);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15224);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15225);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15226);
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
		['б', s @ ..] => {
			super::push(out, 2845);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7838);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7839);
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
					super::push(out, 7840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15227);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2846);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7841);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23204);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 23205);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23206);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2847);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7842);
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
		['р', s @ ..] => {
			super::push(out, 2848);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7843);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15229);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15230);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30829);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30830);
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 30831);
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
						['д', s @ ..] => {
							super::push(out, 15231);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23207);
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
							super::push(out, 15232);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15233);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 15234);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23208);
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
							super::push(out, 15235);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 23209);
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
									super::push(out, 23210);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 23211);
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
							super::push(out, 15236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15237);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15238);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7844);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2849);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t13::n3040(s, out),
		['ф', s @ ..] => super::t13::n3041(s, out),
		['ц', s @ ..] => super::t13::n3042(s, out),
		_ => false,
	}
}

/// Words ending -ьзить.
pub(crate) fn n713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 702);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2853);
			match s {
				[] => true,
				['о', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15248);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15249);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15250);
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

/// Words ending -юзить.
pub(crate) fn n714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 703);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2854);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2855);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7855);
					match s {
						[] => true,
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
			super::push(out, 2856);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15251);
							s.is_empty()
						}
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
		['ь', s @ ..] => {
			super::push(out, 2857);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7857);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23222);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 23223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язить.
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 704);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7858);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7860);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алить.
pub(crate) fn n717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 706);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7861);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15253);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23224);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23225);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 23226);
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
						['х', s @ ..] => {
							super::push(out, 15254);
							match s {
								[] => true,
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
					super::push(out, 7862);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15255);
							match s {
								[] => true,
								['н', 'а', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30836);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30837);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 36317);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t13::n76789(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7863);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15256);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23227);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30838);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7864);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7865);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15257);
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 23228);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30839);
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
				['б', s @ ..] => {
					super::push(out, 7866);
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
				['в', s @ ..] => {
					super::push(out, 7867);
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
					super::push(out, 7868);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15258);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23229);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30840);
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
						['р', s @ ..] => {
							super::push(out, 15259);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23230);
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
					super::push(out, 7869);
					match s {
						[] => true,
						['а', 'р', s @ ..] => super::t13::n20906(s, out),
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t13::n9540(s, out),
				['к', s @ ..] => super::t13::n9541(s, out),
				['о', s @ ..] => super::t13::n9542(s, out),
				['с', s @ ..] => {
					super::push(out, 7873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t13::n20916(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t13::n9544(s, out),
				['у', '\u{1}', s @ ..] => super::t13::n9545(s, out),
				['х', s @ ..] => super::t13::n9546(s, out),
				['ы', s @ ..] => super::t13::n9547(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t13::n3051(s, out),
		['д', s @ ..] => super::t13::n3052(s, out),
		['ж', s @ ..] => super::t13::n3053(s, out),
		['з', s @ ..] => super::t13::n3054(s, out),
		['к', s @ ..] => super::t13::n3055(s, out),
		['м', s @ ..] => super::t13::n3056(s, out),
		['н', s @ ..] => super::t13::n3057(s, out),
		['п', s @ ..] => super::t13::n3058(s, out),
		['р', s @ ..] => super::t13::n3059(s, out),
		['с', s @ ..] => super::t13::n3060(s, out),
		['т', s @ ..] => super::t13::n3061(s, out),
		['ч', s @ ..] => super::t13::n3062(s, out),
		['ш', s @ ..] => super::t13::n3063(s, out),
		_ => false,
	}
}

/// The word выскоблить.
pub(crate) fn n66127(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -углить.
pub(crate) fn n3071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2881);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7945);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7946);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15330);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23279);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23280);
							match s {
								[] => true,
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
					super::push(out, 15331);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23282);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30865);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7947);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15332);
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
			super::push(out, 7948);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15333);
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
		['м', s @ ..] => {
			super::push(out, 7949);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15334);
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
		['р', s @ ..] => {
			super::push(out, 7950);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15335);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23283);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23284);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23285);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30866);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едлить.
pub(crate) fn n3073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2883);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 7951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15336);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15337);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23286);
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
				['и', s @ ..] => {
					super::push(out, 15338);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23287);
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
					super::push(out, 15339);
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
				['у', s @ ..] => {
					super::push(out, 15340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одлить.
pub(crate) fn n3074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2884);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 7952);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15341);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7954);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -быдлить.
pub(crate) fn n9636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7955);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -елить.
pub(crate) fn n722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 711);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2886);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7956);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7957);
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
					super::push(out, 7958);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7959);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7960);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15344);
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
					super::push(out, 7961);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15345);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 15347);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23290);
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
						['п', s @ ..] => {
							super::push(out, 15348);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15349);
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
					super::push(out, 7963);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7965);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15350);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2887);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7966);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7967);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23291);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23292);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 23293);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30867);
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
									super::push(out, 23294);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 23295);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30868);
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
		['д', s @ ..] => {
			super::push(out, 2888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7968);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7969);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15351);
							match s {
								[] => true,
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
					super::push(out, 7970);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15352);
							match s {
								[] => true,
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
					super::push(out, 7971);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15353);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23296);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30869);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => super::t13::n36147(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7972);
					match s {
						[] => true,
						['а', 'р', s @ ..] => super::t13::n21060(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t13::n9654(s, out),
				['т', s @ ..] => super::t13::n9655(s, out),
				['у', s @ ..] => super::t13::n9656(s, out),
				['ы', s @ ..] => super::t13::n9657(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t13::n3079(s, out),
		['л', s @ ..] => super::t13::n3080(s, out),
		['м', s @ ..] => super::t13::n3081(s, out),
		['п', s @ ..] => super::t13::n3082(s, out),
		['р', s @ ..] => super::t13::n3083(s, out),
		['с', s @ ..] => super::t13::n3084(s, out),
		['т', s @ ..] => super::t13::n3085(s, out),
		['ц', s @ ..] => super::t13::n3086(s, out),
		['ч', s @ ..] => super::t13::n3087(s, out),
		['ш', s @ ..] => super::t13::n3088(s, out),
		['щ', s @ ..] => super::t13::n3089(s, out),
		_ => false,
	}
}

/// Words ending -злить.
pub(crate) fn n723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 712);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2900);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2901);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8019);
					match s {
						[] => true,
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
			super::push(out, 2902);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2903);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8021);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15400);
							match s {
								[] => true,
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
					super::push(out, 8022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8023);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15402);
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
					super::push(out, 8024);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15403);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15404);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23320);
									match s {
										[] => true,
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
							super::push(out, 15405);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15406);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15407);
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
					super::push(out, 8025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илить.
pub(crate) fn n724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 713);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2904);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8026);
					match s {
						[] => true,
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
			super::push(out, 2905);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8027);
					match s {
						[] => true,
						['д', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2906);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8028);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15408);
							match s {
								[] => true,
								['ф', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2907);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8029);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8030);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15409);
							match s {
								[] => true,
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
					super::push(out, 8031);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15410);
							match s {
								[] => true,
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
					super::push(out, 8032);
					match s {
						[] => true,
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
			super::push(out, 2908);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8033);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15411);
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
				['у', s @ ..] => {
					super::push(out, 8034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2909);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8035);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8036);
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
					super::push(out, 8037);
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
					super::push(out, 8038);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8039);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8040);
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
						['р', s @ ..] => {
							super::push(out, 15412);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23321);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 23322);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30878);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36329);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 36330);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76802(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8041);
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
					super::push(out, 8042);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8043);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 8044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15413);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15414);
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
						['и', s @ ..] => {
							super::push(out, 15415);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'е', 'ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
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
				['ы', s @ ..] => {
					super::push(out, 8045);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15416);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2910);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8046);
					match s {
						[] => true,
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
			super::push(out, 2911);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8047);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8048);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15417);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 15418);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30879);
											s.is_empty()
										}
										['о', s @ ..] => super::t13::n52231(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t13::n9734(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t13::n3102(s, out),
		['х', s @ ..] => super::t13::n3103(s, out),
		['ч', s @ ..] => super::t13::n3104(s, out),
		_ => false,
	}
}

/// Words ending -йлить.
pub(crate) fn n725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2915);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15419);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15420);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23323);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8056);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15421);
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
				['х', s @ ..] => {
					super::push(out, 8057);
					match s {
						[] => true,
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
			super::push(out, 2916);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8058);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15422);
							match s {
								[] => true,
								['п', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8059);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15423);
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
				['ф', s @ ..] => {
					super::push(out, 8060);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15424);
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

/// Words ending -клить.
pub(crate) fn n726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2917);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15425);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15426);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15427);
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
			super::push(out, 2918);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8062);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23324);
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
		['и', s @ ..] => {
			super::push(out, 2919);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 8063);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15428);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15429);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23325);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллить.
pub(crate) fn n727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 716);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2920);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 8064);
					match s {
						[] => true,
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
			super::push(out, 2921);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8066);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15430);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23326);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23327);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23328);
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
			super::push(out, 2922);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15431);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15432);
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

/// Words ending -млить.
pub(crate) fn n728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 717);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8068);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8069);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 8070);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15433);
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
		['я', s @ ..] => {
			super::push(out, 2924);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8071);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15434);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15435);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23329);
									match s {
										[] => true,
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
							super::push(out, 15436);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23330);
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

/// Words ending -олить.
pub(crate) fn n729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 718);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2925);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8072);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15437);
							match s {
								[] => true,
								['а', 'б', '\u{1}', s @ ..] => {
									super::push(out, 23331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8073);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8074);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15438);
							match s {
								[] => true,
								['з', 'и', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8075);
					match s {
						[] => true,
						['у', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23332);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2926);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8076);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 8077);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15439);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23333);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23334);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 23335);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23336);
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
							super::push(out, 15440);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23337);
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
				['з', s @ ..] => {
					super::push(out, 8078);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15441);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 15442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23338);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23339);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30881);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 30882);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 15443);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15444);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8079);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 15445);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23341);
									match s {
										[] => true,
										['л', 'б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 36331);
													match s {
														[] => true,
														['с', s @ ..] => super::t13::n76803(s, out),
														_ => false,
													}
												}
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
							super::push(out, 15446);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 23342);
									match s {
										[] => true,
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
							super::push(out, 15447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15448);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 23343);
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
				['р', s @ ..] => {
					super::push(out, 8080);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15449);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30883);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30884);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36332);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76804(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t13::n3117(s, out),
		['д', s @ ..] => super::t13::n3118(s, out),
		['з', s @ ..] => super::t13::n3119(s, out),
		['к', s @ ..] => super::t13::n3120(s, out),
		['м', s @ ..] => super::t13::n3121(s, out),
		['п', s @ ..] => super::t13::n3122(s, out),
		['р', s @ ..] => super::t13::n3123(s, out),
		['с', s @ ..] => super::t13::n3124(s, out),
		['т', s @ ..] => super::t13::n3125(s, out),
		['ф', s @ ..] => super::t13::n3126(s, out),
		['х', s @ ..] => super::t13::n3127(s, out),
		_ => false,
	}
}

/// Words ending -плить.
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 719);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2938);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8129);
					match s {
						[] => true,
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
			super::push(out, 2939);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15497);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15498);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23365);
									match s {
										[] => true,
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
							super::push(out, 15499);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15500);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2940);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8131);
					match s {
						[] => true,
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
			super::push(out, 2941);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 15502);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							super::push(out, 15503);
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

/// Words ending -рлить.
pub(crate) fn n731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 720);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2942);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8133);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23366);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23367);
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
								['д', 'а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23368);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 30893);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36333);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76805(s, out),
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
									super::push(out, 23369);
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
									super::push(out, 23370);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30894);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2943);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15505);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15506);
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

/// Words ending -слить.
pub(crate) fn n732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 721);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2944);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2945);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15507);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15508);
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
						['ы', s @ ..] => {
							super::push(out, 15509);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23371);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2946);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8137);
					match s {
						[] => true,
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
			super::push(out, 2947);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15510);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15511);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23372);
									match s {
										[] => true,
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
							super::push(out, 15512);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23373);
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
							super::push(out, 15513);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23374);
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
							super::push(out, 15514);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23375);
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
				['ч', s @ ..] => {
					super::push(out, 8139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15515);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15516);
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
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15517);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 15518);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30895);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 36334);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76806(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
							super::push(out, 15519);
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
							super::push(out, 15520);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15521);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23376);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2948);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8140);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8141);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15522);
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
		['у', s @ ..] => super::t13::n3139(s, out),
		['ы', s @ ..] => super::t13::n3140(s, out),
		_ => false,
	}
}

/// Words ending -тлить.
pub(crate) fn n733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 722);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2951);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2952);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 8144);
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
		['е', s @ ..] => {
			super::push(out, 2953);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8145);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23392);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23393);
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
									super::push(out, 23394);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30903);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2955);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8148);
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
			super::push(out, 2956);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8149);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8150);
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

/// Words ending -улить.
pub(crate) fn n734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 723);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2957);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8151);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23395);
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23396);
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
		['г', s @ ..] => {
			super::push(out, 2959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8152);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8153);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15539);
							match s {
								[] => true,
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
					super::push(out, 8154);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15540);
							match s {
								[] => true,
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
					super::push(out, 8155);
					match s {
						[] => true,
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
			super::push(out, 2961);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8156);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15541);
							match s {
								[] => true,
								['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8157);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15542);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15543);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23397);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 23398);
									match s {
										[] => true,
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
							super::push(out, 15544);
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
		['н', s @ ..] => {
			super::push(out, 2962);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2963);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8159);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 15545);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2964);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8160);
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
		['р', s @ ..] => {
			super::push(out, 2965);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8161);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8162);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15546);
							match s {
								[] => true,
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
					super::push(out, 8163);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15547);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23399);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8164);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15548);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23400);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8165);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15549);
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
					super::push(out, 8166);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15550);
							match s {
								[] => true,
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
					super::push(out, 8167);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15551);
							match s {
								[] => true,
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
					super::push(out, 8168);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2966);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8169);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8170);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15553);
							match s {
								[] => true,
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
					super::push(out, 8171);
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
			super::push(out, 2967);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8172);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 8173);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15554);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 23401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t13::n9867(s, out),
				['у', s @ ..] => super::t13::n9868(s, out),
				['ы', s @ ..] => super::t13::n9869(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t13::n3158(s, out),
		['ш', s @ ..] => super::t13::n3159(s, out),
		_ => false,
	}
}

/// Words ending -флить.
pub(crate) fn n735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 724);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2970);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15559);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15560);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23404);
									match s {
										[] => true,
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
							super::push(out, 15561);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23405);
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
		['и', s @ ..] => {
			super::push(out, 2971);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8182);
					match s {
						[] => true,
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
			super::push(out, 2972);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15562);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15563);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23406);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлить.
pub(crate) fn n736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 725);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2973);
			match s {
				[] => true,
				['р', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15564);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15565);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23407);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 23408);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2974);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15566);
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
			super::push(out, 2975);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 8184);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15567);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15568);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 23409);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2976);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8185);
					match s {
						[] => true,
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
			super::push(out, 2977);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15569);
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
								['и', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлить.
pub(crate) fn n737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 726);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2978);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8187);
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
			super::push(out, 2979);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15570);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылить.
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 727);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2981);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8189);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8190);
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
				['з', s @ ..] => {
					super::push(out, 8191);
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
				['о', s @ ..] => {
					super::push(out, 8192);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8193);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8194);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8195);
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
					super::push(out, 8196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
					super::push(out, 8197);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15572);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23410);
									match s {
										[] => true,
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
							super::push(out, 15573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15574);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23411);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15575);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15576);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23412);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 23413);
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23414);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30904);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2984);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8198);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8200);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15578);
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
				['у', s @ ..] => {
					super::push(out, 8201);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15579);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юлить.
pub(crate) fn n739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 728);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2985);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2986);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2987);
			match s {
				[] => true,
				['е', 'д', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2988);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 2989);
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ялить.
pub(crate) fn n740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 729);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2990);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8202);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8203);
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
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8204);
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
				['ы', s @ ..] => {
					super::push(out, 8205);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8206);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8207);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8208);
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
					super::push(out, 8209);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15581);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рорекламить.
pub(crate) fn n66172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36337);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word подинамить.
pub(crate) fn n66173(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -продинамить.
pub(crate) fn n66174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36338);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -срамить.
pub(crate) fn n9937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8219);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15589);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15590);
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

/// Words ending -самить.
pub(crate) fn n3189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2999);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8220);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15591);
					match s {
						[] => true,
						['э', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хамить.
pub(crate) fn n3190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3000);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8221);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8222);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15592);
					match s {
						[] => true,
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
			super::push(out, 8223);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15593);
					match s {
						[] => true,
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
			super::push(out, 8224);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емить.
pub(crate) fn n742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 731);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3001);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15594);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15595);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3002);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8225);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8226);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8227);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15596);
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
					super::push(out, 8228);
					match s {
						[] => true,
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
					super::push(out, 8229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имить.
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 732);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 3003);
			match s {
				[] => true,
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3004);
			match s {
				[] => true,
				['к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3005);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15597);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23422);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ймить.
pub(crate) fn n744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 733);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3006);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15598);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15599);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3007);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8231);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23423);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23424);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30909);
											match s {
												[] => true,
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
									super::push(out, 23425);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30910);
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
									super::push(out, 23426);
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
									super::push(out, 23427);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30911);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лмить.
pub(crate) fn n745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 734);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3008);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
						['с', s @ ..] => {
							super::push(out, 15601);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23428);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омить.
pub(crate) fn n746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 735);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3009);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8234);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15602);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23429);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 23430);
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
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8235);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15603);
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
					super::push(out, 8236);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15604);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23431);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23432);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30912);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8237);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15605);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23433);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 23434);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30913);
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
									super::push(out, 23435);
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
						['н', s @ ..] => {
							super::push(out, 15606);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23436);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30914);
											s.is_empty()
										}
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30915);
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
										['о', s @ ..] => {
											super::push(out, 30916);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 36339);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76811(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8238);
					match s {
						[] => true,
						['с', 'а', 'н', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3011);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8239);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8240);
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
					super::push(out, 8241);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8243);
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
					super::push(out, 8244);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15607);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23437);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 23438);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30917);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 15608);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8245);
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
					super::push(out, 8246);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 15609);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t13::n36482(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t13::n21478(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t13::n9973(s, out),
				['т', s @ ..] => super::t13::n9974(s, out),
				['у', '\u{1}', s @ ..] => super::t13::n9975(s, out),
				['ы', s @ ..] => super::t13::n9976(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t13::n3202(s, out),
		['р', s @ ..] => super::t13::n3203(s, out),
		['т', s @ ..] => super::t13::n3204(s, out),
		_ => false,
	}
}

/// Words ending -рмить.
pub(crate) fn n747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 736);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3015);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 8259);
					match s {
						[] => true,
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
			super::push(out, 3016);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15621);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15622);
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
							super::push(out, 15623);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23447);
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
							super::push(out, 15624);
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
							super::push(out, 15625);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23448);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8261);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15626);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 23449);
									match s {
										[] => true,
										['й', 'е', 'р', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 15627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23450);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23451);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30927);
											match s {
												[] => true,
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
									super::push(out, 23452);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30928);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8262);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15628);
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
						['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3017);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15629);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15630);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23453);
									match s {
										[] => true,
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
							super::push(out, 15631);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тмить.
pub(crate) fn n748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 737);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3018);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3019);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умить.
pub(crate) fn n749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 738);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 3020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8265);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8266);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23455);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30929);
											match s {
												[] => true,
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
									super::push(out, 23456);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23457);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 30930);
											match s {
												[] => true,
												['а', 'л', 'б', 'о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8267);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15633);
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
					super::push(out, 8268);
					match s {
						[] => true,
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
			super::push(out, 3021);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8269);
					match s {
						[] => true,
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
			super::push(out, 3022);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8270);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хмить.
pub(crate) fn n750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 739);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3023);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15634);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23458);
									match s {
										[] => true,
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
							super::push(out, 15635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ымить.
pub(crate) fn n751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8271);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8272);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 15636);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 15637);
							match s {
								[] => true,
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
					super::push(out, 8273);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15638);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15639);
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
					super::push(out, 8274);
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
			super::push(out, 3025);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8275);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23459);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 23460);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30931);
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
		_ => false,
	}
}

/// Words ending -ьмить.
pub(crate) fn n752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 741);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 3026);
			match s {
				[] => true,
				['у', 'к', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3027);
			match s {
				[] => true,
				['у', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15640);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15641);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 15642);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15643);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15644);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23461);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3028);
			match s {
				[] => true,
				['е', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15645);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15646);
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

/// Words ending -юмить.
pub(crate) fn n753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3029);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8276);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8277);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15647);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямить.
pub(crate) fn n754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 3030);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15648);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23462);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23463);
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
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15650);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23464);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30932);
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
						['ы', s @ ..] => {
							super::push(out, 15651);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23465);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3031);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анить.
pub(crate) fn n755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 744);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8279);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8280);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15652);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'а', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23466);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30933);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30934);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36341);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76813(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 36342);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76814(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30935);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 36343);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76815(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 36344);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76816(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 36345);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t13::n76817(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30936);
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30937);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30938);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30939);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36346);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76818(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30940);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23467);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23468);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 23469);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 23470);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23471);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30941);
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
									super::push(out, 23472);
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
						['ч', s @ ..] => {
							super::push(out, 15653);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8281);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 8282);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15654);
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
				['з', s @ ..] => {
					super::push(out, 8283);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8284);
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
				['р', s @ ..] => super::t13::n10019(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t13::n3223(s, out),
		['г', s @ ..] => super::t13::n3224(s, out),
		['д', s @ ..] => {
			super::push(out, 3035);
			match s {
				[] => true,
				['й', s @ ..] => super::t13::n10030(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t13::n3226(s, out),
		['и', s @ ..] => super::t13::n3227(s, out),
		['к', s @ ..] => super::t13::n3228(s, out),
		['л', s @ ..] => super::t13::n3229(s, out),
		['м', s @ ..] => super::t13::n3230(s, out),
		['п', s @ ..] => super::t13::n3231(s, out),
		['р', s @ ..] => super::t13::n3232(s, out),
		['с', s @ ..] => super::t13::n3233(s, out),
		['т', s @ ..] => super::t13::n3234(s, out),
		['ф', s @ ..] => super::t13::n3235(s, out),
		['щ', s @ ..] => super::t13::n3236(s, out),
		_ => false,
	}
}

/// Words ending -сгнить.
pub(crate) fn n3248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3058);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8355);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15720);
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

/// Words ending -отгнить.
pub(crate) fn n10096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -угнить.
pub(crate) fn n3250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3060);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 8357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15721);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15722);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23525);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгнить.
pub(crate) fn n3251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3061);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягнить.
pub(crate) fn n3252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3062);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8359);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -днить.
pub(crate) fn n759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 748);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3063);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15723);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3064);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8361);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 8362);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15725);
							match s {
								[] => true,
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15726);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8363);
					match s {
						[] => true,
						['с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'а', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8364);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8365);
					match s {
						[] => true,
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
			super::push(out, 3065);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8366);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
				['м', s @ ..] => {
					super::push(out, 8367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15728);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15729);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3066);
			match s {
				[] => true,
				['р', 'т', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енить.
pub(crate) fn n760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 749);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3067);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8368);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15731);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15732);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23526);
									match s {
										[] => true,
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
							super::push(out, 15733);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 23527);
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
						['о', s @ ..] => {
							super::push(out, 15734);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 23528);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30960);
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
						['щ', s @ ..] => {
							super::push(out, 15735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23529);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23530);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 23531);
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
						['ъ', s @ ..] => {
							super::push(out, 15736);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23532);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8369);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15737);
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
					super::push(out, 8370);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15738);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23533);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23534);
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
						['л', s @ ..] => {
							super::push(out, 15739);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3068);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8371);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15740);
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
				['о', s @ ..] => {
					super::push(out, 8372);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23535);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23536);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15741);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3069);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8373);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23537);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23538);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 23539);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23540);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3070);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8374);
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 8375);
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
					super::push(out, 8376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 15743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8377);
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
		['з', s @ ..] => {
			super::push(out, 3071);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8378);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15744);
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
			super::push(out, 3072);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t13::n3264(s, out),
		['м', s @ ..] => super::t13::n3265(s, out),
		['п', s @ ..] => super::t13::n3266(s, out),
		['р', s @ ..] => super::t13::n3267(s, out),
		['с', s @ ..] => super::t13::n3268(s, out),
		['т', s @ ..] => super::t13::n3269(s, out),
		['ф', s @ ..] => {
			super::push(out, 3079);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t13::n10157(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t13::n3271(s, out),
		['ч', s @ ..] => super::t13::n3272(s, out),
		['щ', s @ ..] => super::t13::n3273(s, out),
		_ => false,
	}
}

/// Words ending -жнить.
pub(crate) fn n761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 750);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3083);
			match s {
				[] => true,
				['л', 'в', 'у', s @ ..] => {
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
				['р', 'п', 'с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3084);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8420);
					match s {
						[] => true,
						['с', 'у', s @ ..] => {
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
					super::push(out, 8421);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23571);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23572);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30974);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знить.
pub(crate) fn n762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 751);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3085);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15785);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15786);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15787);
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
				['л', s @ ..] => {
					super::push(out, 8423);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15789);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23573);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23574);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30975);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36354);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 23575);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30976);
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
									super::push(out, 23576);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30977);
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
									super::push(out, 23577);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30978);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
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
								['о', s @ ..] => {
									super::push(out, 23578);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30979);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30980);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36355);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76823(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3086);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8425);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15791);
							match s {
								[] => true,
								['ж', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 8426);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15792);
							match s {
								[] => true,
								['е', 'с', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3087);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 8427);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15793);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3088);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15794);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 15795);
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
		['я', s @ ..] => {
			super::push(out, 3089);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8429);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23579);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23580);
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
								['ы', s @ ..] => {
									super::push(out, 23581);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30981);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инить.
pub(crate) fn n763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 752);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8430);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8431);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15796);
							match s {
								[] => true,
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
					super::push(out, 8432);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15797);
							match s {
								[] => true,
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
					super::push(out, 8433);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 15798);
							match s {
								[] => true,
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
					super::push(out, 8434);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15799);
							match s {
								[] => true,
								['о', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['п', s @ ..] => {
							super::push(out, 15800);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 3091);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15801);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15802);
							match s {
								[] => true,
								['с', s @ ..] => {
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ъ', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3092);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8436);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23583);
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
											super::push(out, 30982);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30983);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 30984);
											match s {
												[] => true,
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 30985);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30986);
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
		['з', s @ ..] => {
			super::push(out, 3093);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8437);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23584);
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23585);
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
				['н', 'е', 'б', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t13::n3285(s, out),
		['о', s @ ..] => super::t13::n3286(s, out),
		['р', s @ ..] => super::t13::n3287(s, out),
		['с', s @ ..] => super::t13::n3288(s, out),
		['т', s @ ..] => super::t13::n3289(s, out),
		['у', s @ ..] => super::t13::n3290(s, out),
		['ф', s @ ..] => super::t13::n3291(s, out),
		['ч', s @ ..] => super::t13::n3292(s, out),
		['ш', s @ ..] => super::t13::n3293(s, out),
		['щ', s @ ..] => super::t13::n3294(s, out),
		_ => false,
	}
}

/// Words ending -йнить.
pub(crate) fn n764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 753);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3104);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8467);
					match s {
						[] => true,
						['и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23605);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 23606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15831);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15832);
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
		['е', s @ ..] => {
			super::push(out, 3105);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8469);
					match s {
						[] => true,
						['п', 'с', 'н', 'е', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3106);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8470);
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
		_ => false,
	}
}

/// Words ending -лнить.
pub(crate) fn n765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 754);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3107);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15834);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15835);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23607);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 23608);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30993);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 36359);
													match s {
														[] => true,
														['е', 'п', '\u{1}', s @ ..] => super::t13::n76827(s, out),
														_ => false,
													}
												}
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
							super::push(out, 15836);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15837);
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
							super::push(out, 15838);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 23609);
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
								['о', s @ ..] => {
									super::push(out, 23610);
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
						['ы', s @ ..] => {
							super::push(out, 15839);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23611);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мнить.
pub(crate) fn n766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 755);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3108);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3109);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15840);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15841);
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
							super::push(out, 15842);
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
							super::push(out, 15843);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23612);
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
		['з', s @ ..] => {
			super::push(out, 3110);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8473);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15844);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3111);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8474);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15846);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15847);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23613);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 23614);
									match s {
										[] => true,
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
							super::push(out, 15848);
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
						['с', s @ ..] => {
							super::push(out, 15849);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 15850);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ннить.
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 756);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3112);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онить.
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 757);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3113);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8477);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15851);
							match s {
								[] => true,
								['а', 'б', '\u{1}', s @ ..] => {
									super::push(out, 23615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8478);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15852);
							match s {
								[] => true,
								['с', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3114);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15853);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15854);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23616);
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
						['б', s @ ..] => {
							super::push(out, 15855);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 23617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 15856);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23618);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30994);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36360);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76828(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30995);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36361);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 36362);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t13::n76829(s, out),
														['н', '\u{1}', s @ ..] => super::t13::n76830(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 36363);
													match s {
														[] => true,
														['а', s @ ..] => super::t13::n76831(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 36364);
													match s {
														[] => true,
														['о', s @ ..] => super::t13::n76832(s, out),
														_ => false,
													}
												}
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
							super::push(out, 15857);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23619);
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
							super::push(out, 15858);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23620);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 23621);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 23622);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30996);
											match s {
												[] => true,
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
									super::push(out, 23623);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 23624);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30997);
											match s {
												[] => true,
												['у', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 15859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 15860);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23625);
									match s {
										[] => true,
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
							super::push(out, 15861);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23626);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3115);
			match s {
				[] => true,
				['з', s @ ..] => super::t13::n10234(s, out),
				['л', s @ ..] => super::t13::n10235(s, out),
				['у', s @ ..] => super::t13::n10236(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t13::n3307(s, out),
		['з', s @ ..] => super::t13::n3308(s, out),
		['и', s @ ..] => super::t13::n3309(s, out),
		['к', s @ ..] => super::t13::n3310(s, out),
		['л', s @ ..] => super::t13::n3311(s, out),
		['м', s @ ..] => super::t13::n3312(s, out),
		['н', s @ ..] => super::t13::n3313(s, out),
		['п', s @ ..] => super::t13::n3314(s, out),
		['р', s @ ..] => super::t13::n3315(s, out),
		['с', s @ ..] => super::t13::n3316(s, out),
		['т', s @ ..] => super::t13::n3317(s, out),
		['ф', s @ ..] => super::t13::n3318(s, out),
		['ь', s @ ..] => super::t13::n3319(s, out),
		_ => false,
	}
}

/// Words ending -пнить.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 758);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3129);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15912);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3130);
			match s {
				[] => true,
				['р', 'к', 'у', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -рнить.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 759);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3131);
			match s {
				[] => true,
				['в', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15914);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15915);
							match s {
								[] => true,
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15916);
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15917);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8520);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15918);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 23680);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15919);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15920);
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
							super::push(out, 15921);
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
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15922);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23681);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снить.
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 760);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3132);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8522);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3133);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8523);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23682);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23683);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31012);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 15925);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23684);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8524);
					match s {
						[] => true,
						['п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15926);
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
						['о', s @ ..] => {
							super::push(out, 15927);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15928);
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
						['т', s @ ..] => {
							super::push(out, 15929);
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
							super::push(out, 15930);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23685);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31013);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36369);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76835(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3134);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15931);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15932);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23686);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3135);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8527);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 15933);
							match s {
								[] => true,
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
					super::push(out, 8528);
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
		['я', s @ ..] => {
			super::push(out, 3136);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8529);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8530);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 8531);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15934);
							s.is_empty()
						}
						['з', 'а', s @ ..] => {
							super::push(out, 15935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8532);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнить.
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 761);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3137);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15937);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15938);
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
		['у', s @ ..] => {
			super::push(out, 3138);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
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
		['я', s @ ..] => {
			super::push(out, 3139);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унить.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 762);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3140);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8534);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
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
		['л', s @ ..] => {
			super::push(out, 3141);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8535);
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
		['м', s @ ..] => {
			super::push(out, 3142);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8536);
					match s {
						[] => true,
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
			super::push(out, 3143);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8537);
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23687);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23688);
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
		['р', s @ ..] => {
			super::push(out, 3144);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15939);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 15940);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23689);
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
							super::push(out, 15941);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23690);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 23691);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31014);
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
									super::push(out, 23692);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31015);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3145);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8538);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15943);
							match s {
								[] => true,
								['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8539);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15944);
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
		_ => false,
	}
}

/// Words ending -чнить.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 763);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3146);
			match s {
				[] => true,
				['т', 'у', s @ ..] => {
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
		['у', 'т', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шнить.
pub(crate) fn n775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 764);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3147);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8540);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3148);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8541);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15946);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15947);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23693);
									match s {
										[] => true,
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
							super::push(out, 15948);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23694);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31016);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 15949);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15950);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23695);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3149);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынить.
pub(crate) fn n776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 765);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3150);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8544);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23696);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23697);
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
		['т', s @ ..] => {
			super::push(out, 3151);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8545);
					match s {
						[] => true,
						['у', 'п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнить.
pub(crate) fn n777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 766);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3152);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15951);
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
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
		['н', s @ ..] => {
			super::push(out, 3153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8547);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8548);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15952);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3154);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янить.
pub(crate) fn n778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 767);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3155);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8550);
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
		['в', s @ ..] => {
			super::push(out, 3156);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8551);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23698);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23699);
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
			super::push(out, 3157);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8552);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23700);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23701);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 8553);
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
		['и', s @ ..] => {
			super::push(out, 3158);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8554);
					match s {
						[] => true,
						['у', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23702);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23703);
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
		['м', s @ ..] => {
			super::push(out, 3159);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8555);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23704);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23705);
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
									super::push(out, 23706);
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
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23707);
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
									super::push(out, 23708);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31017);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3160);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8556);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23709);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3161);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8557);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15953);
							match s {
								[] => true,
								['т', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3162);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15954);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15955);
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
							super::push(out, 15956);
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
		['ь', s @ ..] => {
			super::push(out, 3163);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8559);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15957);
							match s {
								[] => true,
								['б', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31018);
											s.is_empty()
										}
										['е', s @ ..] => super::t13::n52673(s, out),
										['с', s @ ..] => super::t13::n52674(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t13::n10324(s, out),
				['п', s @ ..] => super::t13::n10325(s, out),
				['т', s @ ..] => super::t13::n10326(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -беспокоить.
pub(crate) fn n37022(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31021);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 36370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word упокоить.
pub(crate) fn n22031(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лоить.
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 773);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 3179);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8574);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8575);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8576);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15971);
							match s {
								[] => true,
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
					super::push(out, 8577);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15972);
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
					super::push(out, 8578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8579);
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
			super::push(out, 3180);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8580);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8581);
					match s {
						[] => true,
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
				['о', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ноить.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 774);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 3181);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8582);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8583);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 15974);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8584);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 15975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8585);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8586);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15976);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15977);
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
					super::push(out, 8587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поить.
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 775);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3182);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3183);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3184);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3185);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3186);
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
			super::push(out, 3187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8590);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 8591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8593);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15981);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3188);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3190);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роить.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 776);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3191);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3192);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8595);
					match s {
						[] => true,
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
			super::push(out, 3193);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8596);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15982);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8597);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8598);
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
					super::push(out, 8599);
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
					super::push(out, 8600);
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
					super::push(out, 8601);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8602);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15983);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3196);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8604);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15985);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15986);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 23718);
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
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 23719);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31023);
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
							super::push(out, 15987);
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
							super::push(out, 15988);
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
								['с', '\u{1}', s @ ..] => {
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
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15989);
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
						['ы', s @ ..] => {
							super::push(out, 15990);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23720);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8607);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тоить.
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8608);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8609);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word скосолапить.
pub(crate) fn n66293(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -епить.
pub(crate) fn n795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 784);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8623);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8624);
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
					super::push(out, 8625);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8626);
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
				['з', s @ ..] => {
					super::push(out, 8627);
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
					super::push(out, 8628);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8629);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16003);
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
							super::push(out, 16004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23721);
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
					super::push(out, 8630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16005);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16006);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16007);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 16008);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 8631);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16009);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3212);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8632);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16010);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23722);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31026);
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
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16011);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16012);
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
						['и', s @ ..] => {
							super::push(out, 16013);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 16014);
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
							super::push(out, 16015);
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
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8634);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8635);
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
				['ы', s @ ..] => {
					super::push(out, 8636);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16016);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3214);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8637);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8638);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t13::n10452(s, out),
				['с', s @ ..] => super::t13::n10453(s, out),
				['т', s @ ..] => super::t13::n10454(s, out),
				['ы', s @ ..] => super::t13::n10455(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хайпить.
pub(crate) fn n10460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8646);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16022);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16023);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23723);
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
			super::push(out, 16024);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чайпить.
pub(crate) fn n10461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8647);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16025);
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

/// Words ending -ейпить.
pub(crate) fn n3410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3218);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8648);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лпить.
pub(crate) fn n798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 787);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 3219);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8649);
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

/// Words ending -мпить.
pub(crate) fn n799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 788);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3220);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16027);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16028);
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

