use flexion::Path;

/// Words ending -чишь.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6964);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6965);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 13106);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19309);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 6966);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6967);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 13107);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19310);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 6968);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 6969);
							match s {
								[] => true,
								['д', 'р', 'о', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 19311);
											s.is_empty()
										}
										['ф', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24840);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 24841);
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
										['\u{1}', s @ ..] => {
											super::push(out, 19312);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 19313);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 19314);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 24842);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 19315);
											s.is_empty()
										}
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
					super::push(out, 2474);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13109);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 13110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2475);
					match s {
						[] => true,
						['и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13111);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13112);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 13113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'у', 'б', '\u{1}', s @ ..] => {
							super::push(out, 6970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2476);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6971);
							s.is_empty()
						}
						['е', s @ ..] => super::t4::n7288(s, out),
						['л', s @ ..] => {
							super::push(out, 6973);
							match s {
								[] => true,
								['е', s @ ..] => super::t4::n13866(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t4::n7290(s, out),
						['у', s @ ..] => super::t4::n7291(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t4::n2524(s, out),
				['з', s @ ..] => super::t4::n2525(s, out),
				['л', s @ ..] => super::t4::n2526(s, out),
				['м', s @ ..] => super::t4::n2527(s, out),
				['н', s @ ..] => super::t4::n2528(s, out),
				['п', s @ ..] => super::t4::n2529(s, out),
				['р', s @ ..] => super::t4::n2530(s, out),
				['с', s @ ..] => super::t4::n2531(s, out),
				['т', s @ ..] => super::t4::n2532(s, out),
				['х', s @ ..] => super::t4::n2533(s, out),
				['ш', s @ ..] => super::t4::n2534(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t4::n486(s, out),
		['в', s @ ..] => super::t4::n487(s, out),
		['г', s @ ..] => super::t4::n488(s, out),
		['е', s @ ..] => super::t4::n489(s, out),
		['и', s @ ..] => super::t4::n490(s, out),
		['к', s @ ..] => super::t4::n491(s, out),
		['л', s @ ..] => super::t4::n492(s, out),
		['м', s @ ..] => super::t4::n493(s, out),
		['н', s @ ..] => super::t4::n494(s, out),
		['о', s @ ..] => super::t4::n495(s, out),
		['п', s @ ..] => super::t4::n496(s, out),
		['р', s @ ..] => super::t4::n497(s, out),
		['т', s @ ..] => super::t4::n498(s, out),
		['у', s @ ..] => super::t4::n499(s, out),
		['ы', s @ ..] => super::t4::n500(s, out),
		['ь', s @ ..] => super::t4::n501(s, out),
		['ю', s @ ..] => super::t4::n502(s, out),
		['я', s @ ..] => super::t4::n503(s, out),
		_ => false,
	}
}

/// Words ending -шишь.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2581);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7214);
							match s {
								[] => true,
								['ш', '\u{1}', s @ ..] => {
									super::push(out, 13378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 7215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13379);
									s.is_empty()
								}
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 19433);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 19434);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ъ', s @ ..] => {
									super::push(out, 13380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2582);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7216);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 13381);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19435);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 19436);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 7217);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 7218);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 13383);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19437);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 19438);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 504);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2583);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 7219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7220);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 7221);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7222);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7223);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2585);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7224);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 13385);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19439);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 19440);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 7225);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 7226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2586);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7227);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 13386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 7228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13387);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 13388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2587);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2588);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 7230);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 13389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7231);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 7232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13390);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13391);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 19441);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 13392);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 19442);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 7233);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 13393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2589);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 7234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13394);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 7235);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 7236);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7237);
							match s {
								[] => true,
								['к', s @ ..] => super::t4::n14159(s, out),
								['п', s @ ..] => {
									super::push(out, 13397);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t4::n20539(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t4::n7575(s, out),
						['ы', s @ ..] => super::t4::n7576(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t4::n2637(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t4::n507(s, out),
		['к', s @ ..] => super::t4::n508(s, out),
		['м', s @ ..] => super::t4::n509(s, out),
		['о', s @ ..] => super::t4::n510(s, out),
		['р', s @ ..] => super::t4::n511(s, out),
		['у', s @ ..] => super::t4::n512(s, out),
		['ч', s @ ..] => super::t4::n513(s, out),
		['ы', s @ ..] => super::t4::n514(s, out),
		['ь', s @ ..] => super::t4::n515(s, out),
		['ю', s @ ..] => super::t4::n516(s, out),
		['я', s @ ..] => super::t4::n517(s, out),
		_ => false,
	}
}

/// Words ending -щишь.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2635);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7320);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19496);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 19497);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 24890);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 19498);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 24891);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 19499);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 24892);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31040(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2636);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7321);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7322);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7323);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 7324);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7325);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13476);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 19500);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 24893);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 518);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2638);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2639);
					match s {
						[] => true,
						['е', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13477);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13478);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 19501);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 13479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13480);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13481);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 19502);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 19503);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 13482);
									s.is_empty()
								}
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
			super::push(out, 520);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 2640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7326);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7327);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7329);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7330);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 7331);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 7332);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7333);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7334);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7335);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7336);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 7337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13484);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 13485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 7338);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 2644);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 7339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2645);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7340);
							match s {
								[] => true,
								['е', s @ ..] => super::t4::n14259(s, out),
								['о', s @ ..] => super::t4::n14260(s, out),
								_ => false,
							}
						}
						['м', s @ ..] => super::t4::n7684(s, out),
						['п', s @ ..] => super::t4::n7685(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t4::n524(s, out),
		['у', s @ ..] => super::t4::n525(s, out),
		['ю', s @ ..] => super::t4::n526(s, out),
		['я', s @ ..] => super::t4::n527(s, out),
		_ => false,
	}
}

/// Words ending -шься.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2653);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 7353);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7354);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n531(s, out),
		['и', s @ ..] => super::t4::n532(s, out),
		['ы', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 2702);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15038);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 15039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 2703);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15040);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 15041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -добаешь.
