use flexion::Path;

/// Words ending -чина.
pub(crate) fn n6130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23204);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50289);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 79052);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102540);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 79053);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102541);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118960);
											s.is_empty()
										}
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
					super::push(out, 50290);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79054);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 79055);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102543);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118961);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 79056);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102544);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 102545);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 102546);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118962);
											s.is_empty()
										}
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
					super::push(out, 50291);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79057);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 50292);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79058);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50293);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79059);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102549);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 79060);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102550);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 118964);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 79061);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102551);
									s.is_empty()
								}
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
			super::push(out, 23205);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50294);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 79062);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 50295);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79063);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 50296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 23206);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50297);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 50298);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79066);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102554);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118965);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129250);
													s.is_empty()
												}
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
							super::push(out, 79067);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102555);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118966);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23207);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50299);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 50300);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79068);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102556);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118967);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 79069);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102557);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118968);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129251);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50301);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 50302);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 50303);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79070);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102558);
									s.is_empty()
								}
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
			super::push(out, 23208);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 50304);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79071);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 50305);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79072);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t44::n23344(s, out),
		['й', s @ ..] => {
			super::push(out, 23210);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50312);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 79084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t44::n23346(s, out),
		['о', s @ ..] => super::t44::n23347(s, out),
		['п', s @ ..] => super::t44::n23348(s, out),
		['р', s @ ..] => super::t44::n23349(s, out),
		['с', s @ ..] => super::t44::n23350(s, out),
		['т', s @ ..] => super::t44::n23351(s, out),
		['у', s @ ..] => super::t44::n23352(s, out),
		['ы', s @ ..] => super::t44::n23353(s, out),
		['ю', s @ ..] => super::t44::n23354(s, out),
		['я', s @ ..] => super::t44::n23355(s, out),
		_ => false,
	}
}

/// Words ending -шина.
pub(crate) fn n6131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6116);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23221);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 50342);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 50343);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79113);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102579);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 118980);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 129255);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t44::n136789(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23222);
			match s {
				[] => true,
				['л', 'п', 'о', s @ ..] => {
					super::push(out, 50344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79114);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50345);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102581);
									s.is_empty()
								}
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
			super::push(out, 23223);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 50346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79116);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 79117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 23224);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50347);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79118);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102582);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118981);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129256);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23225);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23226);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 50348);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102583);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 102584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 50349);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79119);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102585);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118982);
											s.is_empty()
										}
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
						['в', s @ ..] => {
							super::push(out, 79120);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 79121);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102587);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118983);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23227);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50350);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 79122);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102588);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118984);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129257);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t44::n136790(s, out),
														_ => false,
													}
												}
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
							super::push(out, 79123);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102589);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118985);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 118986);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129258);
													match s {
														[] => true,
														['ц', s @ ..] => super::t44::n136791(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 118987);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129259);
													match s {
														[] => true,
														['л', s @ ..] => super::t44::n136792(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 50351);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23228);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 50352);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23229);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50353);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79126);
							match s {
								[] => true,
								['р', s @ ..] => super::t44::n103680(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t44::n50868(s, out),
				['е', s @ ..] => super::t44::n50869(s, out),
				['л', s @ ..] => super::t44::n50870(s, out),
				['о', s @ ..] => super::t44::n50871(s, out),
				['р', s @ ..] => super::t44::n50872(s, out),
				['с', s @ ..] => super::t44::n50873(s, out),
				['т', s @ ..] => super::t44::n50874(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t44::n23365(s, out),
		['ы', s @ ..] => super::t44::n23366(s, out),
		['ю', s @ ..] => super::t44::n23367(s, out),
		_ => false,
	}
}

/// Words ending -щина.
pub(crate) fn n6132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6117);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50364);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79143);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 50365);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 50366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79144);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 79145);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102605);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 118994);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t44::n23369(s, out),
		['е', s @ ..] => {
			super::push(out, 23235);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 50371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 50372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 50373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50374);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 23236);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50375);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 50376);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79177);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102677);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119073);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 129327);
													match s {
														[] => true,
														['г', s @ ..] => super::t44::n136838(s, out),
														_ => false,
													}
												}
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
							super::push(out, 79178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23237);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50377);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23238);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50378);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 79180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 50379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50380);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79182);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102678);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 119074);
											match s {
												[] => true,
												['а', s @ ..] => super::t44::n130680(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => super::t44::n80013(s, out),
						['з', s @ ..] => super::t44::n80014(s, out),
						['к', s @ ..] => super::t44::n80015(s, out),
						['м', s @ ..] => super::t44::n80016(s, out),
						['п', s @ ..] => super::t44::n80017(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t44::n50896(s, out),
				['и', s @ ..] => super::t44::n50897(s, out),
				['о', s @ ..] => super::t44::n50898(s, out),
				['ы', s @ ..] => super::t44::n50899(s, out),
				['я', s @ ..] => super::t44::n50900(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t44::n23375(s, out),
		['п', s @ ..] => super::t44::n23376(s, out),
		['р', s @ ..] => super::t44::n23377(s, out),
		['т', s @ ..] => super::t44::n23378(s, out),
		['у', s @ ..] => super::t44::n23379(s, out),
		['ь', s @ ..] => super::t44::n23380(s, out),
		_ => false,
	}
}

/// Words ending -марьиванна.
pub(crate) fn n130701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129349);
	s.is_empty()
}

/// Words ending -мачта-антенна.
pub(crate) fn n130702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 135447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -белладонна.
pub(crate) fn n130703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129351);
	s.is_empty()
}

/// Words ending -виконна.
pub(crate) fn n80074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79242);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 102727);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 119113);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лконна.
pub(crate) fn n50943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50428);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 79243);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 102728);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 119114);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лонна.
pub(crate) fn n23414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23275);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 50429);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79244);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 102729);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119115);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 129354);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135448);
													match s {
														[] => true,
														['а', s @ ..] => super::t44::n140397(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нонна.
pub(crate) fn n23415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23276);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 50430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тонна.
pub(crate) fn n23416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23277);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50431);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 79245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унна.
pub(crate) fn n6145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6129);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 23278);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-лондона.
pub(crate) fn n130707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129355);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 135449);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олмиллиона.
pub(crate) fn n130708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129356);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 135450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пол-лимона.
pub(crate) fn n130709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129357);
	s.is_empty()
}

/// Words ending -запона.
pub(crate) fn n50965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50450);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 79260);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ипона.
pub(crate) fn n23440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23301);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 50451);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79261);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рона.
pub(crate) fn n6157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6141);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23302);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 23303);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 50452);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79262);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102737);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119119);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129358);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t44::n136857(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23304);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79263);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 79264);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102738);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 102739);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119120);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129359);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 50455);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 50456);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79267);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102740);
									s.is_empty()
								}
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
			super::push(out, 23305);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50457);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23306);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 23307);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50458);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сона.
pub(crate) fn n6158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6142);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 23308);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50459);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тона.
pub(crate) fn n6159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6143);
	s.is_empty()
}

/// Words ending -фона.
pub(crate) fn n6160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6144);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23309);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 50460);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79271);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102741);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119121);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хона.
pub(crate) fn n6161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6145);
	s.is_empty()
}

/// Words ending -цона.
pub(crate) fn n6162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6146);
	s.is_empty()
}

/// Words ending -шона.
pub(crate) fn n6163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6147);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьона.
pub(crate) fn n6164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6148);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23312);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 50461);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79272);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102742);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119122);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ов-цистерна.
pub(crate) fn n130712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129360);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 135452);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 138963);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 140983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -он-цистерна.
pub(crate) fn n130713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129361);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 135453);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138964);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 140984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еп-цистерна.
pub(crate) fn n130714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129362);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 135454);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 138965);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 140985);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 142165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ль-цистерна.
pub(crate) fn n130715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129363);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 135455);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 138966);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140986);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 142166);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142828);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ихтиофауна.
pub(crate) fn n130716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129364);
	s.is_empty()
}

