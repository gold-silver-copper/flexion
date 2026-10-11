use flexion::Path;

/// Words ending -линг.
pub(crate) fn n7574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7552);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25671);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25672);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 25673);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 52864);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 81279);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 52865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 25674);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 25675);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 52866);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81280);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104248);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 120046);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129893);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 52867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81282);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104249);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120047);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 81283);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 52868);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81284);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104251);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120048);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25676);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 52869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 25677);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 25678);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 52871);
					match s {
						[] => true,
						['а', 'с', 'и', s @ ..] => {
							super::push(out, 81285);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104252);
									s.is_empty()
								}
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
			super::push(out, 25679);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52872);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81286);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104253);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 120049);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129894);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t45::n137103(s, out),
														_ => false,
													}
												}
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
							super::push(out, 81287);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 81288);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 104255);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120050);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129895);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t45::n137104(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52873);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 52874);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104256);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 104257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 52875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 25680);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25681);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52876);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81289);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 104258);
									s.is_empty()
								}
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
			super::push(out, 25682);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52877);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 52878);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81291);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104259);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120051);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25683);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 52879);
					match s {
						[] => true,
						['е', 'р', 'м', s @ ..] => {
							super::push(out, 81292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25684);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52880);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 81293);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 81294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 52881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -минг.
pub(crate) fn n7575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7553);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25685);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52882);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 81295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 25686);
			match s {
				[] => true,
				['е', 'л', s @ ..] => {
					super::push(out, 52883);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 81296);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 104260);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 120052);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129896);
													match s {
														[] => true,
														['к', s @ ..] => super::t45::n137105(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25687);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 52884);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104261);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 52885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25688);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 25689);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52886);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81298);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нинг.
pub(crate) fn n7576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7554);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25690);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 25691);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 52887);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81299);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 81300);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 104263);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 120053);
											s.is_empty()
										}
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
					super::push(out, 52888);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 52889);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81302);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104264);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 120054);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25692);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 25693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 25694);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 52891);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 52892);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81303);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104265);
									s.is_empty()
								}
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
			super::push(out, 25695);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25696);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 52893);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25697);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52894);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81305);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104266);
									s.is_empty()
								}
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
			super::push(out, 25698);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оинг.
pub(crate) fn n7577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7555);
	s.is_empty()
}

/// Words ending -пинг.
pub(crate) fn n7578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7556);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25699);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 52895);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81306);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104267);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 120055);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 104268);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 120056);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129897);
													s.is_empty()
												}
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
							super::push(out, 81307);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104269);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 120057);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129898);
													s.is_empty()
												}
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
							super::push(out, 81308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 25700);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 25701);
			match s {
				[] => true,
				['а', 'ж', 'д', s @ ..] => {
					match s {
						[] => true,
						['-', 'т', 'р', 'е', 'д', '\u{1}', s @ ..] => {
							super::push(out, 81309);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 81310);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 104270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'д', s @ ..] => {
					super::push(out, 52896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25702);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 25703);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 25704);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52897);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ринг.
pub(crate) fn n7579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7557);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25705);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 52898);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 81311);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 104271);
									s.is_empty()
								}
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
			super::push(out, 25706);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 52899);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 52900);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81313);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 104272);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120058);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 129899);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t45::n137106(s, out),
														_ => false,
													}
												}
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
							super::push(out, 81314);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104273);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120059);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 129900);
													match s {
														[] => true,
														['р', s @ ..] => super::t45::n137107(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52901);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 52902);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 81316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 52903);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81317);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 104274);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 120060);
											s.is_empty()
										}
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
					super::push(out, 52904);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 81318);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 81319);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 52905);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 81320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 52906);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 81321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25707);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 52907);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 81322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 25708);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25709);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 52909);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 81323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 52910);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 52911);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81324);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104276);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120061);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129901);
													s.is_empty()
												}
												_ => false,
											}
										}
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
							super::push(out, 81325);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 104277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 81326);
							match s {
								[] => true,
								['а', 'ф', 'е', s @ ..] => {
									super::push(out, 104278);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 120062);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129902);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 104279);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 120063);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129903);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25710);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 52912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25711);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52913);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25712);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 52914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25713);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 52915);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81328);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104280);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120064);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25714);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 52916);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 52917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -синг.
pub(crate) fn n7580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7558);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 25715);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25716);
			match s {
				[] => true,
				['о', 'с', 'т', s @ ..] => {
					super::push(out, 52918);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 81330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинг.
pub(crate) fn n7581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7559);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25717);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52919);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 81331);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 81332);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 104281);
									s.is_empty()
								}
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
			super::push(out, 25718);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 52920);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81333);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 52921);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81334);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104283);
									match s {
										[] => true,
										['м', '-', s @ ..] => {
											super::push(out, 120065);
											s.is_empty()
										}
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
					super::push(out, 52922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81335);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104284);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120066);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25719);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 52923);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81336);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104285);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 52924);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81337);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104286);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120067);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129904);
													match s {
														[] => true,
														['к', s @ ..] => super::t45::n137108(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 25720);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52926);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 52927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 25721);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 25722);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52928);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 52929);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 81338);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 104287);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120068);
											s.is_empty()
										}
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
					super::push(out, 52930);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81339);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104288);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120069);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 129905);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25723);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 25724);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52931);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 52932);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81340);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104289);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 120070);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129906);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t45::n137109(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25725);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 25726);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81341);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 81342);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 52934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 25727);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 52935);
					match s {
						[] => true,
						['л', 'р', s @ ..] => {
							super::push(out, 81343);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 104291);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 120071);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129907);
													match s {
														[] => true,
														['п', s @ ..] => super::t45::n137110(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уинг.
pub(crate) fn n7582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7560);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 25728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52936);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -финг.
pub(crate) fn n7583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7561);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 25729);
			match s {
				[] => true,
				['е', 'с', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 81344);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104292);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120072);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 129908);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t45::n137111(s, out),
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
									super::push(out, 104293);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120073);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 129909);
													match s {
														[] => true,
														['р', s @ ..] => super::t45::n137112(s, out),
														_ => false,
													}
												}
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
							super::push(out, 81345);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104294);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120074);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 129910);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чинг.
pub(crate) fn n7584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7562);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 25730);
			match s {
				[] => true,
				['о', 'в', 'д', 'р', 'е', 'б', '\u{1}', s @ ..] => {
					super::push(out, 52937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25731);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52938);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 81346);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 104295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шинг.
pub(crate) fn n7585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7563);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25732);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52939);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 81347);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104296);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120075);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 129911);
													match s {
														[] => true,
														['ф', s @ ..] => super::t45::n137113(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25733);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52940);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 81348);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 104297);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 120076);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129912);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бандар-лог.
pub(crate) fn n131266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129914);
	s.is_empty()
}

