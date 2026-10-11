use flexion::Path;

/// Words ending -повка.
pub(crate) fn n21257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21144);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46228);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 74584);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99152);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117017);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128321);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46229);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 74586);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 74587);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 74588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 46230);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74589);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46231);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74590);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 74591);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 99156);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117018);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128322);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134970);
													s.is_empty()
												}
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
							super::push(out, 99157);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117019);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128323);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровка.
pub(crate) fn n21258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21145);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46232);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74592);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99158);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 117020);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128324);
											s.is_empty()
										}
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
					super::push(out, 74593);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99159);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117021);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128325);
											s.is_empty()
										}
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
					super::push(out, 74594);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 74595);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99160);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117022);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128326);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 134971);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 46233);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74597);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99161);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117023);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128327);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 134972);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140176(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46234);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74598);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 74599);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99163);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117025);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128328);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134973);
													match s {
														[] => true,
														['л', s @ ..] => super::t49::n140177(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46235);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 74600);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99164);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117026);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128329);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134974);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46236);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74601);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99165);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117027);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128330);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134975);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74602);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 99166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 74603);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99167);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117028);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128331);
											s.is_empty()
										}
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
					super::push(out, 74604);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99168);
							match s {
								[] => true,
								['е', 'ц', s @ ..] => {
									super::push(out, 117029);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128332);
											match s {
												[] => true,
												['а', '\u{1}', s @ ..] => {
													super::push(out, 134976);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46237);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74605);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99169);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117030);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128333);
											s.is_empty()
										}
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
					super::push(out, 74606);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99170);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117031);
									match s {
										[] => true,
										['н', s @ ..] => super::t49::n129684(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => super::t49::n100245(s, out),
						['т', s @ ..] => super::t49::n100246(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t49::n75385(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t49::n46681(s, out),
		['т', s @ ..] => super::t49::n46682(s, out),
		['у', s @ ..] => super::t49::n46683(s, out),
		_ => false,
	}
}

/// Words ending -совка.
pub(crate) fn n21259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21146);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46241);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 46242);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74616);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99182);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', 'с', s @ ..] => {
					super::push(out, 74617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 74618);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99184);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117039);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128341);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 99185);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117040);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128342);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134982);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46243);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46244);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 74621);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 74622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46245);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 74623);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99189);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117042);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128343);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134983);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 46247);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 46248);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 74625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товка.
pub(crate) fn n21260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21147);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46249);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74626);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99190);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117043);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 74627);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99191);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117044);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128344);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 134984);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46250);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74628);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99192);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117045);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128345);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134985);
													match s {
														[] => true,
														['ж', s @ ..] => super::t49::n140180(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 74629);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99193);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 117046);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128346);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 134986);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46251);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 74630);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 74631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99194);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 99195);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117047);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128347);
											s.is_empty()
										}
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
					super::push(out, 74632);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99196);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117048);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128348);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134987);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74633);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 74634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 46252);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74635);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99198);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117049);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128349);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46253);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 46254);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 99199);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117050);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128350);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 134988);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140181(s, out),
														_ => false,
													}
												}
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
							super::push(out, 99200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 74637);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 74638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46255);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 74639);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 74640);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99202);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117051);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128351);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 117052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 74641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46256);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74642);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 74643);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 99204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 74644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 46257);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74645);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99205);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 99206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t49::n75425(s, out),
				['и', s @ ..] => super::t49::n75426(s, out),
				['к', s @ ..] => super::t49::n75427(s, out),
				['л', s @ ..] => super::t49::n75428(s, out),
				['о', s @ ..] => super::t49::n75429(s, out),
				['у', s @ ..] => super::t49::n75430(s, out),
				['ы', s @ ..] => super::t49::n75431(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t49::n46701(s, out),
		['х', s @ ..] => super::t49::n46702(s, out),
		['ы', s @ ..] => super::t49::n46703(s, out),
		_ => false,
	}
}

/// Words ending -фовка.
pub(crate) fn n21261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21148);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 46261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховка.
pub(crate) fn n21262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21149);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46262);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 46263);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 46264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цовка.
pub(crate) fn n21263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21150);
	s.is_empty()
}

/// Words ending -човка.
pub(crate) fn n21264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21151);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 46265);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74662);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 99220);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шовка.
pub(crate) fn n21265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21152);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46266);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74663);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99221);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 117068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128356);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46267);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74664);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 46268);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99223);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трест-площадка.
pub(crate) fn n142322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140877);
	s.is_empty()
}

/// Words ending -сердцевидка.
pub(crate) fn n136400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134995);
	s.is_empty()
}

/// Words ending -родка.
pub(crate) fn n21316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21202);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46339);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 46340);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74735);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 99275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 74736);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 74737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46341);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 74738);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99276);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 117099);
									s.is_empty()
								}
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
			super::push(out, 46342);
			match s {
				[] => true,
				['б', 'о', s @ ..] => {
					super::push(out, 74739);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 99277);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117100);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128367);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99278);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 117101);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128368);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 99279);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117102);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128369);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 99280);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117103);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128370);
											s.is_empty()
										}
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
					super::push(out, 74740);
					match s {
						[] => true,
						['в', 'о', 'н', '\u{1}', s @ ..] => {
							super::push(out, 99281);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 117104);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128371);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 134996);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140188(s, out),
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
									super::push(out, 117105);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128372);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 99282);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 99283);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117106);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128373);
											s.is_empty()
										}
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
					super::push(out, 74741);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99284);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117107);
									s.is_empty()
								}
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
			super::push(out, 46343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ходка.
pub(crate) fn n21317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21203);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46344);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 74743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46345);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 74744);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99285);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 117108);
									s.is_empty()
								}
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
			super::push(out, 46346);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74745);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 74746);
					s.is_empty()
				}
				['р', 'о', s @ ..] => {
					super::push(out, 74747);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99287);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117109);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128374);
											s.is_empty()
										}
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
					super::push(out, 74748);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99288);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117110);
									s.is_empty()
								}
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
			super::push(out, 46347);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 46348);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плоскогрудка.
pub(crate) fn n136402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134997);
	s.is_empty()
}

/// The word ненаглядка.
pub(crate) fn n136403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134998);
	s.is_empty()
}

/// Words ending -тарообрядка.
pub(crate) fn n136404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134999);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зей-библиотека.
pub(crate) fn n140190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138755);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 140878);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142105);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гон-библиотека.
pub(crate) fn n140191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138756);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140879);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 142106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word клуб-дискотека.
pub(crate) fn n140192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138757);
	s.is_empty()
}

/// Words ending -атория-аптека.
pub(crate) fn n142325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140880);
	s.is_empty()
}

