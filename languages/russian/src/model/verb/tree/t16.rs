use flexion::Path;

/// Words ending -латься.
pub(crate) fn n4662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4222);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10680);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18423);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32350);
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
		['е', s @ ..] => {
			super::push(out, 10681);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25791);
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
							super::push(out, 25792);
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
							super::push(out, 25793);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32351);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25794);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 25795);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25796);
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
			super::push(out, 10682);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 18426);
					match s {
						[] => true,
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
										_ => false,
									}
								}
								['с', 'и', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10683);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18427);
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
			super::push(out, 10684);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18428);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25797);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10685);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18429);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25798);
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
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18430);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25799);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32352);
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
				['о', s @ ..] => super::t21::n26152(s, out),
				['ы', s @ ..] => {
					super::push(out, 18432);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25800);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t21::n13284(s, out),
		['ы', s @ ..] => super::t21::n13285(s, out),
		_ => false,
	}
}

/// Words ending -маться.
pub(crate) fn n4663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4223);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10689);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18436);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18437);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25807);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32359);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32360);
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
						['т', s @ ..] => {
							super::push(out, 25808);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32361);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37143);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 37144);
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
		['и', s @ ..] => {
			super::push(out, 10690);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 18438);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
								['п', '\u{1}', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 18439);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25809);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32362);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37145);
											s.is_empty()
										}
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
											super::push(out, 37146);
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
								['н', s @ ..] => {
									super::push(out, 32363);
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
						['б', s @ ..] => {
							super::push(out, 25810);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => super::t21::n70037(s, out),
										['о', s @ ..] => super::t21::n70038(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => super::t21::n41537(s, out),
						['д', s @ ..] => super::t21::n41538(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t21::n41539(s, out),
						['з', 'а', 'р', '\u{1}', s @ ..] => super::t21::n41540(s, out),
						['и', 'р', 'п', s @ ..] => super::t21::n41541(s, out),
						['о', s @ ..] => super::t21::n41542(s, out),
						['с', s @ ..] => super::t21::n41543(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t21::n41544(s, out),
						['у', s @ ..] => super::t21::n41545(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t21::n41546(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t21::n13288(s, out),
		['о', s @ ..] => super::t21::n13289(s, out),
		['у', s @ ..] => super::t21::n13290(s, out),
		['ы', s @ ..] => super::t21::n13291(s, out),
		_ => false,
	}
}

/// Words ending -наться.
pub(crate) fn n4664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4224);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10695);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18444);
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
		['г', s @ ..] => {
			super::push(out, 10696);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18445);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18446);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18447);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18448);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25823);
							match s {
								[] => true,
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
					super::push(out, 18449);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18450);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25824);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32372);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32373);
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
							super::push(out, 25825);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 32374);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 32375);
									match s {
										[] => true,
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
							super::push(out, 25826);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25827);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32377);
									match s {
										[] => true,
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
							super::push(out, 25828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25829);
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
				['у', s @ ..] => {
					super::push(out, 18451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18452);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25830);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10697);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18453);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32378);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32379);
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
		['з', s @ ..] => {
			super::push(out, 10698);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18454);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18455);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18456);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25831);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32380);
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
						['р', 'п', '\u{1}', s @ ..] => {
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
				['у', s @ ..] => {
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
					super::push(out, 18457);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t21::n13296(s, out),
		['о', s @ ..] => {
			super::push(out, 10700);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25836);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25837);
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
				['т', s @ ..] => super::t21::n26194(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t21::n13298(s, out),
		['т', s @ ..] => super::t21::n13299(s, out),
		['у', s @ ..] => super::t21::n13300(s, out),
		_ => false,
	}
}