/// Words ending -йт-каталог.
pub(crate) fn n131267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129915);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135708);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 139092);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олог.
pub(crate) fn n7626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7604);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 25786);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 25787);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 25788);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 25789);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 52985);
					match s {
						[] => true,
						['и', 'ф', 'о', s @ ..] => {
							super::push(out, 81378);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 104311);
									match s {
										[] => true,
										['и', 'с', 'п', '\u{1}', s @ ..] => {
											super::push(out, 120083);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52986);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 52987);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81379);
							match s {
								[] => true,
								['и', 'г', '-', s @ ..] => {
									super::push(out, 104312);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 120084);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129916);
													match s {
														[] => true,
														['ш', s @ ..] => super::t45::n137116(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52988);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 81380);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104313);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120085);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129917);
													s.is_empty()
												}
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
							super::push(out, 81381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 52989);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81382);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104314);
									s.is_empty()
								}
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
			super::push(out, 25791);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52990);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 81383);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'н', 'э', '\u{1}', s @ ..] => {
							super::push(out, 81384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 25792);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52991);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 52992);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81385);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104316);
									match s {
										[] => true,
										['н', 'е', 'ф', '\u{1}', s @ ..] => {
											super::push(out, 120086);
											s.is_empty()
										}
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
					super::push(out, 52993);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81386);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104317);
									match s {
										[] => true,
										['о', 'д', 'н', 'э', '-', s @ ..] => {
											super::push(out, 120087);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 129918);
													match s {
														[] => true,
														['о', s @ ..] => super::t45::n137117(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52994);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 81387);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104318);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 104319);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120088);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129919);
													match s {
														[] => true,
														['е', s @ ..] => super::t45::n137118(s, out),
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
									super::push(out, 104320);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 120089);
											match s {
												[] => true,
												['у', s @ ..] => super::t45::n131272(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => super::t45::n82236(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t45::n53532(s, out),
				['х', s @ ..] => super::t45::n53533(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t45::n25964(s, out),
		['р', s @ ..] => super::t45::n25965(s, out),
		['с', s @ ..] => super::t45::n25966(s, out),
		['т', s @ ..] => super::t45::n25967(s, out),
		['ф', s @ ..] => super::t45::n25968(s, out),
		['х', s @ ..] => super::t45::n25969(s, out),
		_ => false,
	}
}

/// Words ending -дарлог.
pub(crate) fn n53550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53013);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 81403);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104329);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 120098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерлог.
pub(crate) fn n25972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25801);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 53014);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 81404);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104330);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 120099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлог.
pub(crate) fn n7630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7608);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25802);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53015);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ца-носорог.
pub(crate) fn n131280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129928);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 135718);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 139101);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ук-носорог.
pub(crate) fn n131281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129929);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 135719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -льсингборг.
pub(crate) fn n131285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129933);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135722);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 139103);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -люксембург.
pub(crate) fn n131286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129934);
	s.is_empty()
}

/// Words ending -ранденбург.
pub(crate) fn n131287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129935);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 135723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мекленбург.
pub(crate) fn n131288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129936);
	s.is_empty()
}

/// Words ending -ханнесбург.
pub(crate) fn n131289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129937);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 135724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртбург.
pub(crate) fn n53620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53083);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 81460);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 104359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цбург.
pub(crate) fn n26044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25873);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 53084);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81461);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104360);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120112);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129938);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53085);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 81462);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 53086);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 81463);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104362);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 120113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теург.
pub(crate) fn n26045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25874);
	s.is_empty()
}

/// Words ending -иург.
pub(crate) fn n7675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7653);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 25875);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53087);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 81464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лург.
pub(crate) fn n7676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7654);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25876);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 53088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 25877);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53089);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81465);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рург.
pub(crate) fn n7677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7655);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25878);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 53090);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 81466);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104364);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 120114);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 129939);
													match s {
														[] => true,
														['р', s @ ..] => super::t45::n137132(s, out),
														_ => false,
													}
												}
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
							super::push(out, 81467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тург.
pub(crate) fn n7678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7656);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25879);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 53091);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81468);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лавираусцуг.
pub(crate) fn n137133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135726);
	s.is_empty()
}

/// Words ending -магазин-склад.
pub(crate) fn n142506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141061);
	s.is_empty()
}

/// Words ending -красноград.
pub(crate) fn n131296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129944);
	s.is_empty()
}

/// Words ending -истианстад.
pub(crate) fn n131297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129945);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 135728);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 139106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жук-короед.
pub(crate) fn n131300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129948);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 135731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ук-листоед.
pub(crate) fn n131301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129949);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 135732);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллелепипед.
pub(crate) fn n131302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129950);
	s.is_empty()
}

/// Words ending -ог-ортопед.
pub(crate) fn n131303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129951);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 135733);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139108);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -смерребред.
pub(crate) fn n131304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129952);
	s.is_empty()
}

/// Words ending -латонфюред.
pub(crate) fn n131305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129953);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -паук-муравьед.
pub(crate) fn n131306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129954);
	s.is_empty()
}

/// Words ending -цетилиодид.
pub(crate) fn n131308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129956);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135736);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ен-инвалид.
pub(crate) fn n131309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129957);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 135737);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 139111);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141064);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аплоид.
pub(crate) fn n53877(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 81615);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 81616);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 104434);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 120152);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 129958);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 135738);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 139112);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иплоид.
pub(crate) fn n53878(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', 'о', s @ ..] => {
			super::push(out, 81617);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 104435);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120153);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'о', 'п', 'о', 'з', s @ ..] => {
			super::push(out, 81618);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 104436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'т', 'о', s @ ..] => {
			super::push(out, 81619);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 104437);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 120155);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -моид.
pub(crate) fn n7930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7907);
	s.is_empty()
}

/// Words ending -ноид.
pub(crate) fn n7931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7908);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26180);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 53340);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81620);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 81621);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104438);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120156);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 104439);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120157);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81622);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 81623);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 104441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 53341);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 81624);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104442);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120158);
											s.is_empty()
										}
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
					super::push(out, 53342);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 81625);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 104443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 53343);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81627);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 81628);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104445);
									s.is_empty()
								}
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
			super::push(out, 26181);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['м', 'о', s @ ..] => {
					super::push(out, 53344);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 81629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'у', s @ ..] => {
					super::push(out, 53345);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81630);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 53346);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 81631);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 104447);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120159);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['в', s @ ..] => {
					super::push(out, 53347);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 53348);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 53349);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81633);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104448);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 120160);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129961);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поид.
pub(crate) fn n7932(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', 'р', s @ ..] => {
			super::push(out, 26182);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 53350);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81634);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104449);
									s.is_empty()
								}
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
			super::push(out, 26183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роид.
pub(crate) fn n7933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7909);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 26184);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53351);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 81635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104450);
									s.is_empty()
								}
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
			super::push(out, 26185);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53352);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53353);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 53354);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 81637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 53355);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81638);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 81639);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 104451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 53356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26187);
			match s {
				[] => true,
				['б', 'и', s @ ..] => {
					super::push(out, 53357);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81640);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 53358);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 81641);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 81642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26188);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26189);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 26190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -соид.
pub(crate) fn n7934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7910);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81643);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 104453);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120161);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 81644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 53359);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 81645);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104454);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 120162);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129962);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тоид.
pub(crate) fn n7935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7911);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26191);
			s.is_empty()
		}
		['и', 'з', s @ ..] => {
			super::push(out, 26192);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53360);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81646);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104455);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120163);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26193);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53361);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 81647);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104456);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120164);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 81648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 26194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хоид.
pub(crate) fn n7936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7912);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 26195);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 53362);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 81649);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цоид.
pub(crate) fn n7937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7913);
	s.is_empty()
}

/// Words ending -каллифорид.
pub(crate) fn n131315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129963);
	s.is_empty()
}

/// Words ending -лидерабенд.
pub(crate) fn n131318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129966);
	s.is_empty()
}

/// Words ending -юфаундленд.
pub(crate) fn n131319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129967);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 135739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -диснейленд.
pub(crate) fn n131320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129968);
	s.is_empty()
}

/// Words ending -тленд.
pub(crate) fn n26460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26282);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 53434);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 53435);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 81705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренд.
pub(crate) fn n8010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7984);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 26283);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 26284);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 26285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53436);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 53437);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 81706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 53438);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 81707);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104491);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 120185);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129969);
													match s {
														[] => true,
														['к', s @ ..] => super::t45::n137147(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 53439);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81708);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104492);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120186);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129970);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хенд.
pub(crate) fn n8012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7986);
	s.is_empty()
}

/// Words ending -акета-зонд.
pub(crate) fn n131323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129971);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 135741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вея-полевод.
pub(crate) fn n137149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135742);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 139114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 141065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ектропривод.
pub(crate) fn n137150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135743);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139115);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 141066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 142207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-групповод.
pub(crate) fn n137151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135744);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139116);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141067);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 142208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровод.
pub(crate) fn n26514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26333);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 53507);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 81757);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 104524);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120205);
									s.is_empty()
								}
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
			super::push(out, 53508);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 53509);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 81758);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 81759);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 81760);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 104525);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 120206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 104526);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 120207);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129975);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135745);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t45::n140554(s, out),
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
									super::push(out, 120208);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129976);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135746);
													match s {
														[] => true,
														['л', s @ ..] => super::t45::n140555(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53510);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 81761);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урсовод.