/// Words ending -ека-передвижка.
pub(crate) fn n136414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135009);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 138760);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140882);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лгоножка.
pub(crate) fn n100417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99341);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 117147);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 128394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -доножка.
pub(crate) fn n75599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74815);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 99342);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 117148);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 128395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коножка.
pub(crate) fn n75600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74816);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 99343);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 117149);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 128396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 99344);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 117150);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 128397);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135010);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 138761);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 99345);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 117151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лоножка.
pub(crate) fn n75601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74817);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 99346);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 117152);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ноножка.
pub(crate) fn n75602(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 99347);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 117153);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 128398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 99348);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 117154);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 128399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -проножка.
pub(crate) fn n100425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99349);
	s.is_empty()
}

/// Words ending -соножка.
pub(crate) fn n75604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74819);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 99350);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 117155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 99351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тоножка.
pub(crate) fn n75605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74820);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 99352);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 117156);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 128400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яножка.
pub(crate) fn n46868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46421);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 74821);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рожка.
pub(crate) fn n21374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21260);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46422);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74822);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99353);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117157);
									s.is_empty()
								}
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
					super::push(out, 74823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 74824);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 74825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 74826);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word симпатяжка.
pub(crate) fn n136416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135011);
	s.is_empty()
}

/// Words ending -ина-растяжка.
pub(crate) fn n136417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135012);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 138762);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word корнерезка.
pub(crate) fn n136420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135015);
	s.is_empty()
}

/// Words ending -лесовывозка.
pub(crate) fn n136421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135016);
	s.is_empty()
}

/// Words ending -зка-разгрузка.
pub(crate) fn n136422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135017);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 138765);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140883);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 142108);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -агол-связка.
pub(crate) fn n136423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135018);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138766);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -род-республика.
pub(crate) fn n140202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138767);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140884);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142109);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -неоевгеника.
pub(crate) fn n136425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135020);
	s.is_empty()
}

/// The word вороника.
pub(crate) fn n118477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117216);
	s.is_empty()
}

/// Words ending -троника.
pub(crate) fn n75744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74959);
	s.is_empty()
}

/// Words ending -хроника.
pub(crate) fn n75745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74960);
	s.is_empty()
}

/// Words ending -тоника.
pub(crate) fn n47007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46559);
	s.is_empty()
}

/// Words ending -рника.
pub(crate) fn n21458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21343);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46560);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 46561);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 74961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 46562);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 74962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сника.
pub(crate) fn n21459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21344);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46563);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74963);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 46564);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74964);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уника.
pub(crate) fn n21460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21345);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 46565);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 74965);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99462);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117217);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128423);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46566);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хника.
pub(crate) fn n21461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21346);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46567);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 99463);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117218);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 128424);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 135021);
													match s {
														[] => true,
														['л', s @ ..] => super::t49::n140203(s, out),
														_ => false,
													}
												}
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
							super::push(out, 99464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьника.
pub(crate) fn n21462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21347);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 46568);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74967);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 99465);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117219);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128425);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135022);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140204(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 74968);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 99466);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117220);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128426);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135023);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74969);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99467);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117221);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128427);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юника.
pub(crate) fn n21463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21348);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 46569);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яника.
pub(crate) fn n21464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21349);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 46570);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74971);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 46571);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74972);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99469);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117222);
									s.is_empty()
								}
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
			super::push(out, 46572);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 74973);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99470);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117223);
									s.is_empty()
								}
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
			super::push(out, 46573);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перквадрика.
pub(crate) fn n136429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135024);
	s.is_empty()
}

/// Words ending -олуквадрика.
pub(crate) fn n136430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135025);
	s.is_empty()
}

/// Words ending -ектроэнергетика.
pub(crate) fn n136431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135026);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стика.
pub(crate) fn n21485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21370);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46599);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75003);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117236);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128432);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135027);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140206(s, out),
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
									super::push(out, 117237);
									match s {
										[] => true,
										['и', 'р', s @ ..] => {
											super::push(out, 128433);
											s.is_empty()
										}
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
					super::push(out, 75004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99493);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 99494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46600);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					super::push(out, 75005);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 75006);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 75007);
					match s {
						[] => true,
						['а', 'л', s @ ..] => {
							super::push(out, 99497);
							s.is_empty()
						}
						['г', 'н', 'и', 'л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117238);
									s.is_empty()
								}
								['о', 'и', s @ ..] => {
									super::push(out, 117239);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 128434);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135028);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140207(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75008);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 75009);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75010);
					match s {
						[] => true,
						['а', 'н', 'и', s @ ..] => {
							super::push(out, 99498);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117240);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 99499);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 75011);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							super::push(out, 99500);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 99501);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 99502);
							s.is_empty()
						}
						['о', 'б', s @ ..] => {
							super::push(out, 99503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75012);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99504);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117242);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128435);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135029);
													match s {
														[] => true,
														['в', s @ ..] => super::t49::n140208(s, out),
														_ => false,
													}
												}
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
							super::push(out, 99505);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117243);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128436);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135030);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140209(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75013);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75014);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 75015);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46601);
			match s {
				[] => true,
				['н', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => super::t49::n100586(s, out),
						['о', s @ ..] => {
							super::push(out, 99509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t49::n47055(s, out),
		_ => false,
	}
}

/// Words ending -платье-майка.
pub(crate) fn n140216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138781);
	s.is_empty()
}

/// Words ending -лжевсезнайка.
pub(crate) fn n140217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138782);
	s.is_empty()
}

/// Words ending -риф-индейка.
pub(crate) fn n136445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135040);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 138783);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лейка.
pub(crate) fn n21515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21399);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 46667);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75078);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99551);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 117269);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128448);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 135041);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46668);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75079);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 75080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46669);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 46670);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75081);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99552);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117270);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128449);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75082);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 75083);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75084);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 75085);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 75086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 75087);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'а', '\u{1}', s @ ..] => {
			super::push(out, 46672);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46673);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75088);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							super::push(out, 99556);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117273);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128450);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135042);
													match s {
														[] => true,
														['ш', s @ ..] => super::t49::n140219(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мейка.
pub(crate) fn n21516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46675);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75089);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75090);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 46676);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75091);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46677);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 46678);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75093);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 75094);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99560);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117274);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128451);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135043);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140220(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46679);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99561);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 99562);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 99563);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117275);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128452);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46680);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75096);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пейка.
pub(crate) fn n21518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21402);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 46681);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75099);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 99567);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117277);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128453);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 135044);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n140221(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75100);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99568);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рейка.
pub(crate) fn n21519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21403);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99569);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75102);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 46682);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 46683);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 46684);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99572);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117280);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128454);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135045);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46685);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46686);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75106);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99573);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117281);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128455);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135046);
													match s {
														[] => true,
														['г', s @ ..] => super::t49::n140222(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сейка.
pub(crate) fn n21520(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75107);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75108);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99574);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117282);
									s.is_empty()
								}
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
			super::push(out, 46687);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 46688);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тейка.