pub(crate) fn n2762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2715);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7775);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зобаешь.
pub(crate) fn n2763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2716);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рбаешь.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2717);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7777);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убаешь.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					super::push(out, 7778);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7779);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15057);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', 'о', s @ ..] => {
					super::push(out, 7780);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбаешь.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2718);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7781);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7782);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 15061);
							match s {
								[] => true,
								['к', 'ш', '\u{1}', s @ ..] => {
									super::push(out, 22352);
									s.is_empty()
								}
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
			super::push(out, 2719);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбаешь.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2720);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7784);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябаешь.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 2721);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2722);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2723);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7787);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 7788);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15063);
							s.is_empty()
						}
						['ш', 'о', s @ ..] => {
							super::push(out, 15064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15065);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15066);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ваешь.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2724);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 7789);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 7790);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7791);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2725);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 7792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15069);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 15070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2726);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7793);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2727);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 7794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 2728);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n548(s, out),
		['и', s @ ..] => super::t4::n549(s, out),
		['о', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2763);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 7998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2764);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2765);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 8000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2766);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8001);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 15424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 8002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 2767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8003);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8004);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2768);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 2769);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							super::push(out, 8006);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 8007);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15425);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 8008);
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							super::push(out, 8009);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t4::n552(s, out),
		['я', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['и', 'с', 'о', 'р', s @ ..] => {
					super::push(out, 2782);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 8115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гаешь.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8116);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8117);
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 15595);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 15596);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 22850);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8119);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8120);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8121);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8122);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15597);
									s.is_empty()
								}
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
			super::push(out, 552);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8123);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8124);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 15598);
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									super::push(out, 15599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 8125);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22851);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 8126);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 8127);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 8128);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 8129);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15603);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 15604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 8130);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15606);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 15607);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 15608);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8132);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 8133);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8134);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8135);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15610);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 22852);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28137);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8136);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8137);
							match s {
								[] => true,
								['н', 'и', s @ ..] => {
									super::push(out, 15611);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 22853);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 8138);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15612);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 8139);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8141);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8142);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15613);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 8143);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 15614);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 22854);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 8144);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 15615);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15616);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 22855);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28138);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 15617);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2788);
					match s {
						[] => true,
						['с', s @ ..] => super::t4::n8507(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t4::n556(s, out),
		['и', s @ ..] => super::t4::n557(s, out),
		['м', s @ ..] => super::t4::n558(s, out),
		['н', s @ ..] => super::t4::n559(s, out),
		['о', s @ ..] => super::t4::n560(s, out),
		['р', s @ ..] => super::t4::n561(s, out),
		['у', s @ ..] => super::t4::n562(s, out),
		['ы', s @ ..] => super::t4::n563(s, out),
		['ь', s @ ..] => super::t4::n564(s, out),
		['ю', s @ ..] => super::t4::n565(s, out),
		['я', s @ ..] => super::t4::n566(s, out),
		_ => false,
	}
}

/// Words ending -даешь.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8200);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8201);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8202);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8203);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8204);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8205);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 2817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2818);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2819);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8207);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15689);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 22881);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 22882);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 28145);
													s.is_empty()
												}
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
							super::push(out, 8208);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15690);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 22883);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 8209);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15691);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 22884);
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											super::push(out, 22885);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 15692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2820);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2821);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8211);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8212);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 8213);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 8214);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 8215);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 15693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 8216);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8217);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15694);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15695);
									match s {
										[] => true,
										['п', 'с', s @ ..] => {
											super::push(out, 22886);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 28146);
													s.is_empty()
												}
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
							super::push(out, 8218);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 15696);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 22887);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 8219);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8220);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 15697);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22888);
											s.is_empty()
										}
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
					super::push(out, 2822);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15698);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 15699);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 15700);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22889);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 22890);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 15701);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 15702);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 22891);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 565);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2825);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2826);
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							super::push(out, 8221);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 15703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n571(s, out),
		['ж', s @ ..] => super::t4::n572(s, out),
		['з', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['а', s @ ..] => super::t4::n2895(s, out),
				['и', s @ ..] => {
					super::push(out, 2845);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2846);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 8255);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 8256);
							s.is_empty()
						}
						['с', s @ ..] => super::t4::n8629(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t4::n2898(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t4::n574(s, out),
		['н', s @ ..] => super::t4::n575(s, out),
		['о', s @ ..] => super::t4::n576(s, out),
		['с', s @ ..] => super::t4::n577(s, out),
		['т', s @ ..] => super::t4::n578(s, out),
		['у', s @ ..] => super::t4::n579(s, out),
		['ы', s @ ..] => super::t4::n580(s, out),
		['ю', s @ ..] => super::t4::n581(s, out),
		['я', s @ ..] => super::t4::n582(s, out),
		_ => false,
	}
}

/// Words ending -жаешь.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2873);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15754);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 15755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2874);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8297);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8298);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 15756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 15757);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 15758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2875);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2876);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8299);
							match s {
								[] => true,
								['о', 'о', s @ ..] => {
									super::push(out, 15759);
									match s {
										[] => true,
										['в', 'а', s @ ..] => {
											super::push(out, 22925);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 28154);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', 'о', s @ ..] => {
											super::push(out, 22926);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28155);
													s.is_empty()
												}
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
							super::push(out, 8300);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15760);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 15761);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22927);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 8301);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15762);
									match s {
										[] => true,
										['в', 'а', s @ ..] => {
											super::push(out, 22928);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 28156);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8302);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8303);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 581);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2878);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 8305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 15763);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 15764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 8306);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 8307);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 15767);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15768);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 8308);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15769);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 8309);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 15770);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 22931);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 15771);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 15772);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 15773);
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									super::push(out, 15774);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 15775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 8310);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22932);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22933);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28157);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2881);
					s.is_empty()
				}
				['н', s @ ..] => super::t4::n2936(s, out),
				['т', s @ ..] => {
					super::push(out, 2882);
					match s {
						[] => true,
						['р', s @ ..] => super::t4::n8692(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t4::n589(s, out),
		['н', s @ ..] => super::t4::n590(s, out),
		['о', s @ ..] => super::t4::n591(s, out),
		['у', s @ ..] => super::t5::n592(s, out),
		['ы', s @ ..] => {
			super::push(out, 590);
			s.is_empty()
		}
		['ю', s @ ..] => super::t5::n594(s, out),
		['я', s @ ..] => super::t5::n595(s, out),
		_ => false,
	}
}

/// Words ending -заешь.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2898);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8335);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8336);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 8337);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 8338);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8339);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15789);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15790);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22941);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 594);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2900);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15791);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 15792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 8341);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 8342);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 8343);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 8344);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8345);
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 8346);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8347);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 15796);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22942);
											s.is_empty()
										}
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
					super::push(out, 2901);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8348);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8349);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 15797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 8350);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 8351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2904);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 8352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 597);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8353);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8354);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 8355);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 8356);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 8357);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 8358);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 8359);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15803);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 15804);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 15805);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15806);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8361);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15807);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 15808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 8362);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8363);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8364);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15809);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 22943);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28160);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 598);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t5::n602(s, out),
		['у', s @ ..] => {
			super::push(out, 600);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2908);
					match s {
						[] => true,
						['о', 'л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 15827);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 15828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['л', 'о', 'к', 'с', s @ ..] => super::t5::n2968(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t5::n606(s, out),
		['я', s @ ..] => super::t5::n607(s, out),
		_ => false,
	}
}