pub(crate) fn n82610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81762);
	match s {
		[] => true,
		['к', 'с', 'к', 'э', '-', s @ ..] => {
			super::push(out, 104528);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120209);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 129977);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 135747);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139119);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товод.
pub(crate) fn n26516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26335);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 81763);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104529);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120210);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129978);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 104530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 81764);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 53515);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховод.
pub(crate) fn n26517(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 53516);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81765);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 53517);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 81766);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 104533);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -город.
pub(crate) fn n26575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53557);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 81791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 53558);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 81792);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 104545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 81793);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 104546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 53559);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81794);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 104547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 81795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 53560);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 81796);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 81797);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 104548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 53561);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81798);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 104549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 53562);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 81799);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104550);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120217);
									s.is_empty()
								}
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
			super::push(out, 53563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 81800);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 81801);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 104551);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 120218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 104552);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 120219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 81802);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 104553);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120220);
									s.is_empty()
								}
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
			super::push(out, 53564);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 81803);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 53565);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 81804);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104555);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 120221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктрод.
pub(crate) fn n26580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26398);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 53570);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 81809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урод.
pub(crate) fn n8084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8058);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26399);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрод.
pub(crate) fn n8086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8060);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кавалергард.
pub(crate) fn n137156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135749);
	s.is_empty()
}

/// The word резерфорд.
pub(crate) fn n131335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129983);
	s.is_empty()
}

/// Words ending -талиногруд.
pub(crate) fn n131336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129984);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 135750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чнодышащее.
pub(crate) fn n131341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129989);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135752);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обезболивающее.
pub(crate) fn n131342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129990);
	s.is_empty()
}

/// Words ending -окружающее.
pub(crate) fn n131343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129991);
	s.is_empty()
}

/// Words ending -копитающее.
pub(crate) fn n131344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129992);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -означающее.
pub(crate) fn n131345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129993);
	s.is_empty()
}

/// Words ending -еутоляющее.
pub(crate) fn n131346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129994);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 135754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ящее.
pub(crate) fn n8266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8240);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 26631);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 53772);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 81956);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104634);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 120257);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26632);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 53773);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 81957);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104635);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 120258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 104636);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120259);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 129995);
													match s {
														[] => true,
														['п', s @ ..] => super::t45::n137162(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ейроразнообразие.
pub(crate) fn n131355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130003);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 135760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вященноначалие.
pub(crate) fn n131356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130004);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 135761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ание.
pub(crate) fn n8315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8289);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 26677);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53815);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 53816);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 53817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t45::n26862(s, out),
		['г', s @ ..] => {
			super::push(out, 26679);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53826);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 53827);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 82047);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 82048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 53828);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82049);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 104850);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 82050);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 104851);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120518);
											s.is_empty()
										}
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
					super::push(out, 53829);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82051);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 104852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 82052);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 53830);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 82053);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 82054);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 104854);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120519);
											s.is_empty()
										}
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
					super::push(out, 53831);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82055);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 104855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 53832);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 53833);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82056);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 104856);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120520);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 104857);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120521);
											s.is_empty()
										}
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
					super::push(out, 53834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26680);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53835);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 82057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 53836);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82058);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104858);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120522);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130208);
													s.is_empty()
												}
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
							super::push(out, 82059);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104859);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120523);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 82060);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 104860);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120524);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 130209);
													s.is_empty()
												}
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
							super::push(out, 82061);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 82062);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 82063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 53837);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 53838);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82065);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104861);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 120525);
											s.is_empty()
										}
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
					super::push(out, 53839);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 53840);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82066);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 82067);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104862);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120526);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26681);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 53841);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 53842);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 53843);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 53844);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 53845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t45::n26866(s, out),
		['к', s @ ..] => super::t45::n26867(s, out),
		['л', s @ ..] => {
			super::push(out, 26684);
			match s {
				[] => true,
				['е', s @ ..] => super::t45::n54415(s, out),
				['и', s @ ..] => {
					super::push(out, 53869);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 82129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t45::n54417(s, out),
				['о', s @ ..] => super::t45::n54418(s, out),
				['с', s @ ..] => super::t45::n54419(s, out),
				['ы', s @ ..] => super::t45::n54420(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t45::n26869(s, out),
		['н', s @ ..] => super::t45::n26870(s, out),
		['п', s @ ..] => super::t45::n26871(s, out),
		['р', s @ ..] => super::t45::n26872(s, out),
		['с', s @ ..] => super::t45::n26873(s, out),
		['т', s @ ..] => super::t45::n26874(s, out),
		['х', s @ ..] => super::t45::n26875(s, out),
		['ц', s @ ..] => super::t45::n26876(s, out),
		['ч', s @ ..] => super::t45::n26877(s, out),
		['ш', s @ ..] => super::t45::n26878(s, out),
		['щ', s @ ..] => super::t45::n26879(s, out),
		_ => false,
	}
}

/// Words ending -ение.
pub(crate) fn n8317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8291);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26697);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53931);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 104981);
									s.is_empty()
								}
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
			super::push(out, 26698);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 26699);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53932);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 82200);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104982);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120589);
											s.is_empty()
										}
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
					super::push(out, 53933);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82201);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104983);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120590);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 130237);
													match s {
														[] => true,
														['п', s @ ..] => super::t45::n137310(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 53934);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82202);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 82203);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104984);
									match s {
										[] => true,
										['т', 'о', '-', s @ ..] => {
											super::push(out, 120591);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 130238);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137311(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 53935);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82204);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104985);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 120592);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 130239);
													match s {
														[] => true,
														['е', s @ ..] => super::t45::n137312(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 53936);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 82205);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 104986);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120593);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130240);
													match s {
														[] => true,
														['ш', s @ ..] => super::t45::n137313(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t45::n26884(s, out),
		['е', s @ ..] => {
			super::push(out, 26701);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 53947);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82226);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t45::n26886(s, out),
		['з', s @ ..] => {
			super::push(out, 26703);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 26704);
			s.is_empty()
		}
		['л', s @ ..] => super::t45::n26889(s, out),
		['м', s @ ..] => {
			super::push(out, 26706);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 53970);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82266);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 105061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 53971);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82267);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 105062);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120688);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 130315);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137365(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 53972);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 53973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t45::n26891(s, out),
		['о', s @ ..] => {
			super::push(out, 26708);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 53982);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 53983);
					s.is_empty()
				}
				['р', 'т', 'с', 'о', s @ ..] => super::t45::n54532(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 26709);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 53984);
					match s {
						[] => true,
						['е', 'т', 'о', s @ ..] => {
							super::push(out, 82277);
							match s {
								[] => true,
								['г', s @ ..] => super::t45::n106199(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t45::n26894(s, out),
		['с', s @ ..] => super::t45::n26895(s, out),
		['т', s @ ..] => super::t45::n26896(s, out),
		['ч', s @ ..] => super::t45::n26897(s, out),
		['ш', s @ ..] => super::t45::n26898(s, out),
		['щ', s @ ..] => super::t45::n26899(s, out),
		_ => false,
	}
}

/// The word чародеяние.
pub(crate) fn n131720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130365);
	s.is_empty()
}

/// The word содеяние.
pub(crate) fn n122030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120759);
	s.is_empty()
}

/// Words ending -леяние.
pub(crate) fn n54588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54033);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 82356);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 82357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -меяние.
pub(crate) fn n54589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54034);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 82358);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105162);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пеяние.
pub(crate) fn n54590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54035);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 82359);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реяние.
pub(crate) fn n54591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54036);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 82360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сеяние.
pub(crate) fn n54592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54037);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 82361);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 82362);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105163);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ияние.
pub(crate) fn n26908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26724);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 54038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ляние.
pub(crate) fn n26909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26725);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 54039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82363);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105164);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -няние.
pub(crate) fn n26910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26726);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 54040);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 82364);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ояние.
pub(crate) fn n26911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26727);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 54041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ряние.
pub(crate) fn n26912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26728);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 54042);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -правоверие.
pub(crate) fn n131721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130366);
	s.is_empty()
}

/// Words ending -родноверие.
pub(crate) fn n131722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130367);
	s.is_empty()
}