pub(crate) fn n21521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21404);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75109);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99575);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75110);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46689);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46690);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75113);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99578);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117284);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128456);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фейка.
pub(crate) fn n21522(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 46691);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75114);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99579);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117285);
									s.is_empty()
								}
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
			super::push(out, 46692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чейка.
pub(crate) fn n21523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21405);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 46693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шейка.
pub(crate) fn n21524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21406);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46694);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 46695);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75115);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99580);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117286);
									s.is_empty()
								}
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
			super::push(out, 46696);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75116);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99581);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117287);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128457);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46697);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75117);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99582);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117288);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128458);
											s.is_empty()
										}
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
					super::push(out, 75118);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99583);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117289);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128459);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135047);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щейка.
pub(crate) fn n21525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21407);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 46698);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рибальдийка.
pub(crate) fn n136453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135048);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138788);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word маврикийка.
pub(crate) fn n136454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135049);
	s.is_empty()
}

/// The word гарденийка.
pub(crate) fn n136455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135050);
	s.is_empty()
}

/// Words ending -рийка.
pub(crate) fn n21537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21419);
	match s {
		[] => true,
		['а', 'в', s @ ..] => {
			super::push(out, 46721);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 46722);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75136);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99597);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117298);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128463);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135051);
													match s {
														[] => true,
														['л', s @ ..] => super::t49::n140224(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46723);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75137);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117299);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 117300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 75138);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75139);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99599);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117301);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128464);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135052);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n140225(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75140);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99600);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117302);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128465);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46724);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46725);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75142);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 75143);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99602);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75144);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99603);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75145);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99604);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117305);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128466);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 99605);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117306);
									s.is_empty()
								}
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
			super::push(out, 46726);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75146);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99606);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117307);
									s.is_empty()
								}
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
			super::push(out, 46727);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анцийка.
pub(crate) fn n75937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75149);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 99608);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 117308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стюм-двойка.
pub(crate) fn n136458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135053);
	s.is_empty()
}

/// The word выпойка.
pub(crate) fn n100701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99622);
	s.is_empty()
}

/// Words ending -ройка.
pub(crate) fn n21549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21431);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46747);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75166);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99623);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117320);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128475);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46748);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 75167);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117321);
									s.is_empty()
								}
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
			super::push(out, 46749);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75168);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99625);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117322);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128476);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135054);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46750);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 75169);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99626);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117323);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128477);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135055);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140226(s, out),
														_ => false,
													}
												}
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
							super::push(out, 99627);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 117324);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128478);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135056);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140227(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75170);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99628);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 99629);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117325);
									match s {
										[] => true,
										['о', 'н', '-', s @ ..] => {
											super::push(out, 128479);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135057);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140228(s, out),
														_ => false,
													}
												}
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
							super::push(out, 99630);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117326);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128480);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сойка.
pub(crate) fn n21550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21432);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46751);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46752);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75171);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 99631);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117327);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128481);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тойка.
pub(crate) fn n21551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21433);
	match s {
		[] => true,
		['с', 'ы', s @ ..] => {
			super::push(out, 46753);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цойка.
pub(crate) fn n21552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21434);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тра-хозяйка.
pub(crate) fn n136463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135058);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138794);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140897);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгалка.
pub(crate) fn n47258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46799);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 75213);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 99664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 99665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -далка.
pub(crate) fn n21585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21466);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46800);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75214);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 75215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75216);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99667);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117343);
									s.is_empty()
								}
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
			super::push(out, 46801);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 75217);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жалка.
pub(crate) fn n21586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21467);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46802);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -залка.
pub(crate) fn n21587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21468);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46803);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75218);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 46804);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иалка.
pub(crate) fn n21588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21469);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 46805);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 46806);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75220);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99670);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117344);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128487);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', 'н', s @ ..] => {
			super::push(out, 46807);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75221);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99671);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117345);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128488);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135059);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -калка.
pub(crate) fn n21589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21470);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46808);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 46809);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 75223);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99672);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117346);
									s.is_empty()
								}
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
			super::push(out, 46810);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46811);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75225);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99673);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 75226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99674);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117348);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128489);
											s.is_empty()
										}
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
					super::push(out, 75227);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 75228);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99675);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117349);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128490);
											s.is_empty()
										}
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
					super::push(out, 75229);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46812);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 46813);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75230);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 99677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 46814);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75231);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 75232);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99678);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117350);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128491);
											s.is_empty()
										}
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
					super::push(out, 75233);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99679);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117351);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128492);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46815);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75235);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 46816);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75236);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75237);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99682);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лалка.
pub(crate) fn n21590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21471);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -малка.
pub(crate) fn n21591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21472);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46818);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75238);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99683);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117353);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128493);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 135060);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75239);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99684);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117354);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128494);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46819);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 75240);
					match s {
						[] => true,
						['ы', 'в', 'о', 'з', 'е', 'л', 'с', '\u{1}', s @ ..] => {
							super::push(out, 99685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46820);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 46821);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75241);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 99686);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117355);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128495);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46822);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -налка.
pub(crate) fn n21592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21473);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75243);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99688);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117356);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128496);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 99689);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117357);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128497);
											s.is_empty()
										}
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
					super::push(out, 75244);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 75245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'и', 'с', s @ ..] => {
			super::push(out, 46823);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75246);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99690);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117358);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128498);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135061);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -палка.
pub(crate) fn n21593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21474);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46825);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46826);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75248);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 99692);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117359);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128499);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135062);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140230(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46827);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 46828);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ралка.
pub(crate) fn n21594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21475);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46829);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75251);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99694);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 75252);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46830);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'а', 'е', 'т', '\u{1}', s @ ..] => {
			super::push(out, 46831);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 46832);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75254);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							super::push(out, 99696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -салка.
pub(crate) fn n21595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21476);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46833);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 75255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 46834);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -талка.
pub(crate) fn n21596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21477);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46835);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75257);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 99697);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117361);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128500);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135063);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140231(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46836);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 46837);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 75258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 46838);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99698);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117362);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128501);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135064);
													s.is_empty()
												}
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
							super::push(out, 99699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46839);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46840);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					super::push(out, 75261);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99700);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117363);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128502);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135065);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46841);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75262);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 99701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 75263);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 46842);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75264);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уалка.