/// Words ending -каешь.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2916);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 8384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2917);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 8385);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15831);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 22947);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 8386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8387);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2919);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 8389);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'к', 'а', s @ ..] => {
							super::push(out, 8390);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 2920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8391);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8392);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 8393);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8394);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8395);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15834);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2921);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8396);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8397);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 15835);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22948);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22949);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28162);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 15836);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 15837);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 22950);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 28163);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 15838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8398);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 8399);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2922);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8400);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15841);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 8401);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8402);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 2923);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 8403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8404);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 8405);
							match s {
								[] => true,
								['в', 'а', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22951);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22952);
											s.is_empty()
										}
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
					super::push(out, 2925);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8406);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 15845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8407);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15846);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 15847);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22953);
											s.is_empty()
										}
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
					super::push(out, 2926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2928);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15848);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15849);
									s.is_empty()
								}
								['о', s @ ..] => super::t5::n16726(s, out),
								_ => false,
							}
						}
						['к', s @ ..] => super::t5::n8792(s, out),
						['ч', s @ ..] => super::t5::n8793(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t5::n2989(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t5::n611(s, out),
		['и', s @ ..] => super::t5::n612(s, out),
		['й', s @ ..] => super::t5::n613(s, out),
		['л', s @ ..] => super::t5::n614(s, out),
		['м', s @ ..] => super::t5::n615(s, out),
		['н', s @ ..] => super::t5::n616(s, out),
		['о', s @ ..] => super::t5::n617(s, out),
		['р', s @ ..] => super::t5::n618(s, out),
		['с', s @ ..] => super::t5::n619(s, out),
		['т', s @ ..] => super::t5::n620(s, out),
		['у', s @ ..] => super::t5::n621(s, out),
		['ц', s @ ..] => super::t5::n622(s, out),
		['ч', s @ ..] => super::t5::n623(s, out),
		['ш', s @ ..] => super::t5::n624(s, out),
		['ы', s @ ..] => super::t5::n625(s, out),
		['ь', s @ ..] => super::t5::n626(s, out),
		['э', s @ ..] => super::t5::n627(s, out),
		['ю', s @ ..] => super::t5::n628(s, out),
		['я', s @ ..] => super::t5::n629(s, out),
		_ => false,
	}
}