/// Words ending -ьмисотлетие.
pub(crate) fn n131723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130368);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 135989);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139274);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 141178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -он-общежитие.
pub(crate) fn n131724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130369);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 135990);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139275);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 141179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обытие.
pub(crate) fn n54619(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 82381);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 105174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 82382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отбытие.
pub(crate) fn n83257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82383);
	s.is_empty()
}

/// Words ending -лытие.
pub(crate) fn n26932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26746);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 54062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мытие.
pub(crate) fn n26933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26747);
	s.is_empty()
}

/// Words ending -рытие.
pub(crate) fn n26934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26748);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 54063);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 54064);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82384);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятие.
pub(crate) fn n8339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8313);
	match s {
		[] => true,
		['ъ', 'з', s @ ..] => {
			super::push(out, 26749);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54065);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ейроотличие.
pub(crate) fn n131725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130370);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 135991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -двудышащие.
pub(crate) fn n131726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130371);
	s.is_empty()
}

/// Words ending -окружающие.
pub(crate) fn n131727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130372);
	s.is_empty()
}

/// Words ending -копитающие.
pub(crate) fn n131728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130373);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135992);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плектующие.
pub(crate) fn n131729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130374);
	s.is_empty()
}

/// Words ending -дунапентеле.
pub(crate) fn n137404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135993);
	s.is_empty()
}

/// Words ending -уум-мобиле.
pub(crate) fn n131732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130377);
	s.is_empty()
}

/// Words ending -колеоптиле.
pub(crate) fn n131733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130378);
	s.is_empty()
}

/// Words ending -тальятелле.
pub(crate) fn n131734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130379);
	s.is_empty()
}

/// Words ending -екати-поле.
pub(crate) fn n131735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130380);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 135994);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139277);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-на-майне.
pub(crate) fn n131739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130383);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 135998);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -арпеджионе.
pub(crate) fn n131740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130384);
	s.is_empty()
}

/// Words ending -брессаноне.
pub(crate) fn n131741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130385);
	s.is_empty()
}

/// Words ending -маскарпоне.
pub(crate) fn n131742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130386);
	s.is_empty()
}

/// Words ending -минестроне.
pub(crate) fn n131743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130387);
	s.is_empty()
}

/// Words ending -ловеческое.
pub(crate) fn n131747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130391);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136001);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 139281);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мурманское.
pub(crate) fn n131748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130392);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 136002);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139282);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 141182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знаменское.
pub(crate) fn n131749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130393);
	s.is_empty()
}

/// Words ending -синдинское.
pub(crate) fn n131750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130394);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атское.
pub(crate) fn n54753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54194);
	s.is_empty()
}

/// Words ending -цкое.
pub(crate) fn n8478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8452);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26896);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54195);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82486);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105246);
									s.is_empty()
								}
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
			super::push(out, 26897);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54196);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82487);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 105247);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькое.
pub(crate) fn n8479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8453);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 26898);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54197);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82488);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105248);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 105249);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 120817);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130395);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еобходимое.
pub(crate) fn n131752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130396);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -содержимое.
pub(crate) fn n131753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130397);
	s.is_empty()
}

/// Words ending -екомоядное.
pub(crate) fn n131754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130398);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еное.
pub(crate) fn n8491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8465);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 26924);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54231);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82520);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105272);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120828);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26925);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54232);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 54233);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 26926);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54234);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 82523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жное.
pub(crate) fn n8492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8466);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26927);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54235);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 26928);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54236);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26929);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54237);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82526);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105273);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120829);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130399);
													match s {
														[] => true,
														['е', s @ ..] => super::t45::n137417(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54238);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82527);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105274);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 120830);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130400);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 130401);
													match s {
														[] => true,
														['н', s @ ..] => super::t45::n137418(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54239);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82528);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105275);
									s.is_empty()
								}
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
			super::push(out, 26930);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 54240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 26931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зное.
pub(crate) fn n8493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8467);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26932);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82529);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 82530);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105276);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120831);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 130402);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 26933);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54242);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оромное.
pub(crate) fn n83406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82532);
	s.is_empty()
}

/// Words ending -нное.
pub(crate) fn n8495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8469);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26935);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54244);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82533);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 54245);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82534);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 54246);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82535);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105279);
									s.is_empty()
								}
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
			super::push(out, 26936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54247);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 82536);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105280);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120832);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 130403);
													match s {
														[] => true,
														['о', s @ ..] => super::t45::n137419(s, out),
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
									super::push(out, 105281);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120833);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130404);
													match s {
														[] => true,
														['с', s @ ..] => super::t45::n137420(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54248);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 54249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26937);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 54250);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82537);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105282);
									s.is_empty()
								}
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
			super::push(out, 26938);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 54251);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 54252);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пное.
pub(crate) fn n8496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8470);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 26939);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54253);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 82538);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 105283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 54254);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82539);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105284);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120834);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рное.
pub(crate) fn n8497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8471);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26940);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54255);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 82540);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105285);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 54256);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82541);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105286);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 120835);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26941);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 54257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26942);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54258);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 82542);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105287);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 120836);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26943);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54259);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сное.
pub(crate) fn n8498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8472);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 26944);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 54260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тное.
pub(crate) fn n8499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8473);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26945);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54261);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26946);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54262);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82545);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 105288);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 120837);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130405);
													match s {
														[] => true,
														['н', s @ ..] => super::t45::n137421(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54263);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82546);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 105289);
									s.is_empty()
								}
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
			super::push(out, 26947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54264);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82547);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105290);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 120838);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 130406);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137422(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 26948);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54265);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82548);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 105291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 82549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54266);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82550);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 105293);
									s.is_empty()
								}
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
			super::push(out, 26949);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 54267);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82551);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120839);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26950);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54268);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 82552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 54269);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82553);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 105295);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 120840);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130407);
													match s {
														[] => true,
														['н', s @ ..] => super::t45::n137423(s, out),
														_ => false,
													}
												}
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
							super::push(out, 82554);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 54270);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 26951);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 54271);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82556);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105297);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120841);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 130408);
													match s {
														[] => true,
														['р', s @ ..] => super::t45::n137424(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чное.
pub(crate) fn n8500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8474);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26952);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54272);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82557);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105298);
									s.is_empty()
								}
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
			super::push(out, 26953);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54273);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82558);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105299);
									s.is_empty()
								}
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
			super::push(out, 26954);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54274);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 54275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82560);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 82561);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 105300);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 120842);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130409);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54276);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82562);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 105301);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120843);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 130410);
													match s {
														[] => true,
														['о', s @ ..] => super::t45::n137425(s, out),
														_ => false,
													}
												}
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
							super::push(out, 82563);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105302);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120844);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130411);
													match s {
														[] => true,
														['н', s @ ..] => super::t45::n137426(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шное.
pub(crate) fn n8501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8475);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26955);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 54277);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щное.
pub(crate) fn n8502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8476);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 26956);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 54278);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьное.
pub(crate) fn n8503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8477);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 26957);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54279);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82566);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105303);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 120845);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 130412);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137427(s, out),
														_ => false,
													}
												}
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
							super::push(out, 82567);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105304);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120846);
											s.is_empty()
										}
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
					super::push(out, 54280);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 82568);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105305);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120847);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 120848);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 130413);
													match s {
														[] => true,
														['о', 'с', 'д', s @ ..] => super::t45::n137428(s, out),
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
									super::push(out, 105306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 54281);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82569);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105307);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 120849);
											s.is_empty()
										}
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
					super::push(out, 54282);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82570);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 105308);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120850);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130414);
													match s {
														[] => true,
														['н', s @ ..] => super::t45::n137429(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бугорчатое.
pub(crate) fn n131771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130415);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -остигнутое.
pub(crate) fn n131772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130416);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 136020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -к-на-амуре.
pub(crate) fn n131774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130418);
	s.is_empty()
}

/// Words ending -ьбиселесте.
pub(crate) fn n131780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130424);
	s.is_empty()
}

/// Words ending -тырговиште.
pub(crate) fn n131781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130425);
	s.is_empty()
}