pub(crate) fn n21597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21478);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 46843);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 46844);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75266);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99704);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117366);
									s.is_empty()
								}
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
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99705);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 99706);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117367);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128503);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135066);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75267);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -халка.
pub(crate) fn n21598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21479);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46845);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75268);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 75269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75270);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99709);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117368);
									s.is_empty()
								}
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
			super::push(out, 46846);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 46847);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 46848);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 46849);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чалка.
pub(crate) fn n21599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21480);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46850);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75275);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 99711);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117369);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128504);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135067);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140232(s, out),
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
									super::push(out, 117370);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128505);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135068);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140233(s, out),
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
									super::push(out, 117371);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128506);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135069);
													match s {
														[] => true,
														['ш', s @ ..] => super::t49::n140234(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75276);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 75277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46851);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46852);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46853);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 75280);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 99713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117372);
									s.is_empty()
								}
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
			super::push(out, 46854);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 46855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шалка.
pub(crate) fn n21600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21481);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46856);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щалка.
pub(crate) fn n21601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21482);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46857);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75283);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75284);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46858);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -деяло-грелка.
pub(crate) fn n140236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138801);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140903);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word сексапилка.
pub(crate) fn n136480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135075);
	s.is_empty()
}

/// Words ending -криншотилка.
pub(crate) fn n136481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135076);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138802);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стилка.
pub(crate) fn n47377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46916);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75341);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 75342);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99760);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 117396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128520);
									s.is_empty()
								}
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
			super::push(out, 75343);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 99761);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 75344);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 75345);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 99762);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -филка.
pub(crate) fn n21626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21507);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 46917);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75346);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99763);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117399);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128521);
											s.is_empty()
										}
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
					super::push(out, 75347);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 75348);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 75349);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75350);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word училка.
pub(crate) fn n76144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75351);
	s.is_empty()
}

/// Words ending -шилка.
pub(crate) fn n21628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21509);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 46920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щилка.
pub(crate) fn n21629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21510);
	s.is_empty()
}

/// The word пиздоболка.
pub(crate) fn n136482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135077);
	s.is_empty()
}

/// Words ending -ье-футболка.
pub(crate) fn n136483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135078);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 138803);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 140904);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -народоволка.
pub(crate) fn n136484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135079);
	s.is_empty()
}

/// Words ending -уголка.
pub(crate) fn n47392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46931);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 75364);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 99779);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 117409);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 128529);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135080);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 138804);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n142350(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 75365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -креолка.
pub(crate) fn n76160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75366);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 99780);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -колка.
pub(crate) fn n21636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21517);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46933);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46934);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 75368);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46935);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 46936);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 46937);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75369);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 99782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 75370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75371);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99784);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117410);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128530);
											s.is_empty()
										}
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
					super::push(out, 75372);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99785);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117411);
									s.is_empty()
								}
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
			super::push(out, 46938);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 75373);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лолка.
pub(crate) fn n21637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21518);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 46939);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -молка.
pub(crate) fn n21638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21519);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46940);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46941);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75375);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99788);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75376);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75377);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99789);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 99790);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117414);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128531);
											s.is_empty()
										}
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
					super::push(out, 75378);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46942);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75379);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 75380);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117415);
									s.is_empty()
								}
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
			super::push(out, 46943);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -полка.
pub(crate) fn n21639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21520);
	s.is_empty()
}

/// Words ending -солка.
pub(crate) fn n21640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21521);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46944);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75381);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117416);
									s.is_empty()
								}
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
			super::push(out, 46945);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75382);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99794);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117417);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 128532);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -толка.
pub(crate) fn n21641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21522);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 46946);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					super::push(out, 75383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -холка.
pub(crate) fn n21642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21523);
	s.is_empty()
}

/// Words ending -сомнамбулка.
pub(crate) fn n136486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135081);
	s.is_empty()
}

/// Words ending -ка-стрелялка.
pub(crate) fn n136487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135082);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138805);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 140906);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -верхнетоемка.
pub(crate) fn n140242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138807);
	s.is_empty()
}

/// Words ending -а-невидимка.
pub(crate) fn n136490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135085);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 138808);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 140907);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вольнодумка.
pub(crate) fn n136491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135086);
	s.is_empty()
}

/// Words ending -ляска-сумка.
pub(crate) fn n136492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135087);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138809);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -совгражданка.
pub(crate) fn n136493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135088);
	s.is_empty()
}

/// The word согражданка.
pub(crate) fn n136494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135089);
	s.is_empty()
}

/// Words ending -втогражданка.
pub(crate) fn n136495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135090);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орданка.
pub(crate) fn n76276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75482);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 99857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жанка.
pub(crate) fn n21724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47058);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75483);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99858);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 117450);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128553);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135091);
													match s {
														[] => true,
														['в', s @ ..] => super::t49::n140246(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47059);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 47060);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47061);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 47062);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75485);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99860);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117451);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128554);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47063);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75487);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47064);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75488);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75489);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 75490);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99864);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117452);
									s.is_empty()
								}
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
			super::push(out, 47065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -занка.
pub(crate) fn n21725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21606);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47066);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75492);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 75494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47067);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47068);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75497);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 47069);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47070);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75499);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47071);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75500);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99871);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117453);
									s.is_empty()
								}
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
			super::push(out, 47072);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75501);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ианка.
pub(crate) fn n21726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21607);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 47073);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47074);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 47075);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75503);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99872);
							match s {
								[] => true,
								['е', 'в', '\u{1}', s @ ..] => {
									super::push(out, 117454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -канка.
pub(crate) fn n21727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21608);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47076);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 47077);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75504);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99873);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75505);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 99874);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117456);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128555);
											s.is_empty()
										}
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
					super::push(out, 75506);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99875);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117457);
									s.is_empty()
								}
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
			super::push(out, 47078);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75507);
					match s {
						[] => true,
						['е', 'м', 'а', 'о', s @ ..] => {
							super::push(out, 99876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75508);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99877);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117458);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128556);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135092);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47079);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 47080);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75509);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47081);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 47082);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 47083);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75510);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75511);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99880);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117459);
									s.is_empty()
								}
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
			super::push(out, 47084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ланка.
pub(crate) fn n21728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21609);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 47085);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75512);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99881);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 117460);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128557);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', 'г', '\u{1}', s @ ..] => {
			super::push(out, 47086);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 47087);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75513);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99882);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117461);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128558);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47088);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75514);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47089);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75515);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75516);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 75517);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -манка.