/// Words ending -паться.
pub(crate) fn n4665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4225);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10704);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25841);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25842);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32383);
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
				['л', s @ ..] => {
					super::push(out, 18465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18466);
					match s {
						[] => true,
						['а', 'ц', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32384);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32385);
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
									super::push(out, 32386);
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
								['с', s @ ..] => {
									super::push(out, 32387);
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
									super::push(out, 32388);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37155);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25843);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25844);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['ч', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10705);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18467);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25845);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32389);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t21::n70084(s, out),
										['о', s @ ..] => super::t21::n70085(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => super::t21::n41648(s, out),
						['ш', s @ ..] => super::t21::n41649(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t21::n26205(s, out),
				['щ', s @ ..] => super::t21::n26206(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t21::n13303(s, out),
		['о', s @ ..] => super::t21::n13304(s, out),
		['р', s @ ..] => super::t21::n13305(s, out),
		['с', s @ ..] => super::t21::n13306(s, out),
		['у', s @ ..] => super::t21::n13307(s, out),
		['ы', s @ ..] => super::t21::n13308(s, out),
		['ю', s @ ..] => super::t21::n13309(s, out),
		['я', s @ ..] => super::t21::n13310(s, out),
		_ => false,
	}
}

/// Words ending -раться.
pub(crate) fn n4666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4226);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10714);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 32442);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25899);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 25900);
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
							super::push(out, 25901);
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
						['ы', s @ ..] => {
							super::push(out, 25902);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32443);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18498);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32444);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32445);
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
			super::push(out, 10715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18499);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18500);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32446);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32447);
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
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32448);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 32449);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
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
				['е', s @ ..] => {
					super::push(out, 18501);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25903);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 32450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18502);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25904);
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
				['и', s @ ..] => {
					super::push(out, 18503);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25905);
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
				['о', s @ ..] => super::t21::n26241(s, out),
				['у', s @ ..] => {
					super::push(out, 18505);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25914);
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['ы', s @ ..] => {
					super::push(out, 18506);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25915);
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
		['в', s @ ..] => super::t21::n13313(s, out),
		['г', s @ ..] => super::t21::n13314(s, out),
		['д', s @ ..] => super::t21::n13315(s, out),
		['ж', s @ ..] => super::t21::n13316(s, out),
		['и', s @ ..] => super::t21::n13317(s, out),
		['й', s @ ..] => {
			super::push(out, 10721);
			match s {
				[] => true,
				['а', 'х', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t21::n13319(s, out),
		['с', s @ ..] => super::t21::n13320(s, out),
		['у', s @ ..] => {
			super::push(out, 10724);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 18546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25956);
							s.is_empty()
						}
						['а', s @ ..] => super::t21::n41887(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t21::n41888(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -саться.
pub(crate) fn n4667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4227);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10725);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18547);
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
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18549);
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
										['д', 'о', 'п', '\u{1}', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25959);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32489);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 37205);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
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
			super::push(out, 10726);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 18550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25960);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25961);
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
							super::push(out, 25962);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32490);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25963);
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
							super::push(out, 25964);
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
							super::push(out, 25965);
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
							super::push(out, 25966);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32491);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10727);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18552);
					match s {
						[] => true,
						['с', s @ ..] => super::t21::n41915(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t21::n26294(s, out),
				['н', s @ ..] => super::t21::n26295(s, out),
				['п', s @ ..] => super::t21::n26296(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t21::n13325(s, out),
		['о', s @ ..] => super::t21::n13326(s, out),
		['с', s @ ..] => super::t21::n13327(s, out),
		['у', s @ ..] => super::t21::n13328(s, out),
		['я', s @ ..] => super::t21::n13329(s, out),
		_ => false,
	}
}

/// Words ending -таться.
pub(crate) fn n4668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4228);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10733);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18567);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25994);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32512);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32513);
									match s {
										[] => true,
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
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 25995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32514);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32515);
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
								['о', 'п', '\u{1}', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 18568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25996);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25997);
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
							super::push(out, 25998);
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
							super::push(out, 25999);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32516);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26000);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26001);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 26002);
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
				['р', s @ ..] => {
					super::push(out, 18570);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26003);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32517);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['ч', s @ ..] => super::t21::n26312(s, out),
				['ш', s @ ..] => super::t21::n26313(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t21::n13331(s, out),
		['и', s @ ..] => super::t21::n13332(s, out),
		['л', s @ ..] => super::t21::n13333(s, out),
		['о', s @ ..] => super::t21::n13334(s, out),
		['п', s @ ..] => super::t21::n13335(s, out),
		['р', s @ ..] => {
			super::push(out, 10739);
			match s {
				[] => true,
				['е', s @ ..] => super::t21::n26337(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t21::n13337(s, out),
		['у', s @ ..] => super::t21::n13338(s, out),
		['х', s @ ..] => super::t21::n13339(s, out),
		['ч', s @ ..] => super::t21::n13340(s, out),
		['ш', s @ ..] => super::t21::n13341(s, out),
		['ы', s @ ..] => super::t21::n13342(s, out),
		['я', s @ ..] => super::t21::n13343(s, out),
		_ => false,
	}
}

/// Words ending -хаться.
pub(crate) fn n4669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4229);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10747);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18614);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18615);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26105);
							match s {
								[] => true,
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
					super::push(out, 18616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 18617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18618);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26106);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 26107);
							match s {
								[] => true,
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
					super::push(out, 18619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26108);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26109);
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
							super::push(out, 26110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32596);
									match s {
										[] => true,
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
							super::push(out, 26111);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 26112);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32597);
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
							super::push(out, 26113);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32598);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 32599);
									match s {
										[] => true,
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
							super::push(out, 26114);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26115);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26116);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26117);
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
							super::push(out, 26118);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26119);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32601);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 32602);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 37275);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 26120);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 32603);
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
						['т', s @ ..] => {
							super::push(out, 26121);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26123);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32604);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18621);
					match s {
						[] => true,
						['а', s @ ..] => super::t21::n42133(s, out),
						['т', s @ ..] => super::t21::n42134(s, out),
						['у', s @ ..] => super::t21::n42135(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t21::n13345(s, out),
		['и', s @ ..] => super::t21::n13346(s, out),
		['о', s @ ..] => super::t21::n13347(s, out),
		['п', s @ ..] => super::t21::n13348(s, out),
		['р', s @ ..] => super::t21::n13349(s, out),
		['у', s @ ..] => super::t21::n13350(s, out),
		['ы', s @ ..] => super::t21::n13351(s, out),
		['ю', s @ ..] => super::t21::n13352(s, out),
		['я', s @ ..] => super::t21::n13353(s, out),
		_ => false,
	}
}