/// Words ending -охлолюфтваффе.
pub(crate) fn n140733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139293);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 141188);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окрывальце.
pub(crate) fn n131789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130433);
	s.is_empty()
}

/// Words ending -брызгальце.
pub(crate) fn n131790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130434);
	s.is_empty()
}

/// The word кресельце.
pub(crate) fn n131791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130435);
	s.is_empty()
}

/// Words ending -тульце.
pub(crate) fn n54963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54404);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 82659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыльце.
pub(crate) fn n27282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27095);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 54405);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 54406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яльце.
pub(crate) fn n27283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27096);
	match s {
		[] => true,
		['е', 'д', 'о', '\u{1}', s @ ..] => {
			super::push(out, 54407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -допроебище.
pub(crate) fn n131792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130436);
	s.is_empty()
}

/// Words ending -оговище.
pub(crate) fn n83549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82675);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 105368);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 105369);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 120886);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -довище.
pub(crate) fn n54996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54437);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 82676);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105370);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 120887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 82677);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 105371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковище.
pub(crate) fn n54997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54438);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 82678);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 82679);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 82680);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105372);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 120888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловище.
pub(crate) fn n54998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54439);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 82681);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 82682);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 82683);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 105373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мовище.
pub(crate) fn n54999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54440);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 82684);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 105374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новище.
pub(crate) fn n55000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54441);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82685);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 105375);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 120890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рповище.
pub(crate) fn n83560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82686);
	s.is_empty()
}

/// Words ending -ровище.
pub(crate) fn n55002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54443);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 82687);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 82688);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105377);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 120891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 82689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -совище.
pub(crate) fn n55003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54444);
	s.is_empty()
}

/// Words ending -товище.
pub(crate) fn n55004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54445);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82690);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 82691);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 82692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховище.
pub(crate) fn n55005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54446);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 82693);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105379);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120893);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 130437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дище.
pub(crate) fn n8658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8632);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27127);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54447);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 54448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27128);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54449);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82694);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105380);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120894);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27129);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54450);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 27130);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 54451);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82696);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105381);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 120895);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130438);
													match s {
														[] => true,
														['р', s @ ..] => super::t45::n137439(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54452);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82697);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 105382);
									s.is_empty()
								}
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
			super::push(out, 27131);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54453);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82698);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 105383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54454);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82699);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 105384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54455);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82700);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 105385);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 82701);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 105386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 54456);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82702);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105387);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120897);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130439);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27132);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 54457);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 54458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27133);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 54459);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 27134);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жище.
pub(crate) fn n8659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8633);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27135);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 54461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27136);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54462);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82706);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 82707);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 105388);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120898);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130440);
													match s {
														[] => true,
														['ю', s @ ..] => super::t45::n137440(s, out),
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
									super::push(out, 105389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 54463);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82708);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105390);
									s.is_empty()
								}
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
			super::push(out, 27137);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 54464);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 54465);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 54466);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 54467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27138);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54468);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 82709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27139);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54469);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 54470);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зище.
pub(crate) fn n8660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8634);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27140);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54471);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82711);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105392);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120899);
											s.is_empty()
										}
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
					super::push(out, 54472);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 82712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105393);
									s.is_empty()
								}
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
			super::push(out, 27141);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 54473);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82713);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105394);
									s.is_empty()
								}
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
			super::push(out, 27142);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54474);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 54475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82715);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 82716);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54476);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27143);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54477);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82718);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кище.
pub(crate) fn n8661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8635);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 27144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лище.
pub(crate) fn n8662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27145);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 54479);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82720);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 82721);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105398);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120900);
											s.is_empty()
										}
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
					super::push(out, 54480);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82722);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105399);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120901);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 82723);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 82724);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105401);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120902);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130441);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54481);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 54482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 27146);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27147);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 54484);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82727);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105402);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120903);
											s.is_empty()
										}
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
					super::push(out, 54485);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 54486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 27148);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54487);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105403);
									s.is_empty()
								}
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
			super::push(out, 27149);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54488);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 54489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 54490);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 54491);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 54492);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 54493);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82732);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105404);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 120904);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130442);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27150);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54494);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105405);
									s.is_empty()
								}
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
			super::push(out, 27151);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54495);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82734);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 54496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мище.
pub(crate) fn n8663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8637);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27152);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 54497);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82735);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105407);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 120905);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27153);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54498);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27154);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54499);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27155);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54500);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 54501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 54502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27156);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54503);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 82739);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 105408);
									s.is_empty()
								}
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
			super::push(out, 27157);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 27158);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 27159);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54505);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82740);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нище.
pub(crate) fn n8664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8638);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27160);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54506);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82741);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105410);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120906);
											s.is_empty()
										}
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
					super::push(out, 54507);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82742);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105411);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120907);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 130443);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54508);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54509);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82744);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 54510);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82745);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105413);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120908);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130444);
													s.is_empty()
												}
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
							super::push(out, 82746);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105414);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120909);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 82747);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54511);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82748);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 105416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 54512);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82749);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105417);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 120910);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130445);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27161);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54513);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 82750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 27162);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27163);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 27164);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54515);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82751);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 105418);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 105419);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120911);
											s.is_empty()
										}
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
					super::push(out, 54516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27165);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 54518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54519);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 82754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 54520);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82755);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 54521);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82756);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 54522);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 82757);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 105422);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120912);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27166);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 27167);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54524);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27168);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54525);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82759);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105423);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120913);
											s.is_empty()
										}
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
					super::push(out, 54526);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82760);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 105424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54527);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82761);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 105425);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120914);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t45::n55087(s, out),
				['ф', s @ ..] => super::t45::n55088(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 27169);
			s.is_empty()
		}
		['р', s @ ..] => super::t45::n27357(s, out),
		['т', s @ ..] => super::t45::n27358(s, out),
		['у', s @ ..] => super::t45::n27359(s, out),
		['я', s @ ..] => super::t45::n27360(s, out),
		_ => false,
	}
}

/// Words ending -оище.
pub(crate) fn n8665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8639);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27174);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54537);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27175);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 54538);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пище.
pub(crate) fn n8666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8640);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27176);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27177);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 27178);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27179);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54542);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 54543);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82774);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 54544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27180);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27181);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54546);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82776);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рище.
pub(crate) fn n8667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8641);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27182);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54547);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 82777);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 54548);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82778);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 54549);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82779);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 54550);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82780);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105440);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120919);
											s.is_empty()
										}
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
					super::push(out, 54551);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82781);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 105441);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120920);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27183);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54552);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82782);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 54553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27184);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 54554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27185);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54555);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82783);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105443);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120921);
											s.is_empty()
										}
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
					super::push(out, 54556);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82784);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105444);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 82785);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105445);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120922);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130448);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27186);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54557);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82786);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 105446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 82787);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105447);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 82788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 54558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82789);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 82790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 54559);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82791);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105449);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 54560);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82792);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105450);
									s.is_empty()
								}
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
			super::push(out, 27187);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54561);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27188);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54562);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 54563);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82795);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105451);
									s.is_empty()
								}
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
			super::push(out, 27189);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54564);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 82796);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105452);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 120923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сище.
pub(crate) fn n8668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27190);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54566);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82798);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105453);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120924);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27191);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54567);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82799);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105454);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120925);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130449);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27192);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54569);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82800);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105455);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 120926);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130450);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54570);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 54571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27193);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54572);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 54573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тище.
pub(crate) fn n8669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8643);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 27194);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54574);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82804);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 82805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105456);
									s.is_empty()
								}
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
			super::push(out, 27195);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54575);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82806);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105457);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120927);
											s.is_empty()
										}
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
					super::push(out, 54576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 54577);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82808);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 105458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82809);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 105459);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 120928);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130451);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54579);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27196);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54580);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 82811);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105460);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 120929);
											s.is_empty()
										}
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
					super::push(out, 54581);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 82812);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105461);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 120930);
											s.is_empty()
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 105462);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120931);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 105463);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 120932);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130452);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137442(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54582);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27197);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82815);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 82816);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 105464);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120933);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 130453);
													match s {
														[] => true,
														['о', s @ ..] => super::t45::n137443(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27198);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54585);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 82817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27199);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54586);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 54587);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27200);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54588);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82819);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 105465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 82820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 82821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 54589);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27201);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54590);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82823);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 54591);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82824);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 54592);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82825);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105470);
									s.is_empty()
								}
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
			super::push(out, 27202);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54593);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82826);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уище.