/// Words ending -карлскруна.
pub(crate) fn n130717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129365);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 135456);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скильстуна.
pub(crate) fn n130718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129366);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 135457);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -братанишна.
pub(crate) fn n130719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129367);
	s.is_empty()
}

/// Words ending -пол-алтына.
pub(crate) fn n130720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129368);
	s.is_empty()
}

/// Words ending -нта-барбара.
pub(crate) fn n136864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135458);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138967);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 140987);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыба-зебра.
pub(crate) fn n130722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129370);
	s.is_empty()
}

/// Words ending -саламандра.
pub(crate) fn n130723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129371);
	s.is_empty()
}

/// Words ending -ундра.
pub(crate) fn n23700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23559);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 50680);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79404);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102809);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119152);
									s.is_empty()
								}
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
			super::push(out, 50681);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 79405);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 102810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удра.
pub(crate) fn n6329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6313);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23561);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 23562);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 23563);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 50684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 23564);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50685);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 23565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50686);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдра.
pub(crate) fn n6330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6314);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23566);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пельтигера.
pub(crate) fn n130724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129372);
	s.is_empty()
}

/// Words ending -ент-камера.
pub(crate) fn n130725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129373);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 135459);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 138968);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 140988);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142167);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 142829);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ромера.
pub(crate) fn n51221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50706);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 79420);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 102816);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмера.
pub(crate) fn n23724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23583);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 50707);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 79421);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102817);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умера.
pub(crate) fn n23725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23584);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 50708);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79422);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нера.
pub(crate) fn n6340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6324);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23585);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23586);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					super::push(out, 50709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пера.
pub(crate) fn n6341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6325);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 23587);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50710);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23588);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 50711);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79424);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 79425);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 102819);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119158);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 129374);
													match s {
														[] => true,
														['ф', s @ ..] => super::t44::n136866(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 50712);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рера.
pub(crate) fn n6342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6326);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23589);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 50713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сера.
pub(crate) fn n6343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6327);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 23590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50714);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79427);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102820);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 119159);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 129375);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тера.
pub(crate) fn n6344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6328);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23591);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23592);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 50715);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 50716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23593);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50717);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23594);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 23595);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50718);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79429);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102821);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119160);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129376);
													s.is_empty()
												}
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
							super::push(out, 79430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23596);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 23597);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 50719);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 50720);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79432);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 102822);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119161);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23598);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 50721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 50722);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 79434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 50723);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 79435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фера.
pub(crate) fn n6345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6329);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23599);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 50724);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102824);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119162);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129377);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50725);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23600);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					super::push(out, 50726);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79438);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чера.
pub(crate) fn n6347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6331);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23602);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шера.
pub(crate) fn n6348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6332);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23604);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 50729);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щера.
pub(crate) fn n6349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6333);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьера.
pub(crate) fn n6350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6334);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23606);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 23607);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50730);
					match s {
						[] => true,
						['р', 'п', 'м', s @ ..] => {
							super::push(out, 79439);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102826);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 119163);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129378);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23608);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50731);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 50732);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79441);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 102827);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119164);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 129379);
													match s {
														[] => true,
														['а', s @ ..] => super::t44::n136867(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23609);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 50733);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 50734);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79442);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102828);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119165);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 129380);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таб-квартира.
pub(crate) fn n130733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129381);
	s.is_empty()
}

/// Words ending -зей-квартира.
pub(crate) fn n130734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129382);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 135462);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138969);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ин-квартира.
pub(crate) fn n120430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119168);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 129383);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135463);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138970);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140989);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142168);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фира.
pub(crate) fn n6367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6351);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23639);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50752);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 50753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23640);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50754);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 79453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23641);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50755);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хира.
pub(crate) fn n6368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6352);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23642);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 50756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цира.
pub(crate) fn n6369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6353);
	s.is_empty()
}

/// Words ending -шира.
pub(crate) fn n6370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6354);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23643);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50757);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бжора.
pub(crate) fn n23824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23681);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 50788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ожора.
pub(crate) fn n23826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23683);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 50789);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 79474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иора.
pub(crate) fn n6388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6372);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 23684);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50790);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79475);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102843);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119172);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кора.
pub(crate) fn n6389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23685);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23686);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 50791);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79476);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102844);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119173);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23687);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 50793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50794);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лора.
pub(crate) fn n6390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6374);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23689);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23690);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 50795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 23691);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мора.
pub(crate) fn n6391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6375);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23692);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79479);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 79480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50798);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79481);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102845);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119174);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23693);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50799);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79482);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102846);
									s.is_empty()
								}
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
			super::push(out, 23694);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23695);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нора.
pub(crate) fn n6392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6376);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23696);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23697);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50801);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23698);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 50802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23699);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23700);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50804);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79484);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 102847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 50805);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пора.
pub(crate) fn n6393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6377);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23701);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23702);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50806);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79485);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102848);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119175);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129384);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23703);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 23704);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50807);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79486);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102849);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119176);
											s.is_empty()
										}
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
					super::push(out, 50808);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79487);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102850);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119177);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129385);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 50809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 23705);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рора.
pub(crate) fn n6394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6378);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23706);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50810);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сора.
pub(crate) fn n6395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6379);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 23707);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тора.
pub(crate) fn n6396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6380);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23708);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 50812);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79488);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102851);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 119178);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 129386);
													match s {
														[] => true,
														['г', s @ ..] => super::t44::n136870(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 50813);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79489);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 102852);
									s.is_empty()
								}
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
			super::push(out, 23709);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50814);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79490);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 102853);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119179);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129387);
													match s {
														[] => true,
														['г', s @ ..] => super::t44::n136871(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23710);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 23711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фора.
pub(crate) fn n6397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6381);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23712);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 50816);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 50817);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79493);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 79494);
							match s {
								[] => true,
								['м', '-', s @ ..] => {
									super::push(out, 102854);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119180);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129388);
													match s {
														[] => true,
														['м', s @ ..] => super::t44::n136872(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23713);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 50818);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79495);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 102855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 50819);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 50820);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 79497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 50821);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 79498);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102856);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 119181);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129389);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23714);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50822);
					s.is_empty()
				}
				['о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 50823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23715);
			match s {
				[] => true,
				['а', 'к', '\u{1}', s @ ..] => {
					super::push(out, 50824);
					s.is_empty()
				}
				['о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 50825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23716);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50826);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79499);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 102857);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119182);
											s.is_empty()
										}
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
					super::push(out, 50827);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79500);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 50828);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79501);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 79502);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102860);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119183);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129390);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 102861);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 119184);
											s.is_empty()
										}
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
					super::push(out, 50829);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79503);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102862);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 102863);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119185);
											s.is_empty()
										}
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
					super::push(out, 50830);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 50831);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79504);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 50832);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79505);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 102865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 50833);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79506);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 102866);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119186);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23717);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50834);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79507);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хора.
pub(crate) fn n6398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6382);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23718);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 50835);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79508);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102868);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119187);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23719);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 50836);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 50837);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79509);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102869);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119188);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129391);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23720);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50838);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79510);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102870);
									s.is_empty()
								}
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
			super::push(out, 23721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чора.
pub(crate) fn n6399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6383);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23722);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 50839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шора.
pub(crate) fn n6400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6384);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23723);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23724);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 50840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьора.
pub(crate) fn n6401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6385);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 23725);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50841);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 50842);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкилометра.
pub(crate) fn n130744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129392);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 135467);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word пол-литра.
pub(crate) fn n130745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129393);
	s.is_empty()
}