/// Words ending -лаешь.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 3064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 629);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16133);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16134);
									s.is_empty()
								}
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
			super::push(out, 631);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8684);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8685);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8686);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8687);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8688);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8689);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8690);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8691);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3069);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8692);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16137);
									s.is_empty()
								}
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
			super::push(out, 632);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3070);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3071);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 3073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3075);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 8694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3076);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3077);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3078);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 8695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 637);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8696);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8697);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8698);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8699);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 8700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3080);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 8701);
							match s {
								[] => true,
								['р', 'п', 'а', s @ ..] => {
									super::push(out, 16138);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8702);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16139);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 16140);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 23075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маешь.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3081);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8705);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8706);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8708);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 8709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 641);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8710);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16141);
									s.is_empty()
								}
								['н', 'о', s @ ..] => {
									super::push(out, 16142);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23076);
											s.is_empty()
										}
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
					super::push(out, 3088);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8711);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 23077);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 28190);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 23078);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28191);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', 'о', s @ ..] => {
									super::push(out, 16143);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23079);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 8712);
							match s {
								[] => true,
								['о', 'е', s @ ..] => {
									super::push(out, 16144);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23080);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 8713);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 8714);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 8715);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8716);
							s.is_empty()
						}
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16145);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23081);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 16146);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16147);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23082);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 16148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 8717);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8718);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16149);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 23083);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 23084);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 23085);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 16150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', 'о', s @ ..] => {
							super::push(out, 8719);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 8720);
							s.is_empty()
						}
						['ы', 'в', 'о', s @ ..] => {
							super::push(out, 8721);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16152);
									s.is_empty()
								}
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
			super::push(out, 643);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3090);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8722);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8723);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8724);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8725);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3091);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16154);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16155);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23086);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16156);
									s.is_empty()
								}
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
			super::push(out, 645);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3092);
					s.is_empty()
				}
				['д', s @ ..] => super::t5::n3154(s, out),
				['ч', s @ ..] => super::t5::n3155(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['д', s @ ..] => super::t5::n3156(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наешь.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 648);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8732);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8734);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8735);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 650);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3098);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 8737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3099);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3100);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 8739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 651);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 652);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3102);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3104);
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 3105);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3106);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8741);
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							super::push(out, 8742);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16159);
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									super::push(out, 16160);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23087);
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											super::push(out, 23088);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 8743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8744);
							s.is_empty()
						}
						['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 8745);
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 8746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3108);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 8747);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16161);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16162);
									s.is_empty()
								}
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
			super::push(out, 653);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3109);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8748);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3110);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8750);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 8751);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8752);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8753);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 8754);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 8755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16163);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16164);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23089);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16165);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 16166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 8756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3112);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8757);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 23090);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 28192);
													match s {
														[] => true,
														['о', s @ ..] => super::t5::n33221(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 28193);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 23091);
											match s {
												[] => true,
												['в', s @ ..] => {
													match s {
														[] => true,
														['а', s @ ..] => super::t5::n33222(s, out),
														['о', s @ ..] => super::t5::n33223(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 28194);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3113);
					match s {
						[] => true,
						['а', s @ ..] => super::t5::n9161(s, out),
						['в', s @ ..] => super::t5::n9162(s, out),
						['о', s @ ..] => super::t5::n9163(s, out),
						['т', s @ ..] => super::t5::n9164(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => super::t5::n3175(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t5::n657(s, out),
		['о', s @ ..] => super::t5::n658(s, out),
		['р', s @ ..] => super::t5::n659(s, out),
		['т', s @ ..] => super::t5::n660(s, out),
		['у', s @ ..] => super::t5::n661(s, out),
		_ => false,
	}
}

/// Words ending -паешь.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 659);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3120);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8771);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8772);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8773);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8774);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8775);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8777);
							match s {
								[] => true,
								['ц', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23093);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23094);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23095);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 23096);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 23097);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 28195);
													s.is_empty()
												}
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
							super::push(out, 8778);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 8779);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16174);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 3123);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8782);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8783);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 3126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8784);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3127);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8786);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 16177);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23098);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 8787);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16178);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 16179);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16180);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 8788);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16181);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16182);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16183);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 16184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3128);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 8789);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8790);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 3129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8791);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3130);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8793);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 8794);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 8795);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 16186);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t5::n9200(s, out),
						['с', s @ ..] => {
							super::push(out, 8797);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => super::t5::n9202(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t5::n9203(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t5::n3194(s, out),
				['р', s @ ..] => super::t5::n3195(s, out),
				['щ', s @ ..] => super::t5::n3196(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t5::n665(s, out),
		['о', s @ ..] => super::t5::n666(s, out),
		['р', s @ ..] => super::t5::n667(s, out),
		['у', s @ ..] => super::t5::n668(s, out),
		['ы', s @ ..] => super::t5::n669(s, out),
		['ю', s @ ..] => super::t5::n670(s, out),
		['я', s @ ..] => super::t5::n671(s, out),
		_ => false,
	}
}

/// Words ending -раешь.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3153);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3154);
					match s {
						[] => true,
						['у', '\u{1}', s @ ..] => {
							super::push(out, 8861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8862);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8863);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8864);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8865);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 8866);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8867);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16237);
									s.is_empty()
								}
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
			super::push(out, 670);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8868);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8869);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 8870);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8871);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8872);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3161);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3162);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 16239);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23125);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 16240);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', 'о', s @ ..] => {
									super::push(out, 16241);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23126);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 16242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'с', s @ ..] => {
							super::push(out, 8874);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16243);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23127);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8875);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16245);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23128);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'п', s @ ..] => {
									super::push(out, 16246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', 'о', s @ ..] => {
							super::push(out, 8876);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 8877);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3163);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8879);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 16248);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 16249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3164);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 8880);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8881);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8882);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16250);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23129);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 8883);
							s.is_empty()
						}
						['ы', s @ ..] => super::t5::n9292(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t5::n3229(s, out),
				['з', s @ ..] => super::t5::n3230(s, out),
				['м', s @ ..] => super::t5::n3231(s, out),
				['п', s @ ..] => super::t5::n3232(s, out),
				['с', s @ ..] => super::t5::n3233(s, out),
				['т', s @ ..] => super::t5::n3234(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t5::n677(s, out),
		['о', s @ ..] => super::t5::n678(s, out),
		_ => false,
	}
}

/// Words ending -саешь.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3174);
					match s {
						[] => true,
						['у', 'и', s @ ..] => {
							super::push(out, 8911);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16280);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 16281);
									s.is_empty()
								}
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
			super::push(out, 678);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3179);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8912);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 16282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3180);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8913);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16283);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 16284);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23146);
											s.is_empty()
										}
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
					super::push(out, 3181);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 8914);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 16285);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8915);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 16286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8916);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8917);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8918);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8919);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16287);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23147);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 679);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 8920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3185);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3186);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16288);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16289);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 16290);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 16291);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16292);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23148);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 681);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3187);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16293);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16294);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23149);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28200);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 23150);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 16295);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16296);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16297);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23151);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 682);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8922);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8923);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8924);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8925);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16298);
									s.is_empty()
								}
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
			super::push(out, 683);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3190);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 8926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таешь.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 685);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3192);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 8927);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16299);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16300);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16301);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23152);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 8928);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16302);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16303);
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
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3194);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8929);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8930);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8931);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8932);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 8933);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8934);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8935);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 8936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3196);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3197);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16307);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 3198);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8937);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23153);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23154);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23155);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 23156);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 28201);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8938);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8939);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8940);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 8941);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16309);
									s.is_empty()
								}
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
			super::push(out, 687);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 688);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3201);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3202);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 8943);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16310);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 23157);
											s.is_empty()
										}
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
					super::push(out, 3203);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8944);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16311);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 16312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => super::t5::n9359(s, out),
						['в', s @ ..] => {
							super::push(out, 8946);
							s.is_empty()
						}
						['д', s @ ..] => super::t5::n9361(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t5::n9362(s, out),
						['з', s @ ..] => super::t5::n9363(s, out),
						['и', s @ ..] => super::t5::n9364(s, out),
						['о', s @ ..] => super::t5::n9365(s, out),
						['п', s @ ..] => super::t5::n9366(s, out),
						['с', s @ ..] => super::t5::n9367(s, out),
						['т', s @ ..] => super::t5::n9368(s, out),
						['у', s @ ..] => super::t5::n9369(s, out),
						['ы', s @ ..] => super::t5::n9370(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t5::n3268(s, out),
				['н', s @ ..] => super::t5::n3269(s, out),
				['п', s @ ..] => super::t5::n3270(s, out),
				['р', s @ ..] => super::t5::n3271(s, out),
				['ч', s @ ..] => super::t5::n3272(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n693(s, out),
		['л', s @ ..] => super::t5::n694(s, out),
		['о', s @ ..] => super::t5::n695(s, out),
		['р', s @ ..] => super::t5::n696(s, out),
		['с', s @ ..] => super::t5::n697(s, out),
		['т', s @ ..] => super::t5::n698(s, out),
		['у', s @ ..] => super::t5::n699(s, out),
		['х', s @ ..] => super::t5::n700(s, out),
		['ч', s @ ..] => super::t5::n701(s, out),
		['ш', s @ ..] => super::t5::n702(s, out),
		['ы', s @ ..] => super::t5::n703(s, out),
		['я', s @ ..] => super::t5::n704(s, out),
		_ => false,
	}
}

/// Words ending -хаешь.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 702);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3249);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 3250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9047);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23218);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23219);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 16387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3251);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3252);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3253);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9049);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9050);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3254);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3255);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9051);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16389);
									s.is_empty()
								}
								['ш', '\u{1}', s @ ..] => {
									super::push(out, 16390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9052);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16391);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16392);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 16393);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16394);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 16395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9053);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 16396);
									s.is_empty()
								}
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
			super::push(out, 704);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3257);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3258);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3259);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9056);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9057);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 9058);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 16397);
									s.is_empty()
								}
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
			super::push(out, 706);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9059);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9060);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9061);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3261);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9062);
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 9063);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9064);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9065);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 3262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9066);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9067);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9068);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9069);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16400);
									s.is_empty()
								}
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
			super::push(out, 707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3263);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3264);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9070);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 9071);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 9072);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9073);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23220);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 23221);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 23222);
											s.is_empty()
										}
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
					super::push(out, 3266);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3267);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3268);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3269);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3270);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3271);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3272);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9075);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9076);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16403);
									s.is_empty()
								}
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
			super::push(out, 708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3273);
					s.is_empty()
				}
				['а', s @ ..] => super::t5::n3338(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t5::n712(s, out),
		['с', s @ ..] => super::t5::n713(s, out),
		['у', s @ ..] => super::t5::n714(s, out),
		['ч', s @ ..] => super::t5::n715(s, out),
		['ы', s @ ..] => super::t5::n716(s, out),
		['э', s @ ..] => super::t5::n717(s, out),
		['ю', s @ ..] => super::t5::n718(s, out),
		['я', s @ ..] => super::t5::n719(s, out),
		_ => false,
	}
}