pub(crate) fn n21729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21610);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47091);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47092);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75518);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75519);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99885);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117462);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128559);
											s.is_empty()
										}
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
					super::push(out, 75520);
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					super::push(out, 75521);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 75522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 47093);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47094);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75524);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99887);
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
					super::push(out, 75525);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75526);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99888);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 99889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 75527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47095);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75528);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 99890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75529);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47096);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75530);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75531);
					s.is_empty()
				}
				['н', 'и', 'к', s @ ..] => {
					super::push(out, 75532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75533);
					s.is_empty()
				}
				['т', 'т', s @ ..] => {
					super::push(out, 75534);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99893);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 75535);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 75536);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99894);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117465);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128560);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75537);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99895);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 99896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 75538);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 75539);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75540);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 75541);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99899);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 99900);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 99901);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 99902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47098);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 47099);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75542);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99903);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75543);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99904);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117469);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128561);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135093);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47100);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75544);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117470);
									s.is_empty()
								}
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
			super::push(out, 47101);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75545);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нанка.
pub(crate) fn n21730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21611);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 47102);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75547);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -панка.
pub(crate) fn n21731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21612);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47103);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47104);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75548);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99909);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117471);
									s.is_empty()
								}
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
			super::push(out, 47105);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75550);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 75551);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранка.
pub(crate) fn n21732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21613);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47106);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75552);
					s.is_empty()
				}
				['с', 'а', s @ ..] => {
					super::push(out, 75553);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99912);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117472);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128562);
											s.is_empty()
										}
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
					super::push(out, 75554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 47107);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75555);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 75556);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99914);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75557);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99915);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117474);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128563);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 135094);
													match s {
														[] => true,
														['ь', s @ ..] => super::t49::n140247(s, out),
														_ => false,
													}
												}
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
							super::push(out, 99916);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 99917);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117475);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128564);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 135095);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47108);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75558);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 75559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47109);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75560);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 75561);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99918);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 99919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47110);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47111);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47112);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 75564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47113);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75565);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47114);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75566);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 99921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 47115);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75567);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47116);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 47117);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -санка.
pub(crate) fn n21733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21614);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47118);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47119);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47120);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -танка.
pub(crate) fn n21734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21615);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47123);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47124);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75573);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47125);
			match s {
				[] => true,
				['м', 'с', s @ ..] => {
					super::push(out, 75574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47126);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75575);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99927);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117476);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128565);
											s.is_empty()
										}
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
					super::push(out, 75576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47127);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75577);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99928);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 75578);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47128);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 47129);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75579);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ханка.
pub(crate) fn n21736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21617);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47130);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47131);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47132);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75582);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47133);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цанка.
pub(crate) fn n21737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21618);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чанка.
pub(crate) fn n21738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21619);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47135);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 47136);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47137);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75586);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 75587);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99931);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117478);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128566);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47138);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75588);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99932);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 99933);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117480);
									s.is_empty()
								}
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
			super::push(out, 47139);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47140);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47141);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47142);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 47143);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75590);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 99934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 47144);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 47145);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75591);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99935);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117481);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128567);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шанка.
pub(crate) fn n21739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21620);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47147);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75592);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75593);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47148);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47149);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 47150);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47151);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47152);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99938);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 99939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75598);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 75599);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99940);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117483);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128568);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135096);
													match s {
														[] => true,
														['ш', s @ ..] => super::t49::n140248(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75600);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 75601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', 'л', 'о', '\u{1}', s @ ..] => {
			super::push(out, 47154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щанка.
pub(crate) fn n21740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21621);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47155);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47156);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75603);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юанка.
pub(crate) fn n21741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21622);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 47157);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еденка.
pub(crate) fn n47636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47171);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 75620);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 75621);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 99956);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117496);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жденка.
pub(crate) fn n47637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47172);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75622);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 99957);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 117497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 75623);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 99958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 75624);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 99959);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изденка.
pub(crate) fn n76420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75625);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 99960);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нденка.
pub(crate) fn n47639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47174);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 75626);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 99961);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 117499);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 128576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оденка.
pub(crate) fn n47640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47175);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 75627);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 75628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 75629);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99964);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 117500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еенка.
pub(crate) fn n21747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21628);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 47176);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75630);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женка.
pub(crate) fn n21748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21629);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47177);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47178);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75631);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 75632);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99965);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47179);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75634);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 75635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 47180);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47181);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75636);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99967);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117502);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 75637);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 99968);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 117503);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128577);
											s.is_empty()
										}
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
					super::push(out, 75638);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75639);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99970);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117504);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128578);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135097);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47182);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75640);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75641);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99971);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 99972);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 99973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117507);
									s.is_empty()
								}
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
			super::push(out, 47183);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75642);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99974);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 75643);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47184);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 75644);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99976);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117509);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128579);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 135098);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47185);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75645);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99977);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 117510);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128580);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 99978);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117511);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128581);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зенка.
pub(crate) fn n21749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21630);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47186);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47187);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75647);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ленка.
pub(crate) fn n21750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21631);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47188);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75648);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 75649);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99979);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 117512);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128582);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135099);
													s.is_empty()
												}
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
							super::push(out, 99980);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117513);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128583);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135100);
													s.is_empty()
												}
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
							super::push(out, 99981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 75650);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75651);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99983);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 99984);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117514);
									s.is_empty()
								}
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
			super::push(out, 47190);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75652);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 75653);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99985);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 99986);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 99987);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117515);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128584);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135101);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n140249(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75654);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99988);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117516);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128585);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135102);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75655);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 99989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 75656);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117517);
									s.is_empty()
								}
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
			super::push(out, 47191);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75657);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99991);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117518);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128586);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135103);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47192);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47193);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75659);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99992);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 117519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 75660);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99993);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117520);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128587);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135104);
													s.is_empty()
												}
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
							super::push(out, 99994);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117521);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128588);
											s.is_empty()
										}
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
					super::push(out, 75661);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 75662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 75663);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99996);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 117522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75664);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99997);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 99998);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 99999);
							s.is_empty()
						}
						['о', s @ ..] => super::t49::n101081(s, out),
						['ы', s @ ..] => {
							super::push(out, 100001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t49::n47659(s, out),
		['о', s @ ..] => super::t49::n47660(s, out),
		['п', s @ ..] => super::t49::n47661(s, out),
		['р', s @ ..] => super::t49::n47662(s, out),
		['с', s @ ..] => super::t49::n47663(s, out),
		['т', s @ ..] => {
			super::push(out, 47199);
			match s {
				[] => true,
				['е', s @ ..] => super::t49::n76471(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t49::n47665(s, out),
		['ы', s @ ..] => super::t49::n47666(s, out),
		_ => false,
	}
}

/// Words ending -менка.
pub(crate) fn n21751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21632);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47202);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75679);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75680);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100012);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117532);
									s.is_empty()
								}
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
			super::push(out, 47203);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47204);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75682);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47205);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 75683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75684);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 100016);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117534);
									s.is_empty()
								}
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
			super::push(out, 47206);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75685);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100017);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117535);
									s.is_empty()
								}
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
			super::push(out, 47207);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75687);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100019);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117536);
									s.is_empty()
								}
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
			super::push(out, 47208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75688);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 100020);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117537);
									s.is_empty()
								}
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
			super::push(out, 47209);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100021);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 100022);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117539);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128594);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135108);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100023);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117540);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128595);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135109);
													s.is_empty()
												}
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
							super::push(out, 100024);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75689);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75690);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100025);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117542);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128596);
											s.is_empty()
										}
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
					super::push(out, 75691);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100026);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117543);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128597);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135110);
													s.is_empty()
												}
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
							super::push(out, 100027);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 117544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 75692);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100028);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117545);
									match s {
										[] => true,
										['т', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135111);
													s.is_empty()
												}
												['-', 'х', 'у', 'в', 'д', '\u{1}', s @ ..] => {
													super::push(out, 135112);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47210);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75693);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 47211);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 75694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 47212);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 75695);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47213);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75696);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100031);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117546);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128598);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47214);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75697);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ненка.