/// Words ending -ондиляртра.
pub(crate) fn n130746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129394);
	s.is_empty()
}

/// Words ending -стра.
pub(crate) fn n6416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23752);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23753);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50866);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 50867);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79531);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102879);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119195);
											s.is_empty()
										}
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
					super::push(out, 50868);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23754);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 50869);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79533);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 79534);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102881);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119196);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129395);
													match s {
														[] => true,
														['з', s @ ..] => super::t44::n136874(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23755);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 50870);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23756);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50871);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 23757);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -путра.
pub(crate) fn n23901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23758);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50873);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 79537);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 102882);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119197);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129396);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135469);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штра.
pub(crate) fn n6418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6402);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23760);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 50874);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79538);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 102883);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119198);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129397);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23761);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 50875);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтра.
pub(crate) fn n6419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6403);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23762);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 50876);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ра-обскура.
pub(crate) fn n130750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129398);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135470);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпура.
pub(crate) fn n23932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23789);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 50898);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 79554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сура.
pub(crate) fn n6431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6415);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тура.
pub(crate) fn n6432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6416);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23792);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23793);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50900);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79555);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102891);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119202);
											s.is_empty()
										}
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
					super::push(out, 50901);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 50902);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79556);
							match s {
								[] => true,
								['п', 'п', 'а', 'о', s @ ..] => {
									super::push(out, 102892);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119203);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 129399);
													match s {
														[] => true,
														['и', 'в', 'о', 'и', s @ ..] => super::t44::n136877(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 119204);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 119205);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129400);
													match s {
														[] => true,
														['ф', '\u{1}', s @ ..] => super::t44::n136878(s, out),
														_ => false,
													}
												}
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
							super::push(out, 79557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23794);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50903);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23795);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50904);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79559);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 79560);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 102893);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119206);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129401);
													match s {
														[] => true,
														['ч', s @ ..] => super::t44::n136879(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 50905);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 50906);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 79561);
							match s {
								[] => true,
								['ъ', 'д', s @ ..] => {
									super::push(out, 102894);
									match s {
										[] => true,
										['а', '\u{1}', s @ ..] => {
											super::push(out, 119207);
											s.is_empty()
										}
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
					super::push(out, 50907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23796);
			match s {
				[] => true,
				['е', 'г', s @ ..] => {
					super::push(out, 50908);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 23797);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50909);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79563);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102895);
									s.is_empty()
								}
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
			super::push(out, 23798);
			match s {
				[] => true,
				['л', 'у', 'к', 'з', s @ ..] => {
					super::push(out, 50910);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79564);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 102896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фура.
pub(crate) fn n6433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6417);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 23799);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 50911);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79565);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чура.
pub(crate) fn n6434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6418);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 23800);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50912);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102898);
									s.is_empty()
								}
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
			super::push(out, 23801);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23802);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 50914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шура.
pub(crate) fn n6435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6419);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23803);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23804);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 50915);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янтьятьяра.
pub(crate) fn n130754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129402);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 135474);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ндикатриса.
pub(crate) fn n130756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129404);
	s.is_empty()
}

/// The word трактриса.
pub(crate) fn n130757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129405);
	s.is_empty()
}

/// Words ending -трисектриса.
pub(crate) fn n130758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129406);
	s.is_empty()
}

/// Words ending -акса-вакса.
pub(crate) fn n130759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129407);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 135476);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 138978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лесополоса.
pub(crate) fn n130760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129408);
	s.is_empty()
}

/// Words ending -авиатрисса.
pub(crate) fn n130761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129409);
	s.is_empty()
}

/// Words ending -атисса.
pub(crate) fn n51533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51017);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 79630);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 102933);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтисса.
pub(crate) fn n51534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51018);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 79631);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 102934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цисса.
pub(crate) fn n24069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23923);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 51019);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79632);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осса.
pub(crate) fn n6523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6507);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23924);
			match s {
				[] => true,
				['г', 'о', s @ ..] => {
					super::push(out, 51020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23925);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 23926);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усса.
pub(crate) fn n6524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6508);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 23927);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51022);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102936);
									s.is_empty()
								}
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
			super::push(out, 23928);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51023);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 51024);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 79634);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102937);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119224);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 129410);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23929);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51025);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79635);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясса.
pub(crate) fn n6525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6509);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23930);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -л-автобуса.
pub(crate) fn n130763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129411);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 135477);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кафаревуса.
pub(crate) fn n130764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129412);
	s.is_empty()
}

/// Words ending -кула-лопата.
pub(crate) fn n136884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135478);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -военсовета.
pub(crate) fn n130768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129416);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 135479);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 138980);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рета.
pub(crate) fn n6568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6552);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24015);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51097);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 51098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 51099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 24016);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24017);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 51101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссета.
pub(crate) fn n24164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24018);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тета.
pub(crate) fn n6570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6554);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24019);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51103);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79679);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102963);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 119237);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 129417);
													match s {
														[] => true,
														['н', s @ ..] => super::t44::n136886(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уета.
pub(crate) fn n6571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6555);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 24020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 24021);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51105);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фета.
pub(crate) fn n6572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6556);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 51107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хета.
pub(crate) fn n6573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6557);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24022);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51108);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79681);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102964);
									s.is_empty()
								}
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
			super::push(out, 24023);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51109);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79682);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102965);
									s.is_empty()
								}
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
			super::push(out, 24024);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чета.
pub(crate) fn n6574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6558);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щета.
pub(crate) fn n6575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6559);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24026);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 24027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьета.
pub(crate) fn n6576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6560);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 24028);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 51113);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79683);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51114);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 51115);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трита.
pub(crate) fn n24202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24053);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 51144);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 79700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ифрита.
pub(crate) fn n51662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51145);
	s.is_empty()
}

/// Words ending -тита.
pub(crate) fn n6589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6573);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 24055);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51146);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фита.
pub(crate) fn n6590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6574);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24056);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24057);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 51147);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хита.
pub(crate) fn n6591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6575);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чита.
pub(crate) fn n6592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6576);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щита.
pub(crate) fn n6593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6577);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24060);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 51148);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юита.
pub(crate) fn n6594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6578);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 24061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ампрефекта.
pub(crate) fn n130770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129418);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 135481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -медиапунта.
pub(crate) fn n130771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129419);
	s.is_empty()
}

/// Words ending -кользота.
pub(crate) fn n104080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102990);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 119243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иота.
pub(crate) fn n6610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6594);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 24113);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 24114);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51194);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102991);
									s.is_empty()
								}
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
			super::push(out, 24115);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51195);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24116);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51196);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йота.
pub(crate) fn n6611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6595);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24117);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 51197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кота.
pub(crate) fn n6612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6596);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24118);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 51198);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 51199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51200);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79739);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102992);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119244);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24119);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 51201);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24120);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51202);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 24121);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51203);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 51204);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 24122);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51205);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24123);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51206);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79744);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 79745);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102993);
									s.is_empty()
								}
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
			super::push(out, 24124);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51207);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 24125);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51208);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лота.
pub(crate) fn n6613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6597);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 24126);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51209);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79748);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102994);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119245);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24127);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 51210);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79749);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 79750);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102996);
									s.is_empty()
								}
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
			super::push(out, 24128);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 51211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 24129);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51212);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79752);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 102997);
									s.is_empty()
								}
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
			super::push(out, 24130);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24131);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51214);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 51215);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 51216);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79755);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 51217);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 79756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102999);
									s.is_empty()
								}
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
			super::push(out, 24132);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51218);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24133);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51219);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79758);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103000);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 103001);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119246);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129420);
													match s {
														[] => true,
														['в', s @ ..] => super::t44::n136888(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24134);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51220);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мота.