/// Words ending -цаешь.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 717);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9136);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9137);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3304);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							super::push(out, 9139);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9140);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3306);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 9142);
							match s {
								[] => true,
								['с', 'о', 'в', 'а', s @ ..] => {
									super::push(out, 16463);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23238);
											s.is_empty()
										}
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
					super::push(out, 3307);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9143);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 719);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 3308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 720);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 721);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3310);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9145);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23239);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23240);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 9146);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16465);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16466);
									s.is_empty()
								}
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
			super::push(out, 722);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3312);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 9147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['б', s @ ..] => {
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
										['з', s @ ..] => {
											super::push(out, 23241);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16469);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23242);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 23243);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28221);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чаешь.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 724);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9148);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9149);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9150);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9151);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9152);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3315);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9153);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23244);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 28222);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 23245);
											s.is_empty()
										}
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
					super::push(out, 3316);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3317);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 9155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9156);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9157);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9158);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16471);
									s.is_empty()
								}
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
			super::push(out, 725);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3319);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 16472);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23246);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16473);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23247);
											s.is_empty()
										}
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
					super::push(out, 3320);
					match s {
						[] => true,
						['м', 'о', s @ ..] => {
							super::push(out, 9159);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16474);
									s.is_empty()
								}
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
			super::push(out, 726);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9160);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9161);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 9162);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23248);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23249);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28223);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3322);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9163);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3323);
					match s {
						[] => true,
						['т', 'с', 'в', 'о', s @ ..] => {
							super::push(out, 9164);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n730(s, out),
		['л', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3328);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3329);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16565);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 16566);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16567);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 16568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3330);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9197);
							s.is_empty()
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16569);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 16570);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23433);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16571);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23434);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 16572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16573);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23435);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16574);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23436);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'и', '\u{1}', s @ ..] => {
									super::push(out, 16575);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 16576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t5::n734(s, out),
		['п', s @ ..] => {
			super::push(out, 732);
			match s {
				[] => true,
				['е', s @ ..] => super::t5::n3397(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t5::n736(s, out),
		['у', s @ ..] => super::t5::n737(s, out),
		['ь', s @ ..] => super::t5::n738(s, out),
		['ю', s @ ..] => super::t5::n739(s, out),
		['я', s @ ..] => super::t5::n740(s, out),
		_ => false,
	}
}

/// Words ending -шаешь.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3350);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3351);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16595);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 16596);
									match s {
										[] => true,
										['р', 'п', 'а', s @ ..] => {
											super::push(out, 23447);
											s.is_empty()
										}
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
					super::push(out, 3352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9208);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9209);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9210);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 9211);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9212);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9214);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9215);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9216);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9217);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9218);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3356);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3357);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9220);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16599);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16600);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 9221);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 9222);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 9223);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9224);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3358);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9226);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 742);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3359);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3360);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9228);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 743);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3362);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9230);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'с', 'о', s @ ..] => {
					super::push(out, 3363);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3364);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3367);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 9232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3368);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16603);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 16604);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 16605);
									s.is_empty()
								}
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
			super::push(out, 747);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3369);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3370);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9233);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9234);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 9235);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 9236);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 9237);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9238);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9239);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9240);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9241);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16607);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 16608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							super::push(out, 9242);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9243);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3372);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9244);
							s.is_empty()
						}
						['с', s @ ..] => super::t5::n9680(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t5::n3437(s, out),
				['п', s @ ..] => {
					super::push(out, 3374);
					s.is_empty()
				}
				['р', s @ ..] => super::t5::n3439(s, out),
				['с', s @ ..] => super::t5::n3440(s, out),
				['т', s @ ..] => super::t5::n3441(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t5::n751(s, out),
		['ы', s @ ..] => super::t5::n752(s, out),
		['ь', s @ ..] => super::t5::n753(s, out),
		_ => false,
	}
}