/// Words ending -цаться.
pub(crate) fn n4670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4230);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10757);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18651);
					match s {
						[] => true,
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
			super::push(out, 10758);
			match s {
				[] => true,
				['л', 'к', 'с', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
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
			super::push(out, 10759);
			match s {
				[] => true,
				['е', 'з', 'о', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10760);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18652);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18653);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26174);
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

/// Words ending -чаться.
pub(crate) fn n4671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4231);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10761);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26176);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26177);
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
							super::push(out, 26178);
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
							super::push(out, 26179);
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
							super::push(out, 26180);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32632);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18656);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26181);
							match s {
								[] => true,
								['о', s @ ..] => {
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
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26182);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26183);
							match s {
								[] => true,
								['а', 'н', s @ ..] => {
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
										_ => false,
									}
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
				['р', s @ ..] => {
					super::push(out, 18658);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26184);
							match s {
								[] => true,
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
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26185);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10762);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18660);
					match s {
						[] => true,
						['л', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10763);
			match s {
				[] => true,
				['е', 'л', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 18661);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
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
		['е', s @ ..] => {
			super::push(out, 10764);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ц', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 18662);
					match s {
						[] => true,
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
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => super::t21::n57369(s, out),
								_ => false,
							}
						}
						['ы', 'в', '\u{1}', s @ ..] => super::t21::n42241(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t21::n26415(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t21::n13362(s, out),
		['л', s @ ..] => super::t21::n13363(s, out),
		['м', s @ ..] => super::t21::n13364(s, out),
		['н', s @ ..] => super::t21::n13365(s, out),
		['о', s @ ..] => super::t21::n13366(s, out),
		['р', s @ ..] => super::t21::n13367(s, out),
		['у', s @ ..] => super::t21::n13368(s, out),
		['ы', s @ ..] => super::t21::n13369(s, out),
		['ь', s @ ..] => super::t21::n13370(s, out),
		['ю', s @ ..] => super::t21::n13371(s, out),
		['я', s @ ..] => super::t21::n13372(s, out),
		_ => false,
	}
}

/// Words ending -шаться.
pub(crate) fn n4672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4232);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10776);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
				['л', 'г', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
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
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', 'у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
						['т', 'с', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10777);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10778);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26220);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26221);
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
							super::push(out, 26222);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32666);
									match s {
										[] => true,
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 26223);
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
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 18692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26224);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26225);
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
							super::push(out, 26226);
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
							super::push(out, 26227);
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
							super::push(out, 26228);
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
							super::push(out, 26229);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26230);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32667);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18693);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'с', 'о', 'в', '\u{1}', s @ ..] => {
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
				['т', s @ ..] => {
					super::push(out, 18694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10779);
			match s {
				[] => true,
				['т', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10780);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 18695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26231);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ъ', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10781);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18696);
					match s {
						[] => true,
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t21::n26459(s, out),
				['т', 'с', 'у', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t21::n13379(s, out),
		['у', s @ ..] => super::t21::n13380(s, out),
		['ч', s @ ..] => {
			super::push(out, 10784);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t21::n13382(s, out),
		['ь', s @ ..] => super::t21::n13383(s, out),
		_ => false,
	}
}

/// Words ending -щаться.
pub(crate) fn n4673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4233);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10787);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
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
						_ => false,
					}
				}
				['м', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'с', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
								['о', 'д', '\u{1}', s @ ..] => {
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
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', s @ ..] => {
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
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
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
										['о', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 32684);
									match s {
										[] => true,
										['л', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 37310);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 40381);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 26252);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 37311);
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
											super::push(out, 37312);
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10788);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26253);
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
						['и', s @ ..] => {
							super::push(out, 26254);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32685);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 32686);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 37313);
											match s {
												[] => true,
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
									super::push(out, 32687);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t21::n13386(s, out),
		['и', s @ ..] => super::t21::n13387(s, out),
		['м', s @ ..] => {
			match s {
				[] => true,
				['о', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t21::n13389(s, out),
		['р', s @ ..] => {
			super::push(out, 10792);
			match s {
				[] => true,
				['и', 'м', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t21::n13391(s, out),
		['ы', s @ ..] => super::t21::n13392(s, out),
		['ь', s @ ..] => super::t21::n13393(s, out),
		['я', s @ ..] => super::t21::n13394(s, out),
		_ => false,
	}
}

/// Words ending -адеться.
pub(crate) fn n13399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10800);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 18731);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18732);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26267);
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
			super::push(out, 18733);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ддеться.
pub(crate) fn n13401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10802);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18734);
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

/// Words ending -здеться.
pub(crate) fn n13402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10803);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18735);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26268);
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
		['б', s @ ..] => {
			super::push(out, 18736);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26269);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32703);
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
			super::push(out, 18737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18738);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 26270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32704);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32705);
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
							super::push(out, 32706);
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
							super::push(out, 32707);
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
						['ы', s @ ..] => {
							super::push(out, 32708);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37323);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18739);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идеться.
pub(crate) fn n13403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10804);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18740);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26272);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32709);
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
							super::push(out, 32710);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37324);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26273);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26274);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 32712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 37325);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 40385);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 26275);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32713);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 37326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 26276);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 26277);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 26278);
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
					super::push(out, 26279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 26280);
					match s {
						[] => true,
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
			super::push(out, 18742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26281);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 26282);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26283);
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
				['т', s @ ..] => {
					super::push(out, 26284);
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
					super::push(out, 26285);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одеться.
pub(crate) fn n13404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10805);
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
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдеться.
pub(crate) fn n13405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10806);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18743);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18744);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26286);
					match s {
						[] => true,
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
			super::push(out, 18745);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 26287);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32715);
							match s {
								[] => true,
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
					super::push(out, 26288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32716);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37327);
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
			super::push(out, 18746);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 26289);
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

/// Words ending -удеться.
pub(crate) fn n13406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10807);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 18747);
			match s {
				[] => true,
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18748);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26290);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 26291);
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