pub(crate) fn n6614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6598);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24135);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51221);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 79760);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 79761);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103002);
									s.is_empty()
								}
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
			super::push(out, 24136);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51222);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51223);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24137);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51224);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24138);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51226);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24139);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51227);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 24140);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51228);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нота.
pub(crate) fn n6615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6599);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24141);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 24142);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 51229);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79768);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 103003);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119247);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 129421);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24143);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51230);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79769);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103004);
									s.is_empty()
								}
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
			super::push(out, 24144);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 24145);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24146);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51232);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79770);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103005);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119248);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 129422);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24147);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 24148);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51233);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79771);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 103006);
									s.is_empty()
								}
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
			super::push(out, 24149);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51234);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 24150);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51235);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 24151);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51236);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79774);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 103007);
									s.is_empty()
								}
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
			super::push(out, 24152);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 51237);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79775);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103008);
									s.is_empty()
								}
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
			super::push(out, 24153);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51238);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 51239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51240);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 51241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 24155);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51242);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 51243);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 51244);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пота.
pub(crate) fn n6616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6600);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24156);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24157);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51246);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24158);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51247);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 51248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24159);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51249);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24160);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51250);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 79783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рота.
pub(crate) fn n6617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6601);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24161);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 24162);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51251);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 24163);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51252);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 79785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24164);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 51253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24165);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 51255);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79787);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103009);
									s.is_empty()
								}
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
			super::push(out, 24166);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51256);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 24167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51257);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24168);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51258);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79790);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 103010);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119249);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129423);
													match s {
														[] => true,
														['п', s @ ..] => super::t44::n136889(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 51259);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 24169);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 51260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51261);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 79793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 24171);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51262);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79794);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103011);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 119250);
											s.is_empty()
										}
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
					super::push(out, 51263);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79795);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 79796);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 79797);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 103013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сота.
pub(crate) fn n6618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6602);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24172);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 24173);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51264);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51265);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79798);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103014);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119251);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 79799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24174);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51266);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79800);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 103015);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119252);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24175);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 51267);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79801);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103016);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119253);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24176);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 24177);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51269);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тота.
pub(crate) fn n6619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6603);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24178);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51270);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79803);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103017);
									s.is_empty()
								}
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
			super::push(out, 24179);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51271);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79804);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 103018);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119254);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129424);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24180);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51272);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79805);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103019);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119255);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24181);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51273);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 79806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 51274);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 79807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 51275);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79808);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 103020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 51276);
					match s {
						[] => true,
						['р', 'п', 'у', s @ ..] => {
							super::push(out, 79809);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103021);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119256);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129425);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['г', s @ ..] => {
							super::push(out, 79810);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 79811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24182);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51277);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51278);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фота.
pub(crate) fn n6620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6604);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24183);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 51279);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хота.
pub(crate) fn n6621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6605);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24184);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 24185);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 51280);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51281);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79813);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103022);
									s.is_empty()
								}
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
			super::push(out, 24186);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51282);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 51283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51284);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 51285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24187);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51286);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 51287);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79815);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103023);
									s.is_empty()
								}
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
			super::push(out, 24188);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51288);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24189);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 51289);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 51290);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 79817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 51291);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 51292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шота.
pub(crate) fn n6622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6606);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-йогурта.
pub(crate) fn n130778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129426);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 135484);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word пол-листа.
pub(crate) fn n130779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129427);
	s.is_empty()
}

/// Words ending -анджелиста.
pub(crate) fn n130780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129428);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 135485);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 138983);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риста.
pub(crate) fn n24371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24221);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51325);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 51326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тиста.
pub(crate) fn n24372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24222);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 51327);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79838);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103036);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119260);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129429);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135486);
													match s {
														[] => true,
														['т', s @ ..] => super::t44::n140419(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йста.
pub(crate) fn n6633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6617);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24224);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51329);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кста.
pub(crate) fn n6634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6618);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24225);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нста.
pub(crate) fn n6636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6620);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24226);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оста.
pub(crate) fn n6637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6621);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 24227);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24228);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51333);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79840);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 51334);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 24229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рста.
pub(crate) fn n6638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6622);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24230);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51335);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уста.
pub(crate) fn n6639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6623);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24231);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 24232);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51336);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79843);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103038);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 103039);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119261);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129430);
													match s {
														[] => true,
														['п', s @ ..] => super::t44::n136893(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 51337);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79844);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103040);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 103041);
									s.is_empty()
								}
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
			super::push(out, 24233);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51338);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 51339);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --института.
pub(crate) fn n130783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129431);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 135488);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенна-мачта.
pub(crate) fn n130784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129432);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 135489);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вэньжэньхуа.
pub(crate) fn n136896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135490);
	s.is_empty()
}

/// Words ending -ьм-катастрофа.
pub(crate) fn n140421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138986);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140996);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142170);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 142831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нескладеха.
pub(crate) fn n130789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129437);
	s.is_empty()
}

/// Words ending -распустеха.
pub(crate) fn n130790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129438);
	s.is_empty()
}

/// Words ending -ь-и-мачеха.
pub(crate) fn n130791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129439);
	s.is_empty()
}

/// Words ending -ать-мачеха.
pub(crate) fn n130792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129440);
	s.is_empty()
}

/// Words ending -икобразиха.
pub(crate) fn n130793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129441);
	s.is_empty()
}

/// Words ending -аниха.
pub(crate) fn n24576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24423);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 51484);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79933);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 51485);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79934);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 103087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51486);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 79935);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 51487);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79936);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 51488);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79937);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 103090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 51489);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103091);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119280);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 79938);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 103093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ениха.
pub(crate) fn n24577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24424);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 51490);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 51491);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иниха.
pub(crate) fn n24578(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 51492);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 79940);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103095);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119282);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 129442);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51493);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 79941);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ониха.
pub(crate) fn n24579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24425);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 51494);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 79942);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119283);
									s.is_empty()
								}
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
			super::push(out, 51495);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 79943);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 103098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 51496);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 79944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 51497);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79945);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тниха.
pub(crate) fn n24580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24426);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 51498);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79946);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103099);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 119284);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129443);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135492);
													match s {
														[] => true,
														['в', s @ ..] => super::t44::n140422(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пиха.
pub(crate) fn n6766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6747);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24427);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51499);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79947);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риха.
pub(crate) fn n6767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6748);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24428);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51500);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79948);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79949);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 79950);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103102);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119285);
											s.is_empty()
										}
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
					super::push(out, 51501);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 79951);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103103);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119286);
											s.is_empty()
										}
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
					super::push(out, 51502);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 79952);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103104);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 119287);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 129444);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 51503);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 79953);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 103105);
									s.is_empty()
								}
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
			super::push(out, 24429);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51504);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 51505);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 79955);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 103106);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119288);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24430);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51506);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 79956);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103107);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119289);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129445);
													match s {
														[] => true,
														['н', s @ ..] => super::t44::n136899(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 51507);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24431);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51508);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 51509);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79958);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103108);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 119290);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24432);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51510);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79959);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103109);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119291);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 129446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24433);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сиха.