pub(crate) fn n21752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21633);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47215);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47216);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75698);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75699);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100032);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117547);
									s.is_empty()
								}
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
			super::push(out, 47217);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 75700);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 100033);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117548);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128599);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135113);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140254(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47218);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47219);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 75702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 47220);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75704);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100035);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128600);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47221);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 75705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 47222);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 47223);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75707);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100036);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117550);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128601);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оенка.
pub(crate) fn n21753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21634);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 47224);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 47225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47226);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенка.
pub(crate) fn n21754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21635);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47227);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 75710);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100037);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117551);
									s.is_empty()
								}
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
			super::push(out, 47228);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ренка.
pub(crate) fn n21755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47230);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 47231);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 47232);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75711);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 75712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75713);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100039);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117552);
									s.is_empty()
								}
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
			super::push(out, 47233);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47234);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75714);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100040);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117553);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128602);
											s.is_empty()
										}
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
					super::push(out, 75715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 47235);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75716);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100041);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117554);
									s.is_empty()
								}
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
			super::push(out, 47236);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сенка.
pub(crate) fn n21756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21637);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47237);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100042);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 100043);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117555);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 100044);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117556);
									s.is_empty()
								}
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
			super::push(out, 47238);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75719);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 75720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47239);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100045);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117557);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128603);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135114);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140255(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47240);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75723);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 75724);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100047);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117558);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128604);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенка.
pub(crate) fn n21757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21638);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47241);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75725);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100048);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 75726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47242);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75727);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75728);
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							super::push(out, 100050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117560);
									s.is_empty()
								}
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
			super::push(out, 47243);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47244);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75729);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47245);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 75731);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100053);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117561);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128605);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135115);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n140256(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47246);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ценка.
pub(crate) fn n21758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21639);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 47247);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 47248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ченка.
pub(crate) fn n21759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21640);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47249);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75733);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100055);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117562);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128606);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47250);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75734);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75735);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75736);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100057);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117563);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128607);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135116);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47251);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75738);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100059);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117564);
									s.is_empty()
								}
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
			super::push(out, 47252);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75739);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117565);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128608);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 100061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47253);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 47254);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75741);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117566);
									s.is_empty()
								}
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
			super::push(out, 47255);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенка.
pub(crate) fn n21760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21641);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47256);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75743);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 75744);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100063);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75745);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75746);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47257);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 75748);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 75749);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100066);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117568);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 128609);
											s.is_empty()
										}
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
					super::push(out, 75750);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100067);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 117569);
									s.is_empty()
								}
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
			super::push(out, 47258);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75751);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 75753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47259);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75754);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117570);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 100070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47260);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47261);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75755);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100071);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117571);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128610);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47262);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенка.
pub(crate) fn n21761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47263);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75757);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100072);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 100073);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117573);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128611);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47264);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75758);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100074);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117574);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128612);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135117);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n140257(s, out),
														_ => false,
													}
												}
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
							super::push(out, 100075);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117575);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128613);
											s.is_empty()
										}
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
					super::push(out, 75759);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100076);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 100077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47265);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47266);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75761);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100079);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117576);
									s.is_empty()
								}
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
			super::push(out, 47267);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75762);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75763);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пережабинка.
pub(crate) fn n136524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135118);
	s.is_empty()
}

/// Words ending -адиокабинка.
pub(crate) fn n136525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135119);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138823);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word выщербинка.
pub(crate) fn n136526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135120);
	s.is_empty()
}

/// Words ending -актюбинка.
pub(crate) fn n118846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117584);
	s.is_empty()
}

/// Words ending -ябинка.
pub(crate) fn n47744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47279);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 75783);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 100099);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 117585);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128620);
									s.is_empty()
								}
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
			super::push(out, 75784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -винка.
pub(crate) fn n21765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21646);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47280);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75785);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100100);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117586);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75786);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75787);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 75788);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100102);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117587);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128621);
											s.is_empty()
										}
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
					super::push(out, 75789);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100103);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117588);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128622);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47281);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47282);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75791);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100105);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117589);
									s.is_empty()
								}
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
			super::push(out, 47283);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 75792);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 75793);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100107);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117590);
									s.is_empty()
								}
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
			super::push(out, 47284);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 75794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47285);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 75795);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 75796);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100109);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117591);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128623);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135121);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75797);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 75798);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 100111);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117592);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128624);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 135122);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47286);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 75799);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100112);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117593);
									s.is_empty()
								}
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
			super::push(out, 47287);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 75800);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100113);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117594);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128625);
											s.is_empty()
										}
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
					super::push(out, 75801);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100114);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117595);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 100115);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 117596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 100116);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 100117);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 100118);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117599);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128627);
											s.is_empty()
										}
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
					super::push(out, 75802);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100119);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117600);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 100120);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 75803);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47288);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75805);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47289);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75806);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100122);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117602);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128628);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47290);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75807);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 75808);
					match s {
						[] => true,
						['р', s @ ..] => super::t49::n101205(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t49::n47756(s, out),
		['х', s @ ..] => super::t49::n47757(s, out),
		['ь', s @ ..] => super::t49::n47758(s, out),
		['я', s @ ..] => super::t49::n47759(s, out),
		_ => false,
	}
}

/// Words ending -гинка.
pub(crate) fn n21766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21647);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 47295);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75813);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 100129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75814);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -динка.
pub(crate) fn n21767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21648);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47297);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 75816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75817);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100133);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 100134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75818);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75819);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47298);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 47299);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75821);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 47300);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75822);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100138);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117606);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128629);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135123);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75823);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100139);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117607);
									s.is_empty()
								}
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
			super::push(out, 47301);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47302);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75825);
					match s {
						[] => true,
						['б', 'а', 'к', '\u{1}', s @ ..] => {
							super::push(out, 100140);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 100141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 75826);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 75827);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47303);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75828);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75829);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75830);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100145);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117608);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128630);
											s.is_empty()
										}
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
					super::push(out, 75831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47304);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 47305);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75833);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100146);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117609);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128631);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135124);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47306);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75834);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100147);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117610);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75835);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100148);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 117611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еинка.