/// Words ending -ядеться.
pub(crate) fn n13407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10808);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18749);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32717);
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
						['о', s @ ..] => {
							super::push(out, 32718);
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
								['о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -зеться.
pub(crate) fn n4678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4238);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10809);
			match s {
				[] => true,
				['л', 'г', 'а', s @ ..] => {
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

/// Words ending -леться.
pub(crate) fn n4679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4239);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18750);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26292);
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10811);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26293);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26294);
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
			super::push(out, 10812);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18751);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26295);
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26296);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32719);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18752);
					match s {
						[] => true,
						['о', s @ ..] => {
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
		['т', s @ ..] => {
			super::push(out, 10813);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18753);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18754);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ч', 'е', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32720);
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
									super::push(out, 32721);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37328);
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
				['е', 'в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26297);
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
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -меться.
pub(crate) fn n4680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4240);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10814);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18755);
					match s {
						[] => true,
						['г', 'з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18756);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18757);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18758);
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
			super::push(out, 10816);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18759);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26298);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10817);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18760);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26299);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37329);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 37330);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18761);
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
		['ы', s @ ..] => {
			super::push(out, 10818);
			match s {
				[] => true,
				['з', 'о', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -неться.
pub(crate) fn n4681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4241);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10819);
			match s {
				[] => true,
				['и', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26300);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26301);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 26302);
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
			super::push(out, 10820);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18762);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26303);
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
				['л', s @ ..] => {
					super::push(out, 18763);
					match s {
						[] => true,
						['е', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32722);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32723);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 32724);
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
		['и', s @ ..] => {
			super::push(out, 10821);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26304);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26305);
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
		['м', s @ ..] => {
			super::push(out, 10822);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18764);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32725);
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
		['р', s @ ..] => {
			super::push(out, 10823);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26306);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32726);
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
						['ч', s @ ..] => {
							super::push(out, 26307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		['с', s @ ..] => {
			super::push(out, 10824);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18765);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32727);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32728);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 18766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26308);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 26309);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32729);
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
		['т', s @ ..] => {
			super::push(out, 10825);
			match s {
				[] => true,
				['у', 'м', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -петься.
pub(crate) fn n4682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4242);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10826);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10827);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18769);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 26310);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
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
		['е', s @ ..] => {
			super::push(out, 10828);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18770);
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
			super::push(out, 10829);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18771);
					match s {
						[] => true,
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
				['р', s @ ..] => {
					super::push(out, 18772);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26311);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32730);
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
						['п', s @ ..] => {
							super::push(out, 26312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 26313);
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
				['с', s @ ..] => {
					super::push(out, 18773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26314);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26315);
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
				['ш', s @ ..] => {
					super::push(out, 18774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26316);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
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
			super::push(out, 10830);
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
		['р', s @ ..] => {
			super::push(out, 10831);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18775);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32731);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32732);
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
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
								['ы', s @ ..] => {
									super::push(out, 32733);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37331);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18776);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18777);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26317);
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
						['в', s @ ..] => {
							super::push(out, 26318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 26319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10833);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18778);
					match s {
						[] => true,
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
			super::push(out, 10834);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18779);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реться.
pub(crate) fn n4683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4243);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10835);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 18780);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26320);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10836);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18781);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['т', 'о', s @ ..] => {
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10837);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26321);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26322);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32734);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37332);
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 32735);
									match s {
										[] => true,
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
							super::push(out, 26323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 26324);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32736);
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
							super::push(out, 26325);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32737);
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
						['о', s @ ..] => super::t21::n42579(s, out),
						['с', s @ ..] => super::t21::n42580(s, out),
						['т', s @ ..] => super::t21::n42581(s, out),
						['у', s @ ..] => super::t21::n42582(s, out),
						['ы', s @ ..] => super::t21::n42583(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t21::n26596(s, out),
				['т', s @ ..] => super::t21::n26597(s, out),
				['ч', s @ ..] => super::t21::n26598(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t21::n13437(s, out),
		['о', s @ ..] => super::t21::n13438(s, out),
		['т', s @ ..] => super::t21::n13439(s, out),
		_ => false,
	}
}

/// Words ending -сеться.
pub(crate) fn n4684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4244);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10841);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18797);
					match s {
						[] => true,
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

/// Words ending -теться.
pub(crate) fn n4685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4245);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10842);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18798);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
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
		['л', s @ ..] => {
			super::push(out, 10843);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18799);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32763);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32764);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 32765);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10844);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 18800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26352);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 32766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 26353);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26354);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26355);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26356);
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
		['р', s @ ..] => {
			super::push(out, 10845);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18801);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32767);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32768);
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
									super::push(out, 32769);
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
									super::push(out, 32770);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37345);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10846);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18802);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32771);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32772);
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32773);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37346);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10847);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18803);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32774);
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
				['я', s @ ..] => {
					super::push(out, 18804);
					match s {
						[] => true,
						['р', 'к', 'о', 'р', 'п', '\u{1}', s @ ..] => super::t21::n42625(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -биться.
pub(crate) fn n4690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4250);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10852);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10853);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26362);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26363);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26364);
							match s {
								[] => true,
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
					super::push(out, 18814);
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
					super::push(out, 18815);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26365);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37347);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 37348);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 26366);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', s @ ..] => {
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
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18816);
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
				['р', s @ ..] => {
					super::push(out, 18817);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32776);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 32777);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 32778);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 37349);
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
									super::push(out, 32779);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 32780);
									match s {
										[] => true,
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
							super::push(out, 26368);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32781);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18818);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32782);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32783);
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10854);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18819);
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
			super::push(out, 10855);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10856);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18820);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18821);
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
			super::push(out, 10857);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18822);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26369);
							match s {
								[] => true,
								['л', 'о', 'д', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 26370);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32784);
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
						['р', s @ ..] => super::t21::n42662(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t21::n26647(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t21::n13458(s, out),
		['и', s @ ..] => super::t21::n13459(s, out),
		['л', s @ ..] => super::t21::n13460(s, out),
		['м', s @ ..] => super::t21::n13461(s, out),
		['о', s @ ..] => super::t21::n13462(s, out),
		['р', s @ ..] => super::t21::n13463(s, out),
		['с', s @ ..] => super::t21::n13464(s, out),
		['т', s @ ..] => super::t21::n13465(s, out),
		['у', s @ ..] => super::t22::n13466(s, out),
		['ы', s @ ..] => super::t22::n13467(s, out),
		['ю', s @ ..] => super::t22::n13468(s, out),
		['я', s @ ..] => super::t22::n13469(s, out),
		_ => false,
	}
}