pub(crate) fn n6768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6749);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24434);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51511);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51512);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79960);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103110);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119292);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24435);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 24436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тиха.
pub(crate) fn n6769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6750);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24437);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 51513);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79961);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103111);
									s.is_empty()
								}
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
			super::push(out, 24438);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 24439);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51515);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79962);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103112);
									s.is_empty()
								}
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
			super::push(out, 24440);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 51516);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 51517);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 51518);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 51519);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24441);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51520);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 51521);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79965);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103113);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119293);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129447);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24442);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 51522);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 79966);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 103114);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119294);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129448);
													match s {
														[] => true,
														['н', s @ ..] => super::t44::n136900(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фиха.
pub(crate) fn n6770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6751);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24443);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51523);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79967);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 103115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чиха.
pub(crate) fn n6771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6752);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24444);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51524);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 79968);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103116);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 119295);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129449);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24445);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 24446);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24447);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51525);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 79969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24448);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51526);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 51527);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79970);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103117);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119296);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 79971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103118);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119297);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129450);
													match s {
														[] => true,
														['г', s @ ..] => super::t44::n136901(s, out),
														_ => false,
													}
												}
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
							super::push(out, 79972);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 79973);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 79974);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103119);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 79975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 24449);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 24450);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 24451);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24452);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 24453);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 24454);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 51529);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79976);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 103120);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119299);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129451);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24455);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 24456);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51531);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79977);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103121);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119300);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шиха.
pub(crate) fn n6772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6753);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24457);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51532);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79978);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 103122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119301);
											s.is_empty()
										}
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
					super::push(out, 51533);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овек-эпоха.
pub(crate) fn n130805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129453);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 135496);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 138989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -артоха.
pub(crate) fn n52077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51557);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 79996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утоха.
pub(crate) fn n24643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24489);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 51558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чоха.
pub(crate) fn n6788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6769);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24490);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шоха.
pub(crate) fn n6789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6770);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24491);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жаганнатха.
pub(crate) fn n130806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129454);
	s.is_empty()
}

/// Words ending -куха.
pub(crate) fn n6806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6787);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24525);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24526);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51581);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51582);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 51583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51584);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80019);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103150);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119308);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24528);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -луха.
pub(crate) fn n6807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6788);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24529);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51586);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80020);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 103151);
									s.is_empty()
								}
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
			super::push(out, 24530);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 51588);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80022);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 103152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 51589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24531);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24532);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51590);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103153);
									s.is_empty()
								}
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
			super::push(out, 24533);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 24534);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51592);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -муха.
pub(crate) fn n6808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6789);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24535);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 24536);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51593);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 51594);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24537);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 51595);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51596);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80027);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 103154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 51597);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80028);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 103155);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119309);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24538);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51598);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нуха.
pub(crate) fn n6809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6790);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24539);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 24540);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24541);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51599);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80030);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 24542);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 51601);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80031);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 103157);
									s.is_empty()
								}
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
			super::push(out, 24543);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 24544);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24545);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51602);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103158);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 103159);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119310);
											s.is_empty()
										}
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
					super::push(out, 51603);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24546);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51604);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80033);
							match s {
								[] => true,
								['к', 'о', s @ ..] => {
									super::push(out, 103160);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 119311);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 129455);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пуха.
pub(crate) fn n6810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6791);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24547);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51605);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 51606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24548);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51607);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24549);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24550);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 80037);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 80038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 24551);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51609);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 80039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 24552);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51610);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80040);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -руха.
pub(crate) fn n6811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6792);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24553);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51611);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80041);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 51612);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 51613);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 51614);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80043);
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
				['ж', s @ ..] => {
					super::push(out, 51615);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 51616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 24554);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51617);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24555);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51618);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80046);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 51619);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80047);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 51620);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 51621);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80048);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 51622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 24556);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51623);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80049);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 51624);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 80050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24557);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51625);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80051);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 103168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 51626);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51627);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 24558);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 24559);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51628);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 51629);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 51630);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80054);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103169);
									s.is_empty()
								}
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
			super::push(out, 24560);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51631);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суха.
pub(crate) fn n6812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6793);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24561);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51632);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 51633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 51634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24562);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 51635);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80056);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103170);
									s.is_empty()
								}
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
			super::push(out, 24563);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24564);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51636);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 51637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51639);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80058);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103171);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 80059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 51640);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80060);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103172);
									s.is_empty()
								}
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
			super::push(out, 24566);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51641);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туха.
pub(crate) fn n6813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6794);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24567);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51643);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 80062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103173);
									s.is_empty()
								}
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
			super::push(out, 24568);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24569);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51644);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80063);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103174);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 119312);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129456);
													match s {
														[] => true,
														['б', s @ ..] => super::t44::n136903(s, out),
														['к', s @ ..] => super::t44::n136904(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 51645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 24570);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51646);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 80064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 24571);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24572);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51648);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80065);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103175);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119313);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129457);
													s.is_empty()
												}
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
							super::push(out, 80066);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 103176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 51649);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 51650);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 51651);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80068);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103177);
									s.is_empty()
								}
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
			super::push(out, 24573);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51652);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 80069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103178);
									s.is_empty()
								}
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
			super::push(out, 24574);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51653);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 51654);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 80070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24575);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 24576);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51656);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цуха.
pub(crate) fn n6814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6795);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24577);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24578);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24579);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51657);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 80071);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуха.
pub(crate) fn n6815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6796);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24580);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24581);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 51658);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80072);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 103180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуха.
pub(crate) fn n6816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6797);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 24582);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51659);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80073);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 103181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щуха.
pub(crate) fn n6817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6798);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 24583);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 51660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24584);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -журнашлюха.
pub(crate) fn n130810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129458);
	s.is_empty()
}

/// Words ending -ка-гаубица.
pub(crate) fn n130811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129459);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 135499);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 138992);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -белорыбица.
pub(crate) fn n130812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129460);
	s.is_empty()
}

/// Words ending -азнорыбица.
pub(crate) fn n130813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129461);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 135500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вица.
pub(crate) fn n6849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6830);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24642);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 51707);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80104);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103196);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119322);
											s.is_empty()
										}
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
					super::push(out, 51708);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80105);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103197);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119323);
											s.is_empty()
										}
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
					super::push(out, 51709);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80106);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103198);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119324);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129462);
													s.is_empty()
												}
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
							super::push(out, 80107);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103199);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 103200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', '\u{1}', s @ ..] => {
					super::push(out, 51710);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 51711);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80108);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 51712);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80109);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103202);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 80110);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103203);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119326);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 103204);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119327);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24643);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51713);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80111);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103205);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119328);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129463);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24644);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51714);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 80112);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 51715);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 51716);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 80113);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103207);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119329);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 129464);
													match s {
														[] => true,
														['т', s @ ..] => super::t44::n136907(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 51717);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80114);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 51718);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 80115);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 80116);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 51719);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 80117);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 51720);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80118);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 103212);
									s.is_empty()
								}
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
			super::push(out, 24645);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 51721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80119);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 80120);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103213);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 119330);
											s.is_empty()
										}
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
					super::push(out, 51722);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 51723);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 51724);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80121);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103214);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119331);
											match s {
												[] => true,
												['и', s @ ..] => super::t44::n130817(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t44::n24801(s, out),
		['о', s @ ..] => super::t44::n24802(s, out),
		['р', s @ ..] => super::t44::n24803(s, out),
		['т', s @ ..] => super::t44::n24804(s, out),
		['ь', s @ ..] => super::t44::n24805(s, out),
		['я', s @ ..] => super::t44::n24806(s, out),
		_ => false,
	}
}