pub(crate) fn n21768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21649);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 47307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жинка.
pub(crate) fn n21769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21650);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47308);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75836);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100149);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75837);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 47309);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47310);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75839);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47311);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 75841);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47312);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47313);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75843);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100154);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75844);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75845);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100156);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117614);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 128632);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47314);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 47315);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зинка.
pub(crate) fn n21770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21651);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47316);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 47317);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75848);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 75849);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100159);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75850);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 100160);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117616);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128633);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 100161);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117617);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128634);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47318);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75851);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100162);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 117618);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128635);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 100163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47319);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47320);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47321);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47322);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75854);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47323);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75855);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иинка.
pub(crate) fn n21771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21652);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 47324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кинка.
pub(crate) fn n21772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21653);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47325);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47326);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 75857);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 75858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 47327);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 75859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -линка.
pub(crate) fn n21773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21654);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47328);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75860);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100167);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 75861);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100168);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 75862);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 100169);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117621);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 128636);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135125);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n140259(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75863);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 75864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100170);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 100171);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117622);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128637);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 100172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 75865);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100173);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117623);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75866);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100174);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117624);
									s.is_empty()
								}
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
			super::push(out, 47329);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100175);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 100176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 47330);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 47331);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75867);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47332);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75868);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 75869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75870);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 75871);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100181);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117626);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128638);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135126);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n140260(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75872);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 75873);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100182);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117627);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128639);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47333);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47334);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75874);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100183);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 75875);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75876);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47335);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75877);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 75878);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100187);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117629);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128640);
											match s {
												[] => true,
												['ш', '-', s @ ..] => {
													super::push(out, 135127);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140261(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75879);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t49::n76678(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47336);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 75881);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t49::n47803(s, out),
		['п', s @ ..] => super::t49::n47804(s, out),
		['р', s @ ..] => super::t49::n47805(s, out),
		['с', s @ ..] => super::t49::n47806(s, out),
		['у', s @ ..] => super::t49::n47807(s, out),
		['х', s @ ..] => super::t49::n47808(s, out),
		['ы', s @ ..] => super::t49::n47809(s, out),
		_ => false,
	}
}

/// Words ending -минка.
pub(crate) fn n21774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21655);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47344);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 47345);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 47346);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 47347);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75897);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47348);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75898);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100204);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117639);
									s.is_empty()
								}
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
			super::push(out, 47349);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75899);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100205);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75900);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75901);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47350);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75902);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100207);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117641);
									s.is_empty()
								}
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
			super::push(out, 47351);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 75903);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100208);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117642);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128643);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 135129);
													match s {
														[] => true,
														['р', s @ ..] => super::t49::n140263(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47352);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 47353);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75904);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100209);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117643);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 128644);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135130);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75905);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 47354);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75906);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 47355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нинка.
pub(crate) fn n21775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21656);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47356);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75908);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 75909);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100212);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117644);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128645);
											s.is_empty()
										}
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
					super::push(out, 75910);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75911);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100213);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117645);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128646);
											s.is_empty()
										}
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
					super::push(out, 75912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47357);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 75914);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47358);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 47359);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75916);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 100215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117646);
									s.is_empty()
								}
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
			super::push(out, 47360);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47361);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					super::push(out, 75917);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100216);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117647);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128647);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47362);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 47363);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75918);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100217);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117648);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128648);
											s.is_empty()
										}
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
					super::push(out, 75919);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100218);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117649);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128649);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оинка.
pub(crate) fn n21776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21657);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 47364);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 75920);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100219);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117650);
									s.is_empty()
								}
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
			super::push(out, 47365);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 75921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 47366);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 75922);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100220);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пинка.
pub(crate) fn n21777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21658);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47367);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75923);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100221);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 117652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 100222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47368);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47369);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75926);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100223);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117653);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 128650);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47370);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75928);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 75929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100225);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 100226);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117654);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128651);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 135131);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47371);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ринка.
pub(crate) fn n21778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21659);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47372);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 75931);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100227);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 75932);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100228);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117656);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128652);
											s.is_empty()
										}
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
					super::push(out, 75933);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75934);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75935);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100231);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117657);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128653);
											s.is_empty()
										}
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
					super::push(out, 75936);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 75937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 47373);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 75938);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100234);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117658);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128654);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47374);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75939);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75940);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100235);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117659);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128655);
											s.is_empty()
										}
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
					super::push(out, 75941);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75942);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100237);
							match s {
								[] => true,
								['в', '-', 'й', s @ ..] => {
									super::push(out, 117661);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128656);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47375);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 75943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47376);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75944);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47377);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75945);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100239);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 117662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 100240);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117663);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128657);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135132);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 75946);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 75947);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100241);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128658);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47378);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 75948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47379);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					super::push(out, 75949);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100242);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117665);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128659);
											s.is_empty()
										}
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
					super::push(out, 75950);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100243);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117666);
									s.is_empty()
								}
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
			super::push(out, 47380);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -синка.
pub(crate) fn n21779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21660);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 47381);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75953);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 75954);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117667);
									s.is_empty()
								}
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
			super::push(out, 47382);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47383);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					super::push(out, 75956);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100246);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117668);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128660);
											s.is_empty()
										}
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
					super::push(out, 75957);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75959);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47384);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47385);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 75961);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47386);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 75962);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100251);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									super::push(out, 117669);
									s.is_empty()
								}
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
			super::push(out, 47387);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75963);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100253);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 100254);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117670);
									s.is_empty()
								}
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
			super::push(out, 47388);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75965);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47389);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75966);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 47390);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75967);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100257);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117671);
									s.is_empty()
								}
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
			super::push(out, 47391);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75968);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100258);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинка.