/// Words ending -виться.
pub(crate) fn n4691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4251);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10870);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10871);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26434);
							s.is_empty()
						}
						['а', s @ ..] => super::t22::n42749(s, out),
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => super::t22::n42751(s, out),
						['и', 'р', 'п', s @ ..] => super::t22::n42752(s, out),
						['о', s @ ..] => super::t22::n42753(s, out),
						['с', s @ ..] => super::t22::n42754(s, out),
						['у', s @ ..] => super::t22::n42755(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t22::n26684(s, out),
				['д', s @ ..] => super::t22::n26685(s, out),
				['з', s @ ..] => {
					super::push(out, 18858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t22::n42767(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t22::n26687(s, out),
				['л', s @ ..] => super::t22::n26688(s, out),
				['н', s @ ..] => super::t22::n26689(s, out),
				['р', s @ ..] => super::t22::n26690(s, out),
				['с', s @ ..] => super::t22::n26691(s, out),
				['т', s @ ..] => super::t22::n26692(s, out),
				['ч', s @ ..] => super::t22::n26693(s, out),
				['ш', s @ ..] => super::t22::n26694(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t22::n13472(s, out),
		['в', s @ ..] => super::t22::n13473(s, out),
		['д', s @ ..] => super::t22::n13474(s, out),
		['е', s @ ..] => super::t22::n13475(s, out),
		['з', s @ ..] => super::t22::n13476(s, out),
		['и', s @ ..] => super::t22::n13477(s, out),
		['л', s @ ..] => super::t22::n13478(s, out),
		['о', s @ ..] => super::t22::n13479(s, out),
		['р', s @ ..] => super::t22::n13480(s, out),
		['с', s @ ..] => super::t22::n13481(s, out),
		['т', s @ ..] => super::t22::n13482(s, out),
		['у', s @ ..] => super::t22::n13483(s, out),
		['ы', s @ ..] => super::t22::n13484(s, out),
		['я', s @ ..] => super::t22::n13485(s, out),
		_ => false,
	}
}

/// Words ending -диться.
pub(crate) fn n4693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4253);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10887);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18914);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26526);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32945);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 32946);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 37426);
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
						['т', s @ ..] => {
							super::push(out, 26527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 18915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26528);
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
						['з', 'и', '\u{1}', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 18916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26530);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26531);
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
										['д', 'о', 'п', '\u{1}', s @ ..] => {
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
						['г', s @ ..] => super::t22::n42885(s, out),
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 26533);
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
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t22::n42888(s, out),
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => super::t22::n42890(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t22::n42891(s, out),
						['у', '\u{1}', s @ ..] => super::t22::n42892(s, out),
						['х', s @ ..] => super::t22::n42893(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t22::n26747(s, out),
				['р', s @ ..] => super::t22::n26748(s, out),
				['с', s @ ..] => super::t22::n26749(s, out),
				['т', s @ ..] => super::t22::n26750(s, out),
				['ш', s @ ..] => super::t22::n26751(s, out),
				['щ', s @ ..] => super::t22::n26752(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t22::n13488(s, out),
		['е', s @ ..] => super::t22::n13489(s, out),
		['ж', s @ ..] => super::t22::n13490(s, out),
		['з', s @ ..] => super::t22::n13491(s, out),
		['и', s @ ..] => super::t22::n13492(s, out),
		['й', s @ ..] => super::t22::n13493(s, out),
		['н', s @ ..] => super::t22::n13494(s, out),
		['о', s @ ..] => super::t22::n13495(s, out),
		['р', s @ ..] => super::t22::n13496(s, out),
		['у', s @ ..] => super::t22::n13497(s, out),
		['ы', s @ ..] => super::t22::n13498(s, out),
		['ю', s @ ..] => super::t22::n13499(s, out),
		['я', s @ ..] => super::t22::n13500(s, out),
		_ => false,
	}
}

/// Words ending -житься.
pub(crate) fn n4695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4255);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10905);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10906);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18969);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26676);
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 18970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18971);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26677);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33072);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 37510);
											match s {
												[] => true,
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
									super::push(out, 33073);
									match s {
										[] => true,
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
							super::push(out, 26678);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 33074);
									match s {
										[] => true,
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
							super::push(out, 26679);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 33075);
									match s {
										[] => true,
										['а', 'т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 18972);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 26680);
							match s {
								[] => true,
								['л', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 26681);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33076);
									match s {
										[] => true,
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
							super::push(out, 26682);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26683);
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
					super::push(out, 18974);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26684);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26685);
							match s {
								[] => true,
								['д', 'у', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37511);
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26686);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37512);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 37513);
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
										['с', s @ ..] => {
											super::push(out, 37514);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10907);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18975);
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
			super::push(out, 10908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t22::n13509(s, out),
		['з', s @ ..] => {
			super::push(out, 10910);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18986);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33084);
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
					super::push(out, 18987);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26700);
							match s {
								[] => true,
								['б', s @ ..] => super::t22::n58399(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18988);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t22::n26819(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t22::n13511(s, out),
		['л', s @ ..] => super::t22::n13512(s, out),
		['н', s @ ..] => super::t22::n13513(s, out),
		['о', s @ ..] => super::t22::n13514(s, out),
		['с', s @ ..] => super::t22::n13515(s, out),
		['т', s @ ..] => super::t22::n13516(s, out),
		['у', s @ ..] => super::t22::n13517(s, out),
		['ы', s @ ..] => super::t22::n13518(s, out),
		['ю', s @ ..] => super::t22::n13519(s, out),
		['я', s @ ..] => super::t22::n13520(s, out),
		_ => false,
	}
}