/// Words ending -щаешь.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3380);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 3381);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3382);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9250);
							s.is_empty()
						}
						['в', s @ ..] => {
							match s {
								[] => true,
								['з', 'о', 'в', 'о', s @ ..] => {
									super::push(out, 16617);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23450);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'п', s @ ..] => {
									super::push(out, 16618);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23451);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 16619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 9251);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 9252);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 9253);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23452);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23453);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 23454);
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 23455);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23456);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 752);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3383);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16620);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16621);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16622);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23457);
											s.is_empty()
										}
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
					super::push(out, 3384);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9254);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 9255);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16623);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 9256);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 9257);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16624);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9258);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16626);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23458);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23459);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 16627);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23460);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 16628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9259);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3385);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3386);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9261);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 9262);
							s.is_empty()
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
				['н', s @ ..] => {
					super::push(out, 3387);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9263);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 9264);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 9265);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3388);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3390);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3391);
					s.is_empty()
				}
				['р', 'п', 'у', s @ ..] => {
					super::push(out, 3392);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9267);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9268);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9269);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9270);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['п', 'а', s @ ..] => {
					super::push(out, 3394);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 755);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 756);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3395);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3396);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идешь.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 792);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3439);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йдешь.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 795);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удешь.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3445);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3446);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3447);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3448);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыдешь.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдешь.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9322);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9323);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядешь.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3451);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9325);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9326);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9327);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9328);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9329);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16646);
									s.is_empty()
								}
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
			super::push(out, 805);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 3453);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -веешь.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 810);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 811);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3460);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3461);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16651);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16652);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 16653);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23463);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 16654);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23464);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16655);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23465);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 23466);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 16656);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28420);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 28421);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3462);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3463);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3464);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3465);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16657);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16658);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23467);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9343);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 3466);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9345);
							match s {
								[] => true,
								['у', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23468);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23469);
											s.is_empty()
										}
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
					super::push(out, 3467);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9346);
							match s {
								[] => true,
								['е', 'ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23470);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23471);
											s.is_empty()
										}
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
					super::push(out, 3468);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9347);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23472);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23473);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 812);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3471);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9348);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16659);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23474);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23475);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 16660);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 23476);
											s.is_empty()
										}
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
					super::push(out, 3472);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3473);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9349);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t5::n3545(s, out),
				['р', s @ ..] => {
					super::push(out, 3475);
					match s {
						[] => true,
						['е', s @ ..] => super::t5::n9796(s, out),
						['о', s @ ..] => {
							super::push(out, 9356);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => super::t5::n3547(s, out),
				['ш', s @ ..] => super::t5::n3548(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t5::n826(s, out),
		['и', s @ ..] => super::t5::n827(s, out),
		['л', s @ ..] => super::t5::n828(s, out),
		['о', s @ ..] => super::t5::n829(s, out),
		['с', s @ ..] => super::t5::n830(s, out),
		['т', s @ ..] => super::t5::n831(s, out),
		['ы', s @ ..] => super::t5::n832(s, out),
		['я', s @ ..] => super::t5::n833(s, out),
		_ => false,
	}
}

/// Words ending -седеешь.
pub(crate) fn n3584(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9401);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жидеешь.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3511);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лдеешь.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 827);
	match s {
		[] => true,
		['а', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9403);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндеешь.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 828);
	s.is_empty()
}

/// Words ending -одеешь.
pub(crate) fn n842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 829);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3513);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9405);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3514);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9406);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16731);
							s.is_empty()
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23504);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 16732);
							s.is_empty()
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23506);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23507);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9407);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 16733);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23509);
									s.is_empty()
								}
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
			super::push(out, 3515);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 9408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдеешь.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 830);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3517);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3518);
			s.is_empty()
		}
		['е', 'в', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9409);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9410);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удеешь.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 831);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 3520);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16734);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 16735);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3521);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 3522);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9412);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9413);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9414);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдеешь.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 832);
	s.is_empty()
}

/// Words ending -юдеешь.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 833);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3523);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9415);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23511);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жеешь.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3524);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16737);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16738);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3525);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3526);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3527);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 9418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3528);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3529);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3530);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9422);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9424);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9426);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9427);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9429);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зеешь.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['а', 'л', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3531);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9431);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9433);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 840);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9435);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9437);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -леешь.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3533);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3534);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 3535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9439);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16740);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 16741);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23513);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23514);
											s.is_empty()
										}
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
					super::push(out, 3536);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3537);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9440);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3539);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3540);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9442);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 9443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9444);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'с', '\u{1}', s @ ..] => {
							super::push(out, 9445);
							s.is_empty()
						}
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16743);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			super::push(out, 845);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9446);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9447);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9448);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16745);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23515);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 9449);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9450);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3544);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 9451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 3545);
					match s {
						[] => true,
						['я', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16747);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9453);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9454);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16749);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23516);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9455);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9456);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16750);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16752);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16753);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3548);
					match s {
						[] => true,
						['е', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16755);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16756);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23517);
											s.is_empty()
										}
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
					super::push(out, 3549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', '\u{1}', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9457);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3551);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9459);
							match s {
								[] => true,
								['е', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23518);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23519);
											s.is_empty()
										}
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
					super::push(out, 3552);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16757);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t5::n3639(s, out),
				_ => false,
			}
		}
		['к', 'е', 'т', 'с', '\u{1}', s @ ..] => {
			super::push(out, 849);
			s.is_empty()
		}
		['м', s @ ..] => super::t5::n866(s, out),
		['о', s @ ..] => super::t5::n867(s, out),
		['п', s @ ..] => super::t5::n868(s, out),
		['с', s @ ..] => super::t5::n869(s, out),
		['т', s @ ..] => super::t5::n870(s, out),
		['х', s @ ..] => super::t5::n871(s, out),
		['ш', 'о', 'п', '\u{1}', s @ ..] => {
			super::push(out, 856);
			s.is_empty()
		}
		['ы', s @ ..] => super::t5::n873(s, out),
		_ => false,
	}
}