pub(crate) fn n21780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21661);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75969);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100259);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117673);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128661);
											s.is_empty()
										}
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
					super::push(out, 75970);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100260);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 117674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 100261);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 75971);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100262);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 75972);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100263);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117677);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100264);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 100265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75974);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100266);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75975);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 100267);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117679);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128662);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47393);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75976);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 75977);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100268);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117680);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128663);
											s.is_empty()
										}
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
					super::push(out, 75978);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100269);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 100270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 75979);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 75980);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100272);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117681);
									s.is_empty()
								}
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
			super::push(out, 47394);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75981);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100273);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 75982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47395);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75983);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 47396);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75984);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100275);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117683);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128664);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 100276);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117684);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128665);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 100277);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117685);
									s.is_empty()
								}
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
			super::push(out, 47397);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 75985);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100278);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117686);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 75986);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100279);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 117687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 75987);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100280);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 100281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 75988);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100282);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117689);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128666);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 117690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 100283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75989);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 100284);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 75990);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100285);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 75991);
					match s {
						[] => true,
						['и', s @ ..] => super::t49::n101367(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t49::n76790(s, out),
				['х', s @ ..] => super::t49::n76791(s, out),
				['ь', s @ ..] => super::t49::n76792(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t49::n47866(s, out),
		['с', s @ ..] => super::t49::n47867(s, out),
		['у', s @ ..] => super::t49::n47868(s, out),
		['х', s @ ..] => super::t49::n47869(s, out),
		['ш', s @ ..] => super::t49::n47870(s, out),
		['я', s @ ..] => super::t49::n47871(s, out),
		_ => false,
	}
}

/// Words ending -уинка.
pub(crate) fn n21781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21662);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 47404);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76008);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 47405);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76009);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100303);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -финка.
pub(crate) fn n21782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21663);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хинка.
pub(crate) fn n21783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21664);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 47407);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76010);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100304);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117704);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128670);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цинка.
pub(crate) fn n21784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21665);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47408);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76011);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100305);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117705);
									s.is_empty()
								}
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
			super::push(out, 47409);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 47410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чинка.
pub(crate) fn n21785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21666);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47411);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 47412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76013);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100306);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 117706);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128671);
											s.is_empty()
										}
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
					super::push(out, 76014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 47413);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76015);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117707);
									s.is_empty()
								}
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
			super::push(out, 47414);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 76017);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 100309);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117708);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128672);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135133);
													match s {
														[] => true,
														['з', s @ ..] => super::t49::n140264(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47415);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76018);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 76019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 76020);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100311);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117709);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128673);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135134);
													match s {
														[] => true,
														['ч', s @ ..] => super::t49::n140265(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76021);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47417);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76022);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47418);
			match s {
				[] => true,
				['л', 'з', s @ ..] => {
					super::push(out, 76023);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117710);
									s.is_empty()
								}
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
			super::push(out, 47419);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 47420);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76025);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шинка.
pub(crate) fn n21786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21667);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47421);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 47422);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47423);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76028);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 100316);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117712);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128674);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135135);
													match s {
														[] => true,
														['у', s @ ..] => super::t49::n140266(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'л', s @ ..] => {
			super::push(out, 47424);
			match s {
				[] => true,
				['п', 'о', s @ ..] => {
					super::push(out, 76029);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100317);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117713);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128675);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47425);
			match s {
				[] => true,
				['р', 'о', 'г', s @ ..] => {
					super::push(out, 76030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47426);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76031);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100319);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117714);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128676);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47427);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щинка.
pub(crate) fn n21787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21668);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 47428);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76033);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100320);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117715);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128677);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47429);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47430);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76035);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47431);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47432);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76036);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117716);
									s.is_empty()
								}
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
			super::push(out, 47433);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47434);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76037);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47435);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76038);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100324);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117717);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 128678);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135136);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьинка.
pub(crate) fn n21788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21669);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 47436);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76039);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ветрогонка.
pub(crate) fn n136543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135137);
	s.is_empty()
}

/// Words ending -ионка.
pub(crate) fn n21797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21678);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 47457);
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76057);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 76058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47458);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76059);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100341);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 117729);
									s.is_empty()
								}
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
			super::push(out, 47459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сконка.
pub(crate) fn n47932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47462);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 76061);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 100342);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лонка.
pub(crate) fn n21799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21680);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47463);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76062);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100343);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117730);
									s.is_empty()
								}
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
			super::push(out, 47464);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76063);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 100344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47465);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76064);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47466);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 76066);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 100347);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117731);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128684);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135138);
													match s {
														[] => true,
														['ш', s @ ..] => super::t49::n140267(s, out),
														_ => false,
													}
												}
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
							super::push(out, 100348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 76067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47467);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76068);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100349);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 117732);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128685);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135139);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140268(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 76069);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100350);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117733);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 128686);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135140);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140269(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -монка.
pub(crate) fn n21800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21681);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47468);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76070);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100351);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117734);
									s.is_empty()
								}
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
			super::push(out, 47469);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47470);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76072);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -понка.
pub(crate) fn n21801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21682);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47471);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 76073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47472);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76074);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47473);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 47474);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 47475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76075);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронка.
pub(crate) fn n21802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21683);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47476);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76076);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 76077);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 76078);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100356);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117735);
									s.is_empty()
								}
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
			super::push(out, 47477);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76079);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 76080);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100357);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 100358);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117737);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 117738);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128687);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135141);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 76081);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 76082);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 76083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сонка.
pub(crate) fn n21803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21684);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47478);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'а', 'с', '\u{1}', s @ ..] => {
			super::push(out, 47479);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47480);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76085);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47481);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонка.
pub(crate) fn n21804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21685);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 47482);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76087);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 47483);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76088);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100362);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 117739);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128688);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76089);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 76090);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47484);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47485);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76091);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47486);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47487);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76092);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47488);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76093);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 76094);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 76095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 47489);
			match s {
				[] => true,
				['е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 76096);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фонка.
pub(crate) fn n21805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21686);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47490);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76097);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47491);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76098);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 76099);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ухонка.
pub(crate) fn n21806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21687);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 47492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76100);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чонка.
pub(crate) fn n21807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21688);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47493);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100369);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117741);
									s.is_empty()
								}
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
			super::push(out, 47494);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 76102);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100370);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117742);
									s.is_empty()
								}
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
			super::push(out, 47495);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76103);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 76104);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47496);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 47497);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47498);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 76105);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100373);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 76106);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100374);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117744);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128689);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47499);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 47500);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76107);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47501);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 47502);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 47503);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76108);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 76109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 47504);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 47505);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76110);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100377);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117745);
									s.is_empty()
								}
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
			super::push(out, 47506);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76111);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шонка.
pub(crate) fn n21808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21689);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47507);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47508);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47509);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76113);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 76114);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100379);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117746);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128690);
											s.is_empty()
										}
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
					super::push(out, 76115);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100380);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117747);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128691);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьонка.
pub(crate) fn n21809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21690);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 47510);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76116);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 47511);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76117);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100382);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word финикиянка.
pub(crate) fn n136549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135143);
	s.is_empty()
}

/// Words ending -риянка.
pub(crate) fn n48022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47548);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 76151);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 100412);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 117758);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128695);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135144);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сиянка.
pub(crate) fn n48023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47549);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 76152);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 76153);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