/// Words ending -зиться.
pub(crate) fn n4696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4256);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10921);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19020);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26749);
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
				['к', s @ ..] => {
					super::push(out, 19021);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19022);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26750);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 26751);
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
					super::push(out, 19023);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26752);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 26753);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 37556);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 40459);
													match s {
														[] => true,
														['б', 'о', '\u{1}', s @ ..] => super::t22::n85290(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26754);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 37557);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 37558);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 40460);
													match s {
														[] => true,
														['б', s @ ..] => super::t22::n85291(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 40461);
													s.is_empty()
												}
												['ы', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 37559);
											match s {
												[] => true,
												['в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 40462);
													match s {
														[] => true,
														['з', s @ ..] => super::t22::n85292(s, out),
														_ => false,
													}
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
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 26755);
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
						['о', s @ ..] => {
							super::push(out, 26756);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26758);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 26759);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33134);
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
				['у', s @ ..] => {
					super::push(out, 19024);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 26760);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33135);
									s.is_empty()
								}
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
			super::push(out, 10922);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19025);
					match s {
						[] => true,
						['г', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10923);
			match s {
				[] => true,
				['л', s @ ..] => super::t22::n26858(s, out),
				['р', s @ ..] => super::t22::n26859(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t22::n13524(s, out),
		['м', s @ ..] => super::t22::n13525(s, out),
		['н', s @ ..] => {
			super::push(out, 10926);
			match s {
				[] => true,
				['о', s @ ..] => super::t22::n26865(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t22::n13527(s, out),
		['р', s @ ..] => super::t22::n13528(s, out),
		['у', s @ ..] => super::t22::n13529(s, out),
		['ю', s @ ..] => super::t22::n13530(s, out),
		['я', s @ ..] => super::t22::n13531(s, out),
		_ => false,
	}
}

/// Words ending -литься.
pub(crate) fn n4697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4257);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10932);
			s.is_empty()
		}
		['а', s @ ..] => super::t22::n13533(s, out),
		['б', s @ ..] => {
			super::push(out, 10934);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26869);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 26870);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37607);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 37608);
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
										['т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 37609);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 40477);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t22::n85305(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10936);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19071);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26871);
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
				['у', s @ ..] => {
					super::push(out, 19072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26872);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 26873);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33214);
									match s {
										[] => true,
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
							super::push(out, 26874);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 26875);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33215);
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
							super::push(out, 26876);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 33216);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37610);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 37611);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 37612);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 37613);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 37614);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 40478);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t22::n85306(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19073);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26877);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33217);
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
				['я', s @ ..] => {
					super::push(out, 19074);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26878);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19075);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 19076);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26879);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33218);
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
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 33219);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t22::n26909(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t22::n13538(s, out),
		['з', s @ ..] => super::t22::n13539(s, out),
		['и', s @ ..] => super::t22::n13540(s, out),
		['к', s @ ..] => super::t22::n13541(s, out),
		['м', s @ ..] => super::t22::n13542(s, out),
		['о', s @ ..] => super::t22::n13543(s, out),
		['п', s @ ..] => super::t22::n13544(s, out),
		['р', s @ ..] => super::t22::n13545(s, out),
		['с', s @ ..] => super::t22::n13546(s, out),
		['т', s @ ..] => super::t22::n13547(s, out),
		['у', s @ ..] => super::t22::n13548(s, out),
		['х', s @ ..] => super::t22::n13549(s, out),
		['ш', s @ ..] => super::t22::n13550(s, out),
		['ы', s @ ..] => super::t22::n13551(s, out),
		['э', s @ ..] => super::t22::n13552(s, out),
		['ю', s @ ..] => super::t22::n13553(s, out),
		['я', s @ ..] => super::t22::n13554(s, out),
		_ => false,
	}
}

/// Words ending -миться.
pub(crate) fn n4698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4258);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10955);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19161);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27060);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33380);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19162);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33381);
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
				['р', s @ ..] => {
					super::push(out, 19163);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 27061);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33382);
									match s {
										[] => true,
										['т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33383);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33384);
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
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 19164);
					match s {
						[] => true,
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10956);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19165);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27063);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27064);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33385);
									match s {
										[] => true,
										['ы', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19166);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27065);
							match s {
								[] => true,
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
					super::push(out, 19167);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33386);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 33387);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19168);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27066);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
		['и', s @ ..] => {
			super::push(out, 10957);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19169);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27067);
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
					super::push(out, 19170);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27068);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33388);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33389);
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
		['й', s @ ..] => {
			super::push(out, 10958);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27069);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27070);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27071);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27072);
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
		['л', s @ ..] => {
			super::push(out, 10959);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19171);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33390);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33391);
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
		['м', s @ ..] => {
			super::push(out, 10960);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19172);
					match s {
						[] => true,
						['р', 'г', 'о', 'т', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t22::n13561(s, out),
		['р', s @ ..] => super::t22::n13562(s, out),
		['т', s @ ..] => {
			super::push(out, 10963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19180);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19181);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t22::n13564(s, out),
		['ы', s @ ..] => {
			super::push(out, 10965);
			match s {
				[] => true,
				['д', s @ ..] => super::t22::n27022(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t22::n13566(s, out),
		['ю', s @ ..] => super::t22::n13567(s, out),
		['я', s @ ..] => super::t22::n13568(s, out),
		_ => false,
	}
}

/// Words ending -ниться.
pub(crate) fn n4699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4259);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10969);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27108);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27109);
							match s {
								[] => true,
								['р', 'а', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 40499);
													s.is_empty()
												}
												['а', s @ ..] => super::t22::n80433(s, out),
												['о', s @ ..] => super::t22::n80434(s, out),
												['т', s @ ..] => super::t22::n80435(s, out),
												_ => false,
											}
										}
										['т', s @ ..] => super::t22::n71782(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t22::n43599(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t22::n27027(s, out),
				['г', s @ ..] => super::t22::n27028(s, out),
				['з', s @ ..] => super::t22::n27029(s, out),
				['и', s @ ..] => super::t22::n27030(s, out),
				['к', s @ ..] => super::t22::n27031(s, out),
				['л', s @ ..] => super::t22::n27032(s, out),
				['м', s @ ..] => super::t22::n27033(s, out),
				['р', s @ ..] => super::t22::n27034(s, out),
				['с', s @ ..] => super::t22::n27035(s, out),
				['т', s @ ..] => super::t22::n27036(s, out),
				['ф', s @ ..] => super::t22::n27037(s, out),
				['щ', s @ ..] => super::t22::n27038(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t23::n13570(s, out),
		['в', s @ ..] => super::t23::n13571(s, out),
		['г', s @ ..] => super::t23::n13572(s, out),
		['д', s @ ..] => super::t23::n13573(s, out),
		['е', s @ ..] => super::t23::n13574(s, out),
		['ж', s @ ..] => super::t23::n13575(s, out),
		['з', s @ ..] => super::t23::n13576(s, out),
		['и', s @ ..] => super::t23::n13577(s, out),
		['л', s @ ..] => super::t23::n13578(s, out),
		['м', s @ ..] => super::t23::n13579(s, out),
		['о', s @ ..] => super::t23::n13580(s, out),
		['п', s @ ..] => super::t23::n13581(s, out),
		['р', s @ ..] => super::t23::n13582(s, out),
		['с', s @ ..] => super::t23::n13583(s, out),
		['т', s @ ..] => super::t23::n13584(s, out),
		['у', s @ ..] => super::t23::n13585(s, out),
		['ч', s @ ..] => super::t23::n13586(s, out),
		['ш', s @ ..] => super::t23::n13587(s, out),
		['ы', s @ ..] => super::t23::n13588(s, out),
		['ю', s @ ..] => super::t23::n13589(s, out),
		['я', s @ ..] => super::t23::n13590(s, out),
		_ => false,
	}
}

/// Words ending -оиться.
pub(crate) fn n4700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4260);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10991);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27305);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27306);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33608);
									match s {
										[] => true,
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
							super::push(out, 27307);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33609);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 33610);
									match s {
										[] => true,
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
							super::push(out, 27308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 27309);
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
					super::push(out, 19293);
					match s {
						[] => true,
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 19294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19295);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19296);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19297);
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
					super::push(out, 19298);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27311);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 27312);
							match s {
								[] => true,
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
					super::push(out, 19299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19300);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19301);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10993);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19302);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33611);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['е', 'б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 40549);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 40550);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t23::n85383(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 40551);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t23::n85384(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 37839);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['д', 'о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													match s {
														[] => true,
														['м', 'а', 'с', '\u{1}', s @ ..] => super::t23::n85385(s, out),
														['п', '\u{1}', s @ ..] => super::t23::n85386(s, out),
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10994);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27314);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33612);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27315);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27316);
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
							super::push(out, 27317);
							match s {
								[] => true,
								['р', s @ ..] => super::t23::n59389(s, out),
								_ => false,
							}
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => super::t23::n43847(s, out),
						['с', 'а', 'р', '\u{1}', s @ ..] => super::t23::n43848(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t23::n43849(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t23::n13595(s, out),
		['п', s @ ..] => super::t23::n13596(s, out),
		['р', s @ ..] => super::t23::n13597(s, out),
		['т', s @ ..] => super::t23::n13598(s, out),
		_ => false,
	}
}