/// Words ending -дица.
pub(crate) fn n6850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6831);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24652);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24653);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51744);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80149);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103240);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119345);
											s.is_empty()
										}
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
					super::push(out, 51745);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80150);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103241);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119346);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 129472);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24654);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 51746);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 80151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 24655);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51747);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80152);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103242);
									s.is_empty()
								}
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
			super::push(out, 24656);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51748);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24657);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80154);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 80155);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103243);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 119347);
											s.is_empty()
										}
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
					super::push(out, 51750);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80156);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103244);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119348);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 80157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 51751);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80158);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 103246);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119349);
											s.is_empty()
										}
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
					super::push(out, 51752);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 80159);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103247);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119350);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129473);
													s.is_empty()
												}
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
							super::push(out, 80160);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 103248);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119351);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 129474);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24658);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51753);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 24659);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51754);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 80162);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103249);
									s.is_empty()
								}
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
			super::push(out, 24660);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51755);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 80163);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 103250);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119352);
											s.is_empty()
										}
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
					super::push(out, 51756);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80164);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 80165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бежица.
pub(crate) fn n52281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51758);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 80166);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 103251);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 119353);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 129475);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135503);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снежица.
pub(crate) fn n81003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80167);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 103252);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ижица.
pub(crate) fn n24818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24663);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 51760);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80168);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожица.
pub(crate) fn n24820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24665);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 51761);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 51762);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ржица.
pub(crate) fn n24821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24666);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 51763);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80169);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103253);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119354);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129476);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135504);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужица.
pub(crate) fn n24822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24667);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 51764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зица.
pub(crate) fn n6853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24668);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51765);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 80170);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103254);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119355);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 129477);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24669);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51766);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 80171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103255);
									s.is_empty()
								}
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
			super::push(out, 24670);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51767);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80172);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 103256);
									s.is_empty()
								}
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
			super::push(out, 24671);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51768);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80173);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103257);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119356);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 129478);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лица.
pub(crate) fn n6854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6835);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 24672);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51769);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80174);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103258);
									s.is_empty()
								}
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
			super::push(out, 24673);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51770);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 80175);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103259);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 51771);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80176);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103260);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 103261);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119357);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129479);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 51772);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 51773);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 51774);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 80177);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103262);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 119358);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24674);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51775);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'в', '\u{1}', s @ ..] => {
					super::push(out, 51776);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 51777);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 80179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 24675);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51778);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 51779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24676);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 51781);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80181);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103263);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 119359);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 80182);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103264);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 119360);
											s.is_empty()
										}
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
					super::push(out, 51782);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 80183);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 103265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 51783);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 80184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 51784);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80185);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103266);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119361);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129480);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 51785);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80186);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 103267);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119362);
											s.is_empty()
										}
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
					super::push(out, 51786);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 80187);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 103268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 51787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80188);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 80189);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 103269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 51788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 51789);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 80191);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103270);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119363);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24677);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 51790);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24678);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 51791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 51792);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80194);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 103271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t44::n81031(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 51793);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t44::n52317(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 24679);
			s.is_empty()
		}
		['л', s @ ..] => super::t44::n24835(s, out),
		['м', s @ ..] => {
			super::push(out, 24681);
			match s {
				[] => true,
				['е', s @ ..] => super::t44::n52320(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t44::n24837(s, out),
		['п', s @ ..] => super::t44::n24838(s, out),
		['р', s @ ..] => super::t44::n24839(s, out),
		['с', s @ ..] => super::t44::n24840(s, out),
		['т', s @ ..] => super::t44::n24841(s, out),
		['у', s @ ..] => super::t44::n24842(s, out),
		['ш', s @ ..] => super::t44::n24843(s, out),
		['ы', s @ ..] => super::t44::n24844(s, out),
		['я', s @ ..] => super::t44::n24845(s, out),
		_ => false,
	}
}

/// Words ending -мица.
pub(crate) fn n6855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6836);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 24691);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51820);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103292);
									s.is_empty()
								}
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
			super::push(out, 24692);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51821);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 80233);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 51822);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80234);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 103294);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119377);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129486);
													match s {
														[] => true,
														['п', s @ ..] => super::t44::n136915(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24693);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 51823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24694);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51824);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 80235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 51825);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80236);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103295);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119378);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24695);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 24696);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51826);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ница.
pub(crate) fn n6856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6837);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24697);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 51827);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 80238);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103296);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119379);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 129487);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 51828);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 80239);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 103297);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119380);
											s.is_empty()
										}
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
					super::push(out, 51829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 51830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51831);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 80242);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 80243);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 51832);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 80244);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103299);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119381);
											s.is_empty()
										}
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
					super::push(out, 51833);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80245);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103300);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119382);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 80246);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 80247);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 51834);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80248);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103302);
									s.is_empty()
								}
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
			super::push(out, 24698);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51835);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 80249);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103303);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119383);
											s.is_empty()
										}
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
					super::push(out, 51836);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80250);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103304);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 119384);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 129488);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', 'а', s @ ..] => {
									super::push(out, 103305);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119385);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 80251);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103306);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 119386);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129489);
													s.is_empty()
												}
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
							super::push(out, 80252);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103307);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 80253);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103308);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119388);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 129490);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 51837);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80254);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 103309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119389);
											s.is_empty()
										}
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
					super::push(out, 51838);
					match s {
						[] => true,
						['л', s @ ..] => super::t44::n81091(s, out),
						['р', s @ ..] => {
							super::push(out, 80256);
							match s {
								[] => true,
								['г', s @ ..] => super::t44::n104401(s, out),
								['д', s @ ..] => super::t44::n104402(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t44::n81093(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t44::n52362(s, out),
				['у', s @ ..] => super::t44::n52363(s, out),
				['ы', s @ ..] => super::t44::n52364(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t44::n24854(s, out),
		['г', s @ ..] => super::t44::n24855(s, out),
		['д', s @ ..] => super::t44::n24856(s, out),
		['е', s @ ..] => super::t44::n24857(s, out),
		['ж', s @ ..] => super::t44::n24858(s, out),
		['з', s @ ..] => super::t44::n24859(s, out),
		['и', s @ ..] => super::t44::n24860(s, out),
		['й', s @ ..] => super::t44::n24861(s, out),
		['к', s @ ..] => super::t44::n24862(s, out),
		['л', s @ ..] => super::t44::n24863(s, out),
		['м', s @ ..] => super::t44::n24864(s, out),
		['н', s @ ..] => super::t44::n24865(s, out),
		['о', s @ ..] => super::t44::n24866(s, out),
		['п', s @ ..] => super::t44::n24867(s, out),
		['р', s @ ..] => super::t44::n24868(s, out),
		['с', s @ ..] => super::t44::n24869(s, out),
		['т', s @ ..] => super::t44::n24870(s, out),
		['у', s @ ..] => super::t44::n24871(s, out),
		['ц', s @ ..] => super::t44::n24872(s, out),
		['ч', s @ ..] => super::t45::n24873(s, out),
		['ш', s @ ..] => super::t45::n24874(s, out),
		['щ', s @ ..] => super::t45::n24875(s, out),
		['ь', s @ ..] => super::t45::n24876(s, out),
		['ю', s @ ..] => super::t45::n24877(s, out),
		['я', s @ ..] => super::t45::n24878(s, out),
		_ => false,
	}
}

/// Words ending -оица.
pub(crate) fn n6857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6838);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 24724);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 51974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 24725);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 51975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пица.
pub(crate) fn n6858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6839);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 24726);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51976);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80630);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 103819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 51977);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80631);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 103820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 80632);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103821);
									s.is_empty()
								}
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
			super::push(out, 24727);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51978);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80633);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103822);
									s.is_empty()
								}
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
			super::push(out, 24728);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51979);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24729);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 51980);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 51981);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80635);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 103823);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119815);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 129760);
													match s {
														[] => true,
														['п', s @ ..] => super::t45::n137038(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24730);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51982);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80636);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103824);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119816);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 129761);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51983);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80637);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103825);
									s.is_empty()
								}
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
			super::push(out, 24732);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51984);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 51985);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80639);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 80640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 24733);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51986);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рица.