/// Words ending -меешь.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 3573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9479);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9481);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9482);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3576);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3578);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3579);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16767);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16768);
									s.is_empty()
								}
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
			super::push(out, 862);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3580);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3581);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3584);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3585);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9485);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 9486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3586);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3587);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3588);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3589);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9488);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23522);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23523);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 23524);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 23525);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 28432);
													s.is_empty()
												}
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
							super::push(out, 9489);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 16770);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23526);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23527);
											s.is_empty()
										}
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
					super::push(out, 3590);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 9490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3591);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9491);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 3593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3594);
					match s {
						[] => true,
						['г', 'у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16771);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16772);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -неешь.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3596);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 9494);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3597);
					match s {
						[] => true,
						['а', 'с', 'о', s @ ..] => {
							super::push(out, 9495);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3598);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 9497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16775);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 16776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3599);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16777);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16778);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16780);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16781);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23528);
											s.is_empty()
										}
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
					super::push(out, 3600);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9499);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16782);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 16783);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 9500);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'л', 'о', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9501);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9503);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9505);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3601);
					match s {
						[] => true,
						['л', 'о', 'т', 'с', 'о', s @ ..] => {
							super::push(out, 9507);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['д', 'а', s @ ..] => {
									super::push(out, 16786);
									s.is_empty()
								}
								['л', 'а', s @ ..] => {
									super::push(out, 16787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3602);
					match s {
						[] => true,
						['а', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16788);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16790);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16791);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16793);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16794);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'е', 'т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16796);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'с', 'и', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16798);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 16799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t5::n3694(s, out),
				['л', s @ ..] => super::t5::n3695(s, out),
				['м', s @ ..] => super::t5::n3696(s, out),
				['п', 'е', s @ ..] => super::t5::n3697(s, out),
				['р', s @ ..] => super::t5::n3698(s, out),
				['т', s @ ..] => super::t5::n3699(s, out),
				['ф', 'р', 'о', 'т', s @ ..] => super::t5::n3700(s, out),
				['ч', 'о', 'к', s @ ..] => super::t5::n3701(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t5::n885(s, out),
		['з', s @ ..] => super::t5::n886(s, out),
		['и', s @ ..] => super::t5::n887(s, out),
		['й', 'о', 'р', 'т', 'с', s @ ..] => super::t5::n888(s, out),
		['к', 'с', 'у', 'т', s @ ..] => super::t5::n889(s, out),
		['л', 'о', 'п', s @ ..] => super::t5::n890(s, out),
		['м', s @ ..] => super::t5::n891(s, out),
		['н', 'и', 'л', 'д', s @ ..] => super::t5::n892(s, out),
		['о', s @ ..] => super::t5::n893(s, out),
		['п', 'у', 'р', 'к', s @ ..] => super::t5::n894(s, out),
		['р', s @ ..] => super::t5::n895(s, out),
		['с', s @ ..] => super::t5::n896(s, out),
		['т', s @ ..] => super::t5::n897(s, out),
		['у', s @ ..] => super::t5::n898(s, out),
		['ч', s @ ..] => super::t5::n899(s, out),
		['ш', s @ ..] => super::t5::n900(s, out),
		['щ', s @ ..] => super::t5::n901(s, out),
		['ь', s @ ..] => super::t5::n902(s, out),
		['ю', s @ ..] => super::t5::n903(s, out),
		['я', s @ ..] => super::t5::n904(s, out),
		_ => false,
	}
}

/// Words ending -пеешь.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['е', 'р', 'и', 'в', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3654);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', 'о', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3656);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 881);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3658);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 3660);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3661);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['л', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9559);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реешь.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 883);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 884);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 3662);
					match s {
						[] => true,
						['о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16857);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9563);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9564);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9565);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9566);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16859);
									s.is_empty()
								}
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
			super::push(out, 885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3663);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3664);
					match s {
						[] => true,
						['р', 'х', '\u{1}', s @ ..] => {
							super::push(out, 9567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3665);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16860);
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 16861);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3666);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', '\u{1}', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3667);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3668);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 9571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['у', 'м', 'о', s @ ..] => {
					super::push(out, 3670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['в', 'з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9572);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3671);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9574);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9575);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3672);
					match s {
						[] => true,
						['а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 9577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3673);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3674);
					match s {
						[] => true,
						['е', 'в', 'с', s @ ..] => {
							super::push(out, 9578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3675);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3677);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3678);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 891);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9580);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3679);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 3680);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'о', 'м', '\u{1}', s @ ..] => {
			super::push(out, 892);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 893);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3681);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9583);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 16864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3682);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3683);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3684);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9584);
							s.is_empty()
						}
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16865);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 16866);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16867);
									s.is_empty()
								}
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
			super::push(out, 895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3686);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3687);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3688);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9586);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16868);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 16869);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16870);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23546);
											s.is_empty()
										}
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
					super::push(out, 3689);
					match s {
						[] => true,
						['х', 'о', s @ ..] => {
							super::push(out, 9587);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3690);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9588);
							s.is_empty()
						}
						['е', 'п', 'а', s @ ..] => {
							super::push(out, 9589);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9590);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 897);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9592);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t5::n3801(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t5::n924(s, out),
		_ => false,
	}
}