/// Words ending -питься.
pub(crate) fn n4701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4261);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10999);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11000);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19321);
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
				['н', s @ ..] => {
					super::push(out, 19322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19323);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27335);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33623);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 33624);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11001);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19324);
					match s {
						[] => true,
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
			super::push(out, 11002);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11003);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27336);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27337);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27338);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33625);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33626);
									match s {
										[] => true,
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
							super::push(out, 27340);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33627);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19327);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27341);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33628);
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
							super::push(out, 27342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33629);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33630);
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
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
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
				['ц', s @ ..] => {
					super::push(out, 19328);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27343);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33631);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => super::t23::n27168(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t23::n13604(s, out),
		['й', s @ ..] => super::t23::n13605(s, out),
		['л', s @ ..] => super::t23::n13606(s, out),
		['о', s @ ..] => super::t23::n13607(s, out),
		['с', s @ ..] => super::t23::n13608(s, out),
		['т', s @ ..] => {
			super::push(out, 11009);
			match s {
				[] => true,
				['о', s @ ..] => super::t23::n27184(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t23::n13610(s, out),
		['ы', s @ ..] => super::t23::n13611(s, out),
		['я', s @ ..] => super::t23::n13612(s, out),
		_ => false,
	}
}

/// Words ending -риться.
pub(crate) fn n4702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4262);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11013);
			match s {
				[] => true,
				['в', s @ ..] => super::t23::n27194(s, out),
				['г', s @ ..] => {
					super::push(out, 19356);
					match s {
						[] => true,
						['е', 'ч', 'о', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t23::n27196(s, out),
				['ж', s @ ..] => super::t23::n27197(s, out),
				['з', s @ ..] => {
					super::push(out, 19359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27420);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27421);
							match s {
								[] => true,
								['б', 'з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => super::t23::n44008(s, out),
						['в', '\u{1}', s @ ..] => super::t23::n44009(s, out),
						['з', s @ ..] => super::t23::n44010(s, out),
						['о', s @ ..] => super::t23::n44011(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t23::n27199(s, out),
				['м', s @ ..] => super::t23::n27200(s, out),
				['п', s @ ..] => super::t23::n27201(s, out),
				['т', s @ ..] => super::t23::n27202(s, out),
				['у', s @ ..] => super::t23::n27203(s, out),
				['х', s @ ..] => super::t23::n27204(s, out),
				['ц', s @ ..] => super::t23::n27205(s, out),
				['ш', s @ ..] => super::t23::n27206(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t23::n13614(s, out),
		['в', s @ ..] => super::t23::n13615(s, out),
		['г', s @ ..] => super::t23::n13616(s, out),
		['д', s @ ..] => super::t23::n13617(s, out),
		['е', s @ ..] => super::t23::n13618(s, out),
		['з', s @ ..] => super::t23::n13619(s, out),
		['и', s @ ..] => super::t23::n13620(s, out),
		['к', s @ ..] => super::t23::n13621(s, out),
		['о', s @ ..] => super::t23::n13622(s, out),
		['т', s @ ..] => super::t23::n13623(s, out),
		['у', s @ ..] => super::t23::n13624(s, out),
		['ф', s @ ..] => super::t23::n13625(s, out),
		['х', s @ ..] => super::t23::n13626(s, out),
		['щ', s @ ..] => super::t23::n13627(s, out),
		['ы', s @ ..] => super::t23::n13628(s, out),
		['ю', s @ ..] => super::t23::n13629(s, out),
		['я', s @ ..] => super::t23::n13630(s, out),
		_ => false,
	}
}

/// Words ending -ситься.
pub(crate) fn n4703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11031);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27625);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27626);
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
					super::push(out, 19457);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33846);
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
									super::push(out, 33847);
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
								['ы', s @ ..] => {
									super::push(out, 33848);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 37965);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27628);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19459);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27629);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33849);
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
				['л', s @ ..] => {
					super::push(out, 19460);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27630);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33850);
									match s {
										[] => true,
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
							super::push(out, 27631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33851);
									s.is_empty()
								}
								['з', s @ ..] => {
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
												['о', 'р', 'п', '\u{1}', s @ ..] => {
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
									super::push(out, 33852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						_ => false,
					}
				}
				['п', s @ ..] => super::t23::n27303(s, out),
				['р', s @ ..] => super::t23::n27304(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t23::n13632(s, out),
		['к', s @ ..] => super::t23::n13633(s, out),
		['о', s @ ..] => super::t23::n13634(s, out),
		['р', s @ ..] => super::t23::n13635(s, out),
		['у', s @ ..] => super::t23::n13636(s, out),
		['ы', s @ ..] => super::t23::n13637(s, out),
		['э', s @ ..] => super::t23::n13638(s, out),
		_ => false,
	}
}