pub(crate) fn n6859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6840);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24734);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 51987);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80642);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 103826);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119817);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129762);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 51988);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 51989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 24735);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51990);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 80644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 51991);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 80645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 51992);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 80646);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 103827);
									s.is_empty()
								}
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
			super::push(out, 24736);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51993);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 80647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 24737);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51994);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80648);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 80649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 24738);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 51995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24739);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 51996);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80650);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 103828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 51997);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80651);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103829);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 119818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 80652);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 103830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 51998);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80653);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103831);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 119819);
											s.is_empty()
										}
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
					super::push(out, 51999);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80654);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103832);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119820);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 80655);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 52000);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80656);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 103834);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119821);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129763);
													match s {
														[] => true,
														['ф', s @ ..] => super::t45::n137039(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52001);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 80657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 24740);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 24741);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 52002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 24742);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52003);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 52004);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 80659);
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
				['в', s @ ..] => {
					super::push(out, 52005);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80660);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103835);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119822);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 119823);
											s.is_empty()
										}
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
					super::push(out, 52006);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80661);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103836);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119824);
											s.is_empty()
										}
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
					super::push(out, 52007);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 52008);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80662);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103837);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119825);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 80663);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103838);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 119826);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 80664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t45::n24899(s, out),
		['у', s @ ..] => {
			super::push(out, 24745);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 52011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 24746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52012);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сица.
pub(crate) fn n6860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24747);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52013);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 80671);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103843);
									s.is_empty()
								}
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
			super::push(out, 24748);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52014);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 24749);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 52016);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80674);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 103844);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 119828);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 129765);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24750);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 52017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 52018);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 52019);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80677);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103845);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119829);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129766);
													s.is_empty()
												}
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
							super::push(out, 80678);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 103846);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 103847);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 119830);
											s.is_empty()
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
												['д', s @ ..] => {
													super::push(out, 129767);
													s.is_empty()
												}
												['к', s @ ..] => {
													super::push(out, 129768);
													match s {
														[] => true,
														['с', s @ ..] => super::t45::n137041(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 119831);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129769);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24751);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 24752);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 52020);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80679);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тица.
pub(crate) fn n6861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24753);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 52021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80680);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103849);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119832);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129770);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 52022);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80681);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 103850);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119833);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129771);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24754);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52023);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80682);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 103851);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 119834);
											s.is_empty()
										}
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
					super::push(out, 52024);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 80683);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 103852);
									s.is_empty()
								}
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
			super::push(out, 24755);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 52025);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80684);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103853);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 119835);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 129772);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 52026);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80685);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 103854);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 119836);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 129773);
													s.is_empty()
												}
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
							super::push(out, 80686);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103855);
									s.is_empty()
								}
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
			super::push(out, 24756);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 52027);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80687);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103856);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 119837);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 80688);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 103857);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119838);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 129774);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24757);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52028);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24758);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52029);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 80690);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103858);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119839);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 129775);
													match s {
														[] => true,
														['а', s @ ..] => super::t45::n137042(s, out),
														_ => false,
													}
												}
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
							super::push(out, 80691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 52030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52031);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 52032);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80693);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 103859);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 119840);
											s.is_empty()
										}
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
					super::push(out, 52033);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80694);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 52034);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80695);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103861);
									s.is_empty()
								}
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
			super::push(out, 24760);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52035);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 24761);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 24762);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52037);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80697);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 103862);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 119841);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t45::n24918(s, out),
		_ => false,
	}
}

/// Words ending -чица.
pub(crate) fn n6862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6843);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24764);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52040);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 80701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 24765);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24766);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 24767);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52042);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							super::push(out, 80703);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103865);
									s.is_empty()
								}
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
			super::push(out, 24768);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52043);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103866);
									s.is_empty()
								}
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
			super::push(out, 24769);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52044);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 80705);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103867);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 119842);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129776);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24770);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52045);
					match s {
						[] => true,
						['о', 'с', s @ ..] => {
							super::push(out, 80706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 80707);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103869);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119843);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24771);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52046);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 80708);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103870);
									s.is_empty()
								}
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
			super::push(out, 24772);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 24773);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52047);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80709);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 103871);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 119844);
											match s {
												[] => true,
												['м', 'е', 'и', 'р', 'п', '\u{1}', s @ ..] => {
													super::push(out, 129777);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 129778);
													match s {
														[] => true,
														['и', s @ ..] => super::t45::n137043(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24774);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 52049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 24775);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52050);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шица.
pub(crate) fn n6863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6844);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24776);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 52051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 24777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52052);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24778);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 52053);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 52054);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 80714);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 52055);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 52056);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 52057);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 80716);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 103875);
									s.is_empty()
								}
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
			super::push(out, 24779);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52058);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80717);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 103876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щица.
pub(crate) fn n6864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6845);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 24780);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52059);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80718);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103877);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 119845);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129779);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 24781);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52060);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80719);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103878);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119846);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129780);
													s.is_empty()
												}
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
							super::push(out, 80720);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'с', 'о', s @ ..] => {
							super::push(out, 80721);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 103880);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 103881);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119847);
											s.is_empty()
										}
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
					super::push(out, 52061);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 80722);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103882);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119848);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 129781);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 103883);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119849);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 103884);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119850);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 129782);
													match s {
														[] => true,
														['п', s @ ..] => super::t45::n137044(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52062);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80723);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 80724);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 103885);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 119851);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129783);
													match s {
														[] => true,
														['п', s @ ..] => super::t45::n137045(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t45::n52586(s, out),
				['у', s @ ..] => {
					super::push(out, 52064);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 80736);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 103904);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119872);
											s.is_empty()
										}
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
					super::push(out, 52065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24782);
			match s {
				[] => true,
				['в', s @ ..] => super::t45::n52589(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 24783);
			s.is_empty()
		}
		['м', s @ ..] => super::t45::n24939(s, out),
		['н', s @ ..] => super::t45::n24940(s, out),
		['о', s @ ..] => super::t45::n24941(s, out),
		['п', s @ ..] => super::t45::n24942(s, out),
		['р', s @ ..] => super::t45::n24943(s, out),
		['ь', s @ ..] => super::t45::n24944(s, out),
		_ => false,
	}
}

/// Words ending -ьица.
pub(crate) fn n6865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6846);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 24790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52085);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 52086);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 80767);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 103956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пол-огурца.
pub(crate) fn n131193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129841);
	s.is_empty()
}

/// Words ending -каракульча.
pub(crate) fn n131196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129844);
	s.is_empty()
}

/// Words ending -несвуменша.
pub(crate) fn n131198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129846);
	s.is_empty()
}

/// Words ending -письмоноша.
pub(crate) fn n131199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129847);
	s.is_empty()
}

/// Words ending -комиссарша.
pub(crate) fn n131200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129848);
	s.is_empty()
}