pub(crate) fn n8670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8644);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 27203);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чище.
pub(crate) fn n8671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8645);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27204);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54595);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 54596);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82828);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 105472);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											super::push(out, 120934);
											s.is_empty()
										}
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
					super::push(out, 54597);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82829);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 54598);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82830);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105474);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 120935);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27205);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54599);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82831);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105475);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120936);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 130454);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54600);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 54601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27206);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 54603);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82834);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 27207);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54605);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82835);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105477);
									s.is_empty()
								}
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
			super::push(out, 27208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54606);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 82836);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 82837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105478);
									s.is_empty()
								}
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
			super::push(out, 27209);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54607);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27210);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54608);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27211);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54609);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 54610);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82841);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 105479);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 120937);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130455);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 54612);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82843);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105480);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120938);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130456);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27212);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 82844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 54614);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 82845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 27213);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54615);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 27214);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 54616);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 82847);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105481);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 120939);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 130457);
													match s {
														[] => true,
														['с', s @ ..] => super::t45::n137444(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шище.
pub(crate) fn n8672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8646);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27215);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 54617);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82848);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105482);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 120940);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27216);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54618);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 82849);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьище.
pub(crate) fn n8673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8647);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 27217);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54619);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яище.
pub(crate) fn n8674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8648);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 27218);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54620);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82851);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 105484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льпигиевые.
pub(crate) fn n131815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130459);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136034);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 139299);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -акардиевые.
pub(crate) fn n131816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130460);
	s.is_empty()
}

/// Words ending -ффлезиевые.
pub(crate) fn n131817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130461);
	s.is_empty()
}

/// Words ending -бегониевые.
pub(crate) fn n131818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130462);
	s.is_empty()
}

/// Words ending -литориевые.
pub(crate) fn n131819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130463);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136035);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 139300);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -калициевые.
pub(crate) fn n131820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130464);
	s.is_empty()
}

/// Words ending -официевые.
pub(crate) fn n122221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120950);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 130465);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 136036);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139301);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 141190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -левые.
pub(crate) fn n27410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27223);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 54632);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 82863);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 105497);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120951);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 130466);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -боевые.
pub(crate) fn n55192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54633);
	s.is_empty()
}

/// Words ending -ревые.
pub(crate) fn n27412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27225);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54634);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 82864);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -севые.
pub(crate) fn n27413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27226);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 54635);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 82865);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105498);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 120952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чевые.
pub(crate) fn n27414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27227);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 54636);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82866);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овые.
pub(crate) fn n8677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8651);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27228);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54637);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82867);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105500);
									s.is_empty()
								}
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
			super::push(out, 27229);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 54638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 27230);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 54639);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82868);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 54640);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82869);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105502);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120953);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27231);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54641);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82870);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105503);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 120954);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130467);
													match s {
														[] => true,
														['д', s @ ..] => super::t45::n137448(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54642);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82871);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105504);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 120955);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 82872);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105505);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 120956);
											s.is_empty()
										}
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
					super::push(out, 54643);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82873);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 105506);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120957);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130468);
													match s {
														[] => true,
														['с', s @ ..] => super::t45::n137449(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54644);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 82874);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105507);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120958);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130469);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54645);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82875);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 105508);
									s.is_empty()
								}
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
			super::push(out, 27232);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54646);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82876);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 105509);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120959);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 130470);
													match s {
														[] => true,
														['и', s @ ..] => super::t45::n137450(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54647);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82877);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105510);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 120960);
											s.is_empty()
										}
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
					super::push(out, 54648);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 82878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t45::n27420(s, out),
		['л', s @ ..] => {
			super::push(out, 27234);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54661);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 82902);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105546);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 120997);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 130496);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t45::n83777(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t45::n55221(s, out),
				['о', s @ ..] => super::t45::n55222(s, out),
				['т', s @ ..] => {
					super::push(out, 54664);
					match s {
						[] => true,
						['я', s @ ..] => super::t45::n83780(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t45::n55224(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t45::n27422(s, out),
		['н', s @ ..] => super::t45::n27423(s, out),
		['п', s @ ..] => super::t45::n27424(s, out),
		['р', s @ ..] => super::t45::n27425(s, out),
		['с', s @ ..] => super::t45::n27426(s, out),
		['т', s @ ..] => super::t45::n27427(s, out),
		['х', s @ ..] => super::t45::n27428(s, out),
		_ => false,
	}
}

/// Words ending -ювые.
pub(crate) fn n8678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8652);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 27242);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54690);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82943);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105584);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121027);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 130517);
													match s {
														[] => true,
														['н', s @ ..] => super::t45::n137481(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сцифоидные.
pub(crate) fn n131878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130522);
	s.is_empty()
}

/// Words ending -опроходные.
pub(crate) fn n131879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130523);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136073);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 139316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -но-зеленые.
pub(crate) fn n131880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130524);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 136074);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139317);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -то-зеленые.
pub(crate) fn n131881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130525);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136075);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139318);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 141195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -потиражные.
pub(crate) fn n131882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130526);
	s.is_empty()
}

/// Words ending -зные.
pub(crate) fn n8686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8660);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27251);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54702);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105598);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121037);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130527);
													match s {
														[] => true,
														['ф', s @ ..] => super::t45::n137487(s, out),
														_ => false,
													}
												}
												['в', s @ ..] => {
													super::push(out, 130528);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137488(s, out),
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
													super::push(out, 130529);
													match s {
														[] => true,
														['с', s @ ..] => super::t45::n137489(s, out),
														_ => false,
													}
												}
												['у', s @ ..] => {
													super::push(out, 130530);
													match s {
														[] => true,
														['г', s @ ..] => super::t45::n137490(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 121038);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 130531);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137491(s, out),
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
									super::push(out, 105599);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 121039);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130532);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 121040);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130533);
													match s {
														[] => true,
														['р', s @ ..] => super::t45::n137492(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													match s {
														[] => true,
														['е', s @ ..] => super::t45::n137493(s, out),
														['ю', s @ ..] => super::t45::n137494(s, out),
														_ => false,
													}
												}
												['у', s @ ..] => {
													super::push(out, 130534);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137495(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 121041);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130535);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 121042);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130536);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 121043);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 130537);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бьиные.
pub(crate) fn n55264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54705);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 82957);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105601);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мные.
pub(crate) fn n8688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8662);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27255);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 54706);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82958);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105602);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121046);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27256);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54707);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 82959);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105603);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 121047);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130538);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нные.
pub(crate) fn n8689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8663);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27257);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54708);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82960);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 105604);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 121048);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130539);
													match s {
														[] => true,
														['м', s @ ..] => super::t45::n137496(s, out),
														_ => false,
													}
												}
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
							super::push(out, 82961);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105605);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121049);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 130540);
													s.is_empty()
												}
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
							super::push(out, 82962);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 121050);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 121051);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 130541);
													match s {
														[] => true,
														['в', s @ ..] => super::t45::n137497(s, out),
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
									super::push(out, 105606);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 121052);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130542);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137498(s, out),
														_ => false,
													}
												}
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
							super::push(out, 82963);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105607);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 121053);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130543);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27258);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54709);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82964);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105608);
									s.is_empty()
								}
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
			super::push(out, 27259);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 54710);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82965);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105609);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121054);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 130544);
													match s {
														[] => true,
														['о', s @ ..] => super::t45::n137499(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пные.
pub(crate) fn n8690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8664);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 27260);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 54711);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 82966);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105610);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121055);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рные.
pub(crate) fn n8691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8665);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27261);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54712);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82967);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 105611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 54713);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 82968);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105612);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 121056);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130545);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137500(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сные.
pub(crate) fn n8692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8666);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27262);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54714);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82969);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105613);
									s.is_empty()
								}
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
			super::push(out, 27263);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54715);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 82970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27264);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 54716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82971);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105614);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121057);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 130546);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тные.