/// Words ending -сеешь.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 899);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 900);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3691);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3692);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3694);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9602);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 16873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 904);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3696);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9603);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3697);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9605);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 9606);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теешь.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 907);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3699);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 16875);
									s.is_empty()
								}
								['ч', 'о', s @ ..] => {
									super::push(out, 16876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3701);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9609);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 16877);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23547);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 23548);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 28438);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 23549);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 28439);
													match s {
														[] => true,
														['о', s @ ..] => super::t5::n33384(s, out),
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
									super::push(out, 16878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['о', 'р', 'о', 'б', 'о', s @ ..] => {
							super::push(out, 9610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3703);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3704);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 9611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3705);
					match s {
						[] => true,
						['с', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16879);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 9612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3706);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9613);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 16881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 9614);
							match s {
								[] => true,
								['о', 'м', 'б', s @ ..] => {
									super::push(out, 16882);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23550);
											s.is_empty()
										}
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
					super::push(out, 3707);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3708);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9616);
							match s {
								[] => true,
								['л', 'о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23551);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23552);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 23553);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3709);
					match s {
						[] => true,
						['ю', 'р', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16883);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16884);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16885);
									s.is_empty()
								}
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
			super::push(out, 908);
			match s {
				[] => true,
				['е', 'ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9618);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9619);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9620);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 9621);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9622);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 9623);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3711);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9624);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 16889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 3712);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9625);
							s.is_empty()
						}
						['я', 'т', 'о', s @ ..] => {
							super::push(out, 9626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'о', 'з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9628);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 3714);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9630);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9631);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9632);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9633);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9634);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t5::n3829(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t5::n936(s, out),
		['р', s @ ..] => super::t5::n937(s, out),
		['с', s @ ..] => super::t5::n938(s, out),
		['у', s @ ..] => super::t5::n939(s, out),
		['ы', s @ ..] => super::t5::n940(s, out),
		['ю', s @ ..] => super::t5::n941(s, out),
		['я', s @ ..] => super::t5::n942(s, out),
		_ => false,
	}
}

/// Words ending -уеешь.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3722);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3723);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хеешь.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 917);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9642);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чеешь.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['я', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9644);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 3724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 920);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3725);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеешь.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3726);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3727);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['р', 'о', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9646);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 924);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3729);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ижешь.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3758);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3759);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 3760);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3761);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3762);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3763);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3764);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3765);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3766);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3768);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 3769);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16908);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16909);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16910);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16911);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23559);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожешь.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 3770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9664);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 9665);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 943);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3771);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржешь.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 945);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яжешь.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 947);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 948);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3774);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3775);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 949);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3777);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9668);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плешь.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3828);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 9712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3829);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16924);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16925);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 16926);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16927);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23563);
											s.is_empty()
										}
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
					super::push(out, 3830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 978);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 3831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9714);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9715);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9716);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9717);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16929);
									s.is_empty()
								}
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
			super::push(out, 979);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 3832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9718);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9719);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9720);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9721);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9722);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9723);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябнешь.
pub(crate) fn n1046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1019);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 3861);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9743);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9744);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9745);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9746);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3862);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -внешь.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 3863);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3864);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гнешь.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1022);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 3865);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9748);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16944);
									s.is_empty()
								}
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
			super::push(out, 1024);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3866);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9749);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3867);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3868);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9751);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 16945);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23565);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1025);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3870);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 9752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 3871);
					match s {
						[] => true,
						['р', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16946);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16947);
									s.is_empty()
								}
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
			super::push(out, 1026);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3872);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3873);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3874);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3875);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3876);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3877);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3878);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16948);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 16949);
									s.is_empty()
								}
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
			super::push(out, 1028);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 3880);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3881);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 3882);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 3883);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3884);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16950);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 16951);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16952);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23566);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 23567);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 9757);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 9758);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 16953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3885);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3887);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3888);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 9759);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16954);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 16955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 3889);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9761);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16956);
									s.is_empty()
								}
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
			super::push(out, 1031);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3890);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3891);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16957);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16958);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23568);
											s.is_empty()
										}
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
					super::push(out, 3892);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9762);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 9763);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 16960);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23569);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9764);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 9765);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16961);
									s.is_empty()
								}
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
			super::push(out, 1032);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3893);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3894);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3895);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -днешь.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1034);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3896);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3897);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 16962);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 16963);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 16964);
									s.is_empty()
								}
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
			super::push(out, 1036);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3899);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3900);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3901);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 3902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енешь.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1043);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жнешь.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1046);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знешь.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3907);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9769);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1051);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9771);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9772);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'с', 'о', s @ ..] => {
							super::push(out, 9773);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9774);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16966);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9776);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 9777);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 9778);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16968);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16969);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16970);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16971);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23570);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3910);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3911);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							super::push(out, 9780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9781);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 16972);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23571);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 9782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1056);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3912);
					match s {
						[] => true,
						['о', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16973);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16974);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23572);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1057);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9783);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9784);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9786);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инешь.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9788);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9789);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16975);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23573);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1060);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3914);
					match s {
						[] => true,
						['р', 'п', 'ы', s @ ..] => {
							super::push(out, 9790);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16976);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3915);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 3916);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1062);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3917);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3918);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1063);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йнешь.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 3920);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кнешь.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9793);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 16977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 3922);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3923);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3924);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 3925);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3926);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9794);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 16978);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23574);
											s.is_empty()
										}
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
					super::push(out, 3927);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 3928);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3929);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9795);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1068);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 3931);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 3932);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16980);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16981);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16982);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16983);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3933);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9796);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9797);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 16984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9798);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3934);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9800);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 16985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 9801);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 9802);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9803);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 16986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3935);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 9804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3936);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 3937);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 3938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3939);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 9805);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9806);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 9807);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9808);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9809);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3941);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 16989);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16990);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23576);
											s.is_empty()
										}
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
					super::push(out, 3942);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9810);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3943);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 3944);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3945);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1070);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3946);
					match s {
						[] => true,
						['щ', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16992);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23577);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 16993);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23578);
											s.is_empty()
										}
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
					super::push(out, 3947);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 9813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16994);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 9814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16996);
									s.is_empty()
								}
								['а', s @ ..] => super::t5::n17937(s, out),
								['у', s @ ..] => super::t5::n17938(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t5::n10332(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t5::n1099(s, out),
		['о', s @ ..] => super::t5::n1100(s, out),
		['р', s @ ..] => super::t5::n1101(s, out),
		['с', s @ ..] => super::t5::n1102(s, out),
		['т', s @ ..] => super::t5::n1103(s, out),
		['у', s @ ..] => super::t5::n1104(s, out),
		['ч', s @ ..] => super::t5::n1105(s, out),
		['ш', s @ ..] => super::t5::n1106(s, out),
		['ы', s @ ..] => super::t5::n1107(s, out),
		['ь', s @ ..] => super::t5::n1108(s, out),
		['ю', s @ ..] => super::t5::n1109(s, out),
		['я', s @ ..] => super::t5::n1110(s, out),
		_ => false,
	}
}

/// Words ending -мнешь.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1084);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1085);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1086);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1087);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3986);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онешь.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1088);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 3987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 3989);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9862);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9863);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17026);
									s.is_empty()
								}
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
			super::push(out, 1091);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3991);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3992);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3993);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9865);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3994);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 3995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9866);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9867);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