/// Words ending -ерша.
pub(crate) fn n6993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6974);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 24968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52234);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 52235);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80853);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104002);
									s.is_empty()
								}
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
			super::push(out, 24969);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52236);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80854);
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
				['а', s @ ..] => {
					super::push(out, 52237);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 80855);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 52238);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 52239);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80857);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 52240);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80858);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104005);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119947);
											s.is_empty()
										}
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
					super::push(out, 52241);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80859);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 104006);
									s.is_empty()
								}
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
			super::push(out, 24970);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52242);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 24971);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 52243);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 80861);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 104007);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 119948);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129849);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 52244);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80862);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 104008);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 52245);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 80863);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104009);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 52246);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 52247);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 80865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 52248);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 80866);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104010);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119949);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 24972);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52249);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 52250);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 80867);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104011);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119950);
											s.is_empty()
										}
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
					super::push(out, 52251);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 80868);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 80869);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 104013);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 119951);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 129850);
													match s {
														[] => true,
														['л', s @ ..] => super::t45::n137080(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 52252);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 52253);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80871);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104014);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 119952);
											s.is_empty()
										}
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
					super::push(out, 52254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 24973);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52255);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 52256);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 80873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 52257);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 80874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 52258);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 80875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 52259);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 80876);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104015);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 80877);
							match s {
								[] => true,
								['р', s @ ..] => super::t45::n105117(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t45::n52786(s, out),
				['т', s @ ..] => super::t45::n52787(s, out),
				['ф', s @ ..] => super::t45::n52788(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t45::n25133(s, out),
		['н', s @ ..] => super::t45::n25134(s, out),
		['п', s @ ..] => super::t45::n25135(s, out),
		['с', s @ ..] => super::t45::n25136(s, out),
		['т', s @ ..] => super::t45::n25137(s, out),
		['ф', s @ ..] => super::t45::n25138(s, out),
		['х', s @ ..] => super::t45::n25139(s, out),
		['ц', s @ ..] => super::t45::n25140(s, out),
		['ч', s @ ..] => super::t45::n25141(s, out),
		['ь', s @ ..] => super::t45::n25142(s, out),
		['ю', s @ ..] => super::t45::n25143(s, out),
		_ => false,
	}
}

/// Words ending -лабрадорша.
pub(crate) fn n131204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129852);
	s.is_empty()
}

/// Words ending -прокурорша.
pub(crate) fn n131205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129853);
	s.is_empty()
}

/// Words ending -кредиторша.
pub(crate) fn n131206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129854);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 135674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -форша.
pub(crate) fn n25157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24995);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52303);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80919);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 104047);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цорша.
pub(crate) fn n25158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24996);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 52304);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80920);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 104048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трша.
pub(crate) fn n6996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6977);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 24997);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 24998);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 52306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урша.
pub(crate) fn n6997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6978);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 24999);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52307);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80921);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 104049);
									s.is_empty()
								}
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
			super::push(out, 25000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырша.
pub(crate) fn n6998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6979);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 25001);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52309);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 80922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эрша.
pub(crate) fn n6999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6980);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 25002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрша.
pub(crate) fn n7000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6981);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 25003);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 52310);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярша.
pub(crate) fn n7001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6982);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 25004);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52311);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 52312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -влуша.
pub(crate) fn n25188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25025);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52337);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -милуша.
pub(crate) fn n52867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52338);
	s.is_empty()
}

/// Words ending -клуша.
pub(crate) fn n25190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25027);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52339);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олуша.
pub(crate) fn n25191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25028);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 52340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хлуша.
pub(crate) fn n25192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25029);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 52341);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 80950);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -муша.
pub(crate) fn n7011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6992);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 25030);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52342);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 80951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нуша.
pub(crate) fn n7012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6993);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 25031);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52343);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25032);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 52344);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оуша.
pub(crate) fn n7013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6994);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 25033);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 52345);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 80953);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пуша.
pub(crate) fn n7014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6995);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25034);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 52346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 25035);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52347);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 80954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25036);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 52348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -руша.
pub(crate) fn n7015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6996);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25037);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 25038);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25039);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52350);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80955);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 104061);
									s.is_empty()
								}
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
			super::push(out, 25040);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 52351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80956);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 80957);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 104062);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 119967);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суша.
pub(crate) fn n7016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6997);
	s.is_empty()
}

/// Words ending -туша.
pub(crate) fn n7017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6998);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25041);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52352);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 80958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25042);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 52353);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80959);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 104063);
									s.is_empty()
								}
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
			super::push(out, 25043);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52354);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 80960);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 104064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хуша.
pub(crate) fn n7018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6999);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25044);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52355);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуша.
pub(crate) fn n7019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7000);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -довательша.
pub(crate) fn n131207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129855);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135675);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139079);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пол-училища.
pub(crate) fn n137083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135676);
	s.is_empty()
}

/// Words ending -овгородцев.
pub(crate) fn n131215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129863);
	s.is_empty()
}

/// Words ending -новозыбков.
pub(crate) fn n131219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129867);
	s.is_empty()
}

/// Words ending -калашников.
pub(crate) fn n131220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129868);
	s.is_empty()
}

/// Words ending -нь-рыболов.
pub(crate) fn n131221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129869);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135683);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 139082);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 141046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -голов.
pub(crate) fn n25583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25414);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81136);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 81137);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 104162);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 120006);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 129870);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135684);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
				['л', s @ ..] => {
					super::push(out, 81138);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104163);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 120007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 81139);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едолов.
pub(crate) fn n25584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25415);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 52651);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 81140);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -колов.
pub(crate) fn n25585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25416);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52652);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 52653);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 81141);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 52654);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81142);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104165);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120008);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129871);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135685);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -молов.
pub(crate) fn n25586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25417);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52655);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 81143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пролов.
pub(crate) fn n25587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25418);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 52656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -солов.
pub(crate) fn n25588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25419);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 52657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -толов.
pub(crate) fn n25589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25420);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 52658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -плов.
pub(crate) fn n7369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7348);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25421);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слов.
pub(crate) fn n7370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7349);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25422);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 52659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25423);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 52660);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81145);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 52661);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 81146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 52662);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 81147);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104168);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 120009);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25424);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52663);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81148);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104169);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 120010);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129872);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 52664);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 52665);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81149);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 104170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 52666);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81150);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 104171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 52667);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 81151);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 104172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлов.
pub(crate) fn n7371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7350);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 25425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улов.
pub(crate) fn n7372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7351);
	s.is_empty()
}

/// Words ending -шлов.
pub(crate) fn n7373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7352);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 25426);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 52668);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81152);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 104173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылов.
pub(crate) fn n7374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7353);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25427);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 52669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фстроганов.
pub(crate) fn n131226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129874);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 135687);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лександров.
pub(crate) fn n131227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129875);
	s.is_empty()
}

/// Words ending -массфолловинг.
pub(crate) fn n131233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129881);
	s.is_empty()
}

/// The word релоадинг.
pub(crate) fn n131234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129882);
	s.is_empty()
}

/// Words ending -бодибилдинг.
pub(crate) fn n131235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129883);
	s.is_empty()
}

/// The word аттендинг.
pub(crate) fn n131236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129884);
	s.is_empty()
}

/// Words ending -кбординг.
pub(crate) fn n105337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104236);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 120038);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 129885);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 135691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тбординг.
pub(crate) fn n105338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104237);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 120039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 129886);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 135692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 129887);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 135693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убординг.
pub(crate) fn n105339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104238);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 120040);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 129888);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 135694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зинг.
pub(crate) fn n7571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7549);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 25663);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 52856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 25664);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52857);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81270);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 104239);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 81271);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 104240);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 120041);
											match s {
												[] => true,
												['а', 'д', '\u{1}', s @ ..] => {
													super::push(out, 129889);
													s.is_empty()
												}
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
							super::push(out, 81272);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 104241);
									s.is_empty()
								}
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
			super::push(out, 25665);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52858);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 81273);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 104242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 81274);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 104243);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 120042);
											s.is_empty()
										}
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
					super::push(out, 52859);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 81275);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 104244);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 120043);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 129890);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иинг.
pub(crate) fn n7572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7550);
	s.is_empty()
}

/// Words ending -кинг.
pub(crate) fn n7573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7551);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25666);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 25667);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 52860);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 81276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 52861);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 52862);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81277);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 104245);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 120044);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129891);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25668);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 25669);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 25670);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 52863);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 81278);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 104246);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 120045);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 129892);
													match s {
														[] => true,
														['л', s @ ..] => super::t45::n137102(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