pub(crate) fn n8693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8667);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27265);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54717);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 82972);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105615);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121058);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 130547);
													match s {
														[] => true,
														['и', s @ ..] => super::t45::n137501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 121059);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 130548);
													match s {
														[] => true,
														['е', s @ ..] => super::t45::n137502(s, out),
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
									super::push(out, 105616);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 121060);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 130549);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 121061);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130550);
													match s {
														[] => true,
														['р', s @ ..] => super::t45::n137503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 121062);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130551);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 130552);
													match s {
														[] => true,
														['о', s @ ..] => super::t45::n137504(s, out),
														_ => false,
													}
												}
												['у', s @ ..] => {
													super::push(out, 130553);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 121063);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 121064);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130554);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 121065);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 130555);
													match s {
														[] => true,
														['е', s @ ..] => super::t45::n137505(s, out),
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
									super::push(out, 105617);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 121066);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130556);
													match s {
														[] => true,
														['т', s @ ..] => super::t45::n137506(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27266);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54718);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 82973);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 105618);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121067);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130557);
													match s {
														[] => true,
														['с', s @ ..] => super::t45::n137507(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t45::n27454(s, out),
		['ы', s @ ..] => super::t45::n27455(s, out),
		_ => false,
	}
}

/// Words ending -чные.
pub(crate) fn n8694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8668);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27269);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54721);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 82976);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105621);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 121070);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27270);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 54722);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 82977);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 105622);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 54723);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 82978);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105623);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 121071);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 130560);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54724);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 82979);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105624);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 121072);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27271);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54725);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82980);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 105625);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121073);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130561);
													match s {
														[] => true,
														['в', s @ ..] => super::t45::n137510(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54726);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 82981);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105626);
									s.is_empty()
								}
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
			super::push(out, 27272);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 54727);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 82982);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьные.
pub(crate) fn n8695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8669);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 27273);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54728);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 82983);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105628);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 121074);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130562);
													match s {
														[] => true,
														['п', s @ ..] => super::t45::n137511(s, out),
														_ => false,
													}
												}
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
							super::push(out, 82984);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105629);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 121075);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130563);
													match s {
														[] => true,
														['л', s @ ..] => super::t45::n137512(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54729);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 82985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чешуйчатые.
pub(crate) fn n131922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130565);
	s.is_empty()
}

/// Words ending -бугорчатые.
pub(crate) fn n131923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130566);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -подмосковье.
pub(crate) fn n137514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136103);
	s.is_empty()
}

/// The word пригородье.
pub(crate) fn n131927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130570);
	s.is_empty()
}

/// Words ending -безденежье.
pub(crate) fn n131928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130571);
	s.is_empty()
}

/// The word набережье.
pub(crate) fn n131929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130572);
	s.is_empty()
}

/// Words ending -седертелье.
pub(crate) fn n131931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130574);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -асилье.
pub(crate) fn n55371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54812);
	s.is_empty()
}

/// Words ending -олье.
pub(crate) fn n8730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8704);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27350);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 27351);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 54813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27352);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 27353);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54814);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 54815);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83043);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105665);
									s.is_empty()
								}
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
			super::push(out, 27354);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54816);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 54817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 27355);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 54818);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83044);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 105666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 54819);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83045);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 105667);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121098);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130575);
													match s {
														[] => true,
														['с', s @ ..] => super::t45::n137516(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54820);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 83046);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 54821);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 83047);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105669);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121099);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130576);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t45::n137517(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 54822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27356);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 54823);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83048);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 105670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 83049);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105671);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121100);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130577);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улье.
pub(crate) fn n8731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8705);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27357);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 54824);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83050);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105672);
									s.is_empty()
								}
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
			super::push(out, 27358);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 27359);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 27360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ылье.
pub(crate) fn n8732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8706);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27361);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 27362);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54825);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 83051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27363);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анье.
pub(crate) fn n8739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8713);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27368);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54830);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 83053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t45::n27557(s, out),
		['г', s @ ..] => {
			super::push(out, 27370);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54839);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 54840);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 83101);
							s.is_empty()
						}
						['р', '\u{1}', s @ ..] => {
							super::push(out, 83102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 54841);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 83103);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 83104);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105744);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 121167);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 83105);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 54842);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 83106);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 105746);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 83107);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 83108);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 105747);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 121168);
											s.is_empty()
										}
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
					super::push(out, 54843);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 83109);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105748);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 121169);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 83110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'е', s @ ..] => {
					super::push(out, 54844);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 83111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 54845);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 54846);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 83113);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 105749);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105750);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 105751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 54847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27371);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54848);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 83114);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'в', s @ ..] => {
					super::push(out, 54849);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83115);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105753);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 121170);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 130624);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 54850);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 54851);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 54852);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 83116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105754);
									s.is_empty()
								}
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
			super::push(out, 27372);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54853);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 27373);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54854);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 54855);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 83118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 27374);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54856);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 83119);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 83120);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 105756);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121171);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 83121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					super::push(out, 54857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 54858);
					s.is_empty()
				}
				['л', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 54859);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 54860);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 54861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 54862);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 83124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t45::n27563(s, out),
		['л', s @ ..] => {
			super::push(out, 27376);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54878);
					match s {
						[] => true,
						['д', s @ ..] => super::t45::n84044(s, out),
						['ж', s @ ..] => {
							super::push(out, 83158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 54879);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 54880);
					s.is_empty()
				}
				['о', s @ ..] => super::t45::n55443(s, out),
				['с', s @ ..] => super::t45::n55444(s, out),
				['т', s @ ..] => super::t45::n55445(s, out),
				['ы', s @ ..] => super::t45::n55446(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t45::n27565(s, out),
		['н', s @ ..] => super::t45::n27566(s, out),
		['п', s @ ..] => super::t45::n27567(s, out),
		['р', s @ ..] => super::t45::n27568(s, out),
		['с', s @ ..] => super::t45::n27569(s, out),
		['т', s @ ..] => super::t45::n27570(s, out),
		['х', s @ ..] => super::t45::n27571(s, out),
		['ц', s @ ..] => super::t45::n27572(s, out),
		['ч', s @ ..] => super::t45::n27573(s, out),
		['ш', s @ ..] => super::t45::n27574(s, out),
		['щ', s @ ..] => super::t45::n27575(s, out),
		_ => false,
	}
}

/// Words ending -енье.
pub(crate) fn n8740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8714);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27388);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 83242);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105860);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 121231);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', 'о', s @ ..] => {
									super::push(out, 105861);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121232);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130650);
													s.is_empty()
												}
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
							super::push(out, 83243);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 83244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 54946);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 83245);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105863);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121233);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130651);
													match s {
														[] => true,
														['ш', s @ ..] => super::t45::n137555(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 54947);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 54948);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 54949);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 83246);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105864);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 121234);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 105865);
									s.is_empty()
								}
								['д', 'е', s @ ..] => {
									super::push(out, 105866);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121235);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130652);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 105867);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121236);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 121237);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 121238);
											s.is_empty()
										}
										['ц', s @ ..] => {
											super::push(out, 121239);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 105868);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 121240);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 130653);
													match s {
														[] => true,
														['к', s @ ..] => super::t45::n137556(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', 'у', s @ ..] => {
											super::push(out, 121241);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130654);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137557(s, out),
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
									super::push(out, 105869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 83247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 54950);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 54951);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 83249);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105870);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 105871);
									s.is_empty()
								}
								['о', 'н', s @ ..] => {
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 121242);
											s.is_empty()
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 130655);
													s.is_empty()
												}
												['я', s @ ..] => {
													super::push(out, 130656);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ь', s @ ..] => {
											super::push(out, 121243);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 83250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 54952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t45::n27578(s, out),
		['л', s @ ..] => super::t45::n27579(s, out),
		['м', s @ ..] => super::t45::n27580(s, out),
		['н', s @ ..] => {
			super::push(out, 27393);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27394);
			match s {
				[] => true,
				['р', 'т', 'с', 'о', 'т', 'е', 'л', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
					super::push(out, 54966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 27395);
			s.is_empty()
		}
		['р', s @ ..] => super::t45::n27584(s, out),
		['с', s @ ..] => {
			super::push(out, 27397);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 27398);
			match s {
				[] => true,
				['е', 'р', s @ ..] => super::t45::n55541(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t46::n27587(s, out),
		['ш', s @ ..] => super::t46::n27588(s, out),
		['щ', s @ ..] => super::t46::n27589(s, out),
		_ => false,
	}
}

/// The word подоконье.
pub(crate) fn n132041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130684);
	s.is_empty()
}