/// Words ending -титься.
pub(crate) fn n4704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4264);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11039);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19482);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27690);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 37997);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 37998);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 37999);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 38000);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33916);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33917);
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
				['в', s @ ..] => super::t23::n27325(s, out),
				['г', s @ ..] => super::t23::n27326(s, out),
				['к', s @ ..] => super::t23::n27327(s, out),
				['л', s @ ..] => super::t23::n27328(s, out),
				['м', s @ ..] => super::t23::n27329(s, out),
				['н', s @ ..] => super::t23::n27330(s, out),
				['п', s @ ..] => super::t23::n27331(s, out),
				['р', s @ ..] => super::t23::n27332(s, out),
				['х', s @ ..] => super::t23::n27333(s, out),
				['ч', s @ ..] => super::t23::n27334(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t23::n13640(s, out),
		['е', s @ ..] => super::t23::n13641(s, out),
		['и', s @ ..] => super::t23::n13642(s, out),
		['й', s @ ..] => super::t23::n13643(s, out),
		['л', s @ ..] => super::t23::n13644(s, out),
		['н', s @ ..] => super::t23::n13645(s, out),
		['о', s @ ..] => super::t23::n13646(s, out),
		['п', s @ ..] => super::t23::n13647(s, out),
		['р', s @ ..] => super::t23::n13648(s, out),
		['с', s @ ..] => super::t23::n13649(s, out),
		['у', s @ ..] => super::t23::n13650(s, out),
		['ч', s @ ..] => super::t23::n13651(s, out),
		['ы', s @ ..] => super::t23::n13652(s, out),
		['э', s @ ..] => super::t23::n13653(s, out),
		['ю', s @ ..] => super::t23::n13654(s, out),
		['я', s @ ..] => super::t23::n13655(s, out),
		_ => false,
	}
}

/// Words ending -уиться.
pub(crate) fn n4705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4265);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11056);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27835);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 34090);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фиться.
pub(crate) fn n4706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4266);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11057);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19555);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27836);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34091);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 34092);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 38132);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 27837);
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
		['и', s @ ..] => {
			super::push(out, 11058);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19556);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34093);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 34094);
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
					super::push(out, 19557);
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
		['л', s @ ..] => {
			super::push(out, 11059);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19558);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34095);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34096);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 38133);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -читься.
pub(crate) fn n4707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4267);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11060);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19559);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 27838);
							match s {
								[] => true,
								['а', 'б', s @ ..] => {
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
						['о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34097);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
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
						['ы', 'д', s @ ..] => {
							super::push(out, 27839);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 34098);
									match s {
										[] => true,
										['о', 'ф', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19560);
					match s {
						[] => true,
						['л', 'у', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34099);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 19561);
					match s {
						[] => true,
						['а', 'з', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19562);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27840);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 34100);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', s @ ..] => {
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
						['в', s @ ..] => {
							super::push(out, 27841);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34101);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 27842);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 34102);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19563);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34103);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34104);
									match s {
										[] => true,
										['н', s @ ..] => {
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
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 34105);
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
								['ы', s @ ..] => {
									super::push(out, 34106);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 38134);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t24::n27407(s, out),
				['с', s @ ..] => {
					super::push(out, 19565);
					match s {
						[] => true,
						['ы', s @ ..] => super::t24::n44541(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t24::n27409(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t24::n13661(s, out),
		['г', s @ ..] => super::t24::n13662(s, out),
		['е', s @ ..] => super::t24::n13663(s, out),
		['и', s @ ..] => super::t24::n13664(s, out),
		['л', s @ ..] => super::t24::n13665(s, out),
		['н', s @ ..] => super::t24::n13666(s, out),
		['о', s @ ..] => super::t24::n13667(s, out),
		['р', s @ ..] => super::t24::n13668(s, out),
		['т', s @ ..] => super::t24::n13669(s, out),
		['у', s @ ..] => super::t24::n13670(s, out),
		['ы', s @ ..] => super::t24::n13671(s, out),
		['ь', s @ ..] => super::t24::n13672(s, out),
		['ю', s @ ..] => super::t24::n13673(s, out),
		['я', s @ ..] => super::t24::n13674(s, out),
		_ => false,
	}
}