/// Words ending -ашлянье.
pub(crate) fn n84216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83325);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 105934);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 121285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оянье.
pub(crate) fn n27602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27414);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 55010);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рянье.
pub(crate) fn n27603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27415);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 55011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одмастерье.
pub(crate) fn n132042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130685);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 136165);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иземноморье.
pub(crate) fn n132043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130686);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 136166);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139366);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырье.
pub(crate) fn n8754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8728);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 27450);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 55040);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 55041);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 55042);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83343);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 105942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрье.
pub(crate) fn n8755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8729);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 27451);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 55043);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 83344);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поднебесье.
pub(crate) fn n132044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130687);
	s.is_empty()
}

/// Words ending -онферансье.
pub(crate) fn n132045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130688);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 136167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -адпереносье.
pub(crate) fn n132046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130689);
	s.is_empty()
}

/// Words ending -вик-платье.
pub(crate) fn n132047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130690);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136168);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 139367);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 141218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отье.
pub(crate) fn n8767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8741);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 27484);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 55078);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83366);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105956);
									s.is_empty()
								}
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
			super::push(out, 27485);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55079);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 83367);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 105957);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 121298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 83368);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 83369);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 105958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 83370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 27486);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55080);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 83371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 55081);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83372);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 105959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 55082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27487);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 55083);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 83373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27488);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55084);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 83374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ортье.
pub(crate) fn n27680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27490);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 55086);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83376);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уртье.
pub(crate) fn n27681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27491);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 55087);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стье.
pub(crate) fn n8769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8743);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27492);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 27493);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27494);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 55088);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 55089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 55090);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 55091);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 83378);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 105960);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121299);
											s.is_empty()
										}
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
					super::push(out, 55092);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 55093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 27496);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 55094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83381);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 83382);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105961);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 121300);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 130691);
													match s {
														[] => true,
														['а', s @ ..] => super::t46::n137580(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утье.
pub(crate) fn n8770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8744);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 27497);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 55095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27498);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 55096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 27499);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55097);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 55098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 27500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытье.
pub(crate) fn n8771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8745);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27501);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 55099);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 83383);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 105962);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 121301);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 130692);
													match s {
														[] => true,
														['ж', s @ ..] => super::t46::n137581(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 55100);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 83384);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105963);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 105964);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121302);
											s.is_empty()
										}
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
					super::push(out, 55101);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 83385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 55102);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83386);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 105965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 55103);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 83387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 55104);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 27502);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55105);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 83389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 27503);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 27504);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 27505);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 27506);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 27507);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 55106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютье.
pub(crate) fn n8772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8746);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 27508);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 55107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятье.
pub(crate) fn n8773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8747);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27509);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 55108);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27510);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 55109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 27511);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 27512);
			s.is_empty()
		}
		['ъ', s @ ..] => {
			super::push(out, 27513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дверстачье.
pub(crate) fn n132050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130693);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136171);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 139369);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дсумеречье.
pub(crate) fn n132051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130694);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136172);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139370);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -подлавочье.
pub(crate) fn n132052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130695);
	s.is_empty()
}

/// Words ending -лочье.
pub(crate) fn n27719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27528);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 55125);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 83405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 55126);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83406);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рочье.
pub(crate) fn n27721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27530);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 55128);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 83407);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 105974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учье.
pub(crate) fn n8779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8753);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27531);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 55129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27532);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 55130);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83408);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 105975);
									s.is_empty()
								}
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
			super::push(out, 27533);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27534);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 55132);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83409);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105976);
									s.is_empty()
								}
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
			super::push(out, 27535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ычье.
pub(crate) fn n8780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8754);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 27536);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стоунхендж.
pub(crate) fn n132059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130702);
	s.is_empty()
}

/// Words ending -ла-листорез.
pub(crate) fn n137592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136181);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139376);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 141223);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 142284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кольпейриз.
pub(crate) fn n132066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130709);
	s.is_empty()
}

/// Words ending -мобиль-битумовоз.
pub(crate) fn n142671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141225);
	s.is_empty()
}

/// Words ending -биль-мусоровоз.
pub(crate) fn n140819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139379);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141226);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -биль-цементовоз.
pub(crate) fn n140820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139380);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141227);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гиперкератоз.
pub(crate) fn n132072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130715);
	s.is_empty()
}

/// Words ending -ирма-колхоз.
pub(crate) fn n132073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130716);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 136186);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139381);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -веб-услуги.
pub(crate) fn n132080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130723);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 136189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пол-россии.
pub(crate) fn n132085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130728);
	s.is_empty()
}

/// Words ending -ол-епархии.
pub(crate) fn n132086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130729);
	s.is_empty()
}

/// Words ending -ол-франции.
pub(crate) fn n132087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130730);
	s.is_empty()
}

/// The word выскребки.
pub(crate) fn n132088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130731);
	s.is_empty()
}

/// Words ending -есопосадки.
pub(crate) fn n132089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130732);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -полселедки.
pub(crate) fn n132090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130733);
	s.is_empty()
}

/// Words ending -республики.
pub(crate) fn n132091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130734);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ики-нолики.
pub(crate) fn n132092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130735);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 136192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нэкономики.
pub(crate) fn n132093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130736);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 136193);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 139383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word березники.
pub(crate) fn n132094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130737);
	s.is_empty()
}

/// Words ending -ессалоники.
pub(crate) fn n132095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130738);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 136194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пики.
pub(crate) fn n9435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9408);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 28233);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 55658);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 83744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рики.
pub(crate) fn n9436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9409);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28234);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 55659);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83745);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 106157);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 121401);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130739);
													match s {
														[] => true,
														['п', s @ ..] => super::t46::n137606(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 28235);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 28236);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 55660);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 83746);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106158);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121402);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130740);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сики.
pub(crate) fn n9437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9410);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 28237);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 55661);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 83747);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 106159);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121403);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 28238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тики.
pub(crate) fn n9438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9411);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 28239);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 55662);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83748);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 106160);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121404);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 130741);
													match s {
														[] => true,
														['р', s @ ..] => super::t46::n137607(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чики.
pub(crate) fn n9439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9412);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28240);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьваностоки.
pub(crate) fn n132099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130742);
	s.is_empty()
}

/// Words ending -младотурки.
pub(crate) fn n132100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130743);
	s.is_empty()
}

/// Words ending -киберриски.
pub(crate) fn n132101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130744);
	s.is_empty()
}

/// Words ending -камифубуки.
pub(crate) fn n132102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130745);
	s.is_empty()
}

/// Words ending -штанишечки.
pub(crate) fn n132103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130746);
	s.is_empty()
}

/// Words ending -сочки.
pub(crate) fn n28567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28373);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 55771);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 55772);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 83824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 55773);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 83825);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точки.
pub(crate) fn n28568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28374);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 55774);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 55775);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 83826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 106201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 55776);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 83827);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106202);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121425);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 55777);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83828);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 106203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 55778);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83829);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 106204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121426);
									s.is_empty()
								}
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
			super::push(out, 55779);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 55780);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чочки.
pub(crate) fn n28569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28375);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 55781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83830);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючки.
pub(crate) fn n9502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9475);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28376);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 55782);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 83831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 28377);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 55783);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 55784);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 83832);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 106205);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 121427);
											match s {
												[] => true,
												['ч', 'у', 'т', 'ш', '\u{1}', s @ ..] => {
													super::push(out, 130748);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гушки.
pub(crate) fn n28589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28395);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 55804);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 83847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 55805);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 83848);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -адушки.
pub(crate) fn n56379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55806);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 83849);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мушки.
pub(crate) fn n28591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28397);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 55807);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 83850);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106216);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 121432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

